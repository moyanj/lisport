use procfs::{net, process, process::Process};
mod services;
use services::get_service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcps = net::tcp()?;
    let tcps6 = net::tcp6()?;

    for tcp in tcps.iter().chain(tcps6.iter()) {
        if tcp.state == net::TcpState::Listen {
            let inode = tcp.inode;
            if let Some(pid) = find_pid_by_inode(inode) {
                let proc = Process::new(pid)?;
                let port = tcp.local_address.port();
                if let Some(service) = get_service(port, "tcp") {
                    println!(
                        "{:5} -> PID {}: {} ({})",
                        port,
                        pid,
                        proc.stat()?.comm,
                        service.name
                    );
                } else {
                    println!(
                        "{:5} -> PID {}: {} (unknown service)",
                        port,
                        pid,
                        proc.stat()?.comm
                    );
                }
            } else {
                println!("{:5} -> (unknown PID)", tcp.local_address.port());
            }
        }
    }

    Ok(())
}

fn find_pid_by_inode(inode: u64) -> Option<i32> {
    // 获取所有进程
    if let Ok(all_procs) = procfs::process::all_processes() {
        for proc_result in all_procs {
            // 解包 Process
            if let Ok(proc) = proc_result {
                // 读取进程的 fd 目录
                if let Ok(fds) = proc.fd() {
                    for fd in fds {
                        if let Ok(fd) = fd {
                            // 检查是否为目标 inode 的 socket
                            if let process::FDTarget::Socket(socket) = fd.target {
                                if socket == inode {
                                    return Some(proc.pid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
