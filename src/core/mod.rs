mod local;
#[cfg(feature = "remote")]
mod remote;

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
pub fn get_listening_ports(method: &String) -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    if method == "local" {
        local::get_listening_ports()
    } else if method == "remote" {
        #[cfg(feature = "remote")]
        return remote::get_listening_ports();
        #[cfg(not(feature = "remote"))]
        return Err("Remote method is not enabled".into());
    } else {
        Err("Invalid method specified".into())
    }
}
