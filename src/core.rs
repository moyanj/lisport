use std::collections::HashMap;

use procfs::{
    net::{TcpNetEntry, TcpState},
    process::{FDTarget, Process},
};

use crate::services;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PortInfo {
    pub port: u16, // 端口号
    pub inode: u64,
    pub is_ipv6: bool, // 是否为IPv6
    pub host: String,  // 主机名

    pub pid: Option<i32>,          // 进程 ID
    pub process: Option<String>,   // 进程名称
    pub full_path: Option<String>, // 进程完整路径
    pub cwd: Option<String>,       // 当前工作目录
    pub service: Option<String>,   // 服务名称

    pub is_privileged: bool,  // 是否有特权
    pub user: Option<String>, // 用户
}

pub fn get_listening_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    let mut ports = Vec::new();
    let all_procs = procfs::process::all_processes()?;
    let inode_map = create_inode_map(&all_procs.filter_map(Result::ok).collect::<Vec<_>>())?;

    for tcp in tcp_entrys()? {
        let proc = match inode_map.get(&tcp.inode) {
            Some(pid) => Process::new(*pid)?,
            None => continue,
        };
        ports.push(PortInfo {
            port: tcp.local_address.port(),
            inode: tcp.inode,
            is_ipv6: tcp.local_address.is_ipv6(),
            host: tcp.local_address.ip().to_string(),

            pid: Some(proc.pid),
            process: Some(proc.stat()?.comm),
            full_path: Some(proc.cmdline()?.join(" ")),
            cwd: Some(
                proc.cwd()?
                    .as_os_str()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
            ),
            service: services::get_service(tcp.local_address.port(), "tcp").map(|s| s.name.clone()),

            is_privileged: if tcp.local_address.port() < 1024 {
                true
            } else {
                false
            },
            user: proc.uid().ok().and_then(|uid| {
                users::get_user_by_uid(uid).map(|u| u.name().to_os_string().into_string().unwrap())
            }),
        });
    }
    Ok(ports)
}

fn tcp_entrys() -> Result<impl Iterator<Item = TcpNetEntry>, Box<dyn std::error::Error>> {
    let mut v = Vec::new();
    v.extend(procfs::net::tcp()?);
    v.extend(procfs::net::tcp6()?);
    Ok(v.into_iter().filter(|tcp| tcp.state == TcpState::Listen))
}

fn create_inode_map(
    all_procs: &[Process],
) -> Result<HashMap<u64, i32>, Box<dyn std::error::Error>> {
    let mut inode_to_pid = HashMap::new();

    for proc in all_procs {
        if let Ok(fds) = proc.fd() {
            for fd in fds {
                if let Ok(fd) = fd {
                    if let FDTarget::Socket(socket_inode) = fd.target {
                        inode_to_pid.insert(socket_inode, proc.pid);
                    }
                }
            }
        }
    }

    Ok(inode_to_pid)
}
