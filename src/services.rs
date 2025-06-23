// src/nmap_services.rs
use std::{collections::HashMap, str::FromStr};

/// 端口协议组合（用于 HashMap 键）
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PortKey {
    pub port: u16,
    pub protocol: String,
}

/// 静态存储所有端口服务
static NMAP_SERVICES: once_cell::sync::Lazy<HashMap<PortKey, PortService>> =
    once_cell::sync::Lazy::new(load_nmap_services);

/// 端口服务信息
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
        let _frequency = parts.next().ok_or("Missing frequency")?; // 跳过 frequency

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

/// 从嵌入的 nmap-services 文件加载数据
pub fn load_nmap_services() -> HashMap<PortKey, PortService> {
    include_str!("nmap-services")
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

/// 查询端口服务（静态方法）
pub fn get_service(port: u16, protocol: &str) -> Option<&'static PortService> {
    let key = PortKey {
        port,
        protocol: protocol.to_string(),
    };
    NMAP_SERVICES.get(&key)
}

/// 获取所有端口服务（静态方法）
#[allow(dead_code)]
pub fn all_services() -> &'static HashMap<PortKey, PortService> {
    &NMAP_SERVICES
}
