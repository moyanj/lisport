use std::collections::HashMap;

use procfs::{
    net::{TcpNetEntry, TcpState},
    process::{FDTarget, Process},
};

use crate::services;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub inode: u64,
    pub is_ipv6: bool,
    pub host: String,
    pub pid: Option<i32>,
    pub process: Option<String>,      // Process name
    pub full_command: Option<String>, // Full command line
    pub cwd: Option<String>,          // Process working directory
    pub service: Option<String>,
    pub is_privileged: bool,
    pub user: Option<String>,
}

/// Retrieves information about listening ports
pub fn get_listening_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    let mut ports = Vec::new();
    let all_procs = procfs::process::all_processes()?;
    let inode_map = create_inode_map(&all_procs.filter_map(Result::ok).collect::<Vec<_>>())?; // Create a map of inode to PID
    let mut seen_entries = HashMap::new();

    for tcp in tcp_entrys()? {
        let proc = match inode_map.get(&tcp.inode) {
            Some(pid) => Process::new(*pid)?,
            None => continue,
        };

        let entry_key = (tcp.local_address.port(), proc.pid);
        if seen_entries.contains_key(&entry_key) {
            continue; // Skip duplicate entry for the same port and PID
        }
        seen_entries.insert(entry_key, true);

        ports.push(PortInfo {
            port: tcp.local_address.port(),
            inode: tcp.inode,
            is_ipv6: tcp.local_address.is_ipv6(),
            host: tcp.local_address.ip().to_string(),

            pid: Some(proc.pid),
            process: Some(proc.stat()?.comm),
            full_command: Some(proc.cmdline()?.join(" ")),
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

/// Retrieves TCP entries for both IPv4 and IPv6 protocols
fn tcp_entrys() -> Result<impl Iterator<Item = TcpNetEntry>, Box<dyn std::error::Error>> {
    let mut v = Vec::new();
    v.extend(procfs::net::tcp()?);
    v.extend(procfs::net::tcp6()?);
    Ok(v.into_iter().filter(|tcp| tcp.state == TcpState::Listen))
}

/// Creates a map of socket inode numbers to corresponding process PIDs
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
