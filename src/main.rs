use clap::{Parser, ValueEnum};
use procfs::{
    net::TcpState,
    process::{FDTarget, Process},
};
use services::get_service;
use std::collections::HashMap;

mod services;
mod ui;

#[derive(Debug)]
struct PortInfo {
    port: u16,
    pid: Option<i32>,
    process_name: Option<String>,
    service_name: Option<String>,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum)]
    output: Option<OutputType>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputType {
    Json,
    Text,
    MD,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.output {
        Some(OutputType::Json) => Ok(()),
        Some(OutputType::Text) => Ok(()),
        Some(OutputType::MD) => Ok(()),
        None => ui::ui_main(),
    }
}

fn get_listening_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    let tcps = procfs::net::tcp()?;
    let tcps6 = procfs::net::tcp6()?;
    let all_procs = procfs::process::all_processes()?;

    // Pre-cache inode to PID mapping for performance
    let inode_to_pid = create_inode_map(&all_procs.filter_map(Result::ok).collect::<Vec<_>>())?;

    let mut ports = Vec::new();
    for tcp in tcps.iter().chain(tcps6.iter()) {
        if tcp.state == TcpState::Listen {
            let inode = tcp.inode;
            let port = tcp.local_address.port();
            let pid = inode_to_pid.get(&inode).copied();

            let process_name = if let Some(pid) = pid {
                Process::new(pid)
                    .ok()
                    .and_then(|p| p.stat().ok())
                    .map(|stat| stat.comm)
            } else {
                None
            };

            let service_name = get_service(port, "tcp").map(|s| s.name.clone());

            ports.push(PortInfo {
                port,
                pid,
                process_name,
                service_name,
            });
        }
    }

    Ok(ports)
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
