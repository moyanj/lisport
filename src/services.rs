use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PortKey {
    pub port: u16,
    pub protocol: String,
}

/// All services data
static NMAP_SERVICES: once_cell::sync::Lazy<HashMap<PortKey, PortService>> =
    once_cell::sync::Lazy::new(load_nmap_services);

/// Port service information
#[derive(Debug, PartialEq, Clone)]
pub struct PortService {
    pub name: String,
    pub port: u16,
    pub protocol: String,
    pub comment: Option<String>,
}

impl FromStr for PortService {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split_whitespace();

        let name = parts.next().ok_or("Missing name")?.to_string();
        let port_proto = parts.next().ok_or("Missing port/proto")?;

        let (port, protocol) = port_proto
            .split_once('/')
            .ok_or("Invalid port/proto format")?;
        let port = port.parse().map_err(|_| "Invalid port")?;

        let comment = parts.collect::<Vec<_>>().join(" ");
        let comment = if !comment.is_empty() {
            Some(comment)
        } else {
            None
        };

        Ok(PortService {
            name,
            port,
            protocol: protocol.to_string(),
            comment,
        })
    }
}

/// Loads service data from the embedded services file
pub fn load_nmap_services() -> HashMap<PortKey, PortService> {
    include_str!("services.list")
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.parse::<PortService>().ok())
        .map(|service| {
            let key = PortKey {
                port: service.port,
                protocol: service.protocol.clone(),
            };
            (key, service)
        })
        .collect()
}

/// Queries port service information (static method)
pub fn get_service(port: u16, protocol: &str) -> Option<&'static PortService> {
    let key = PortKey {
        port,
        protocol: protocol.to_string(),
    };
    NMAP_SERVICES.get(&key)
}

/// Gets all port services (static method)
#[allow(dead_code)]
pub fn all_services() -> &'static HashMap<PortKey, PortService> {
    &NMAP_SERVICES
}
