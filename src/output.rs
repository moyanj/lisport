use crate::core::PortInfo;
use clap::ValueEnum;
use serde_json::to_string_pretty;
use std::fmt::Write;
use std::vec::Vec;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Text,
    Json,
    Md,
}

/// 输出端口信息，统一返回 String 格式
pub fn output_ports(format: OutputFormat, ports: Vec<PortInfo>) -> String {
    match format {
        OutputFormat::Text => output_text(&ports),
        OutputFormat::Json => output_json(&ports),
        OutputFormat::Md => output_markdown(&ports),
    }
}

/// 返回 tab 分隔的文本格式输出
fn output_text(ports: &[PortInfo]) -> String {
    let mut output = String::new();
    output.push_str("PORT\tPID\tPROCESS\t\tSERVICE\n");

    for port_info in ports {
        let process = port_info.process.as_deref().unwrap_or("unknown");
        let service = port_info.service.as_deref().unwrap_or("unknown");
        let pid = port_info.pid.unwrap_or(0);

        writeln!(
            &mut output,
            "{}\t{}\t{}\t\t{}",
            port_info.port, pid, process, service
        )
        .unwrap();
    }

    output
}

/// 返回 JSON 格式字符串输出
fn output_json(ports: &[PortInfo]) -> String {
    match to_string_pretty(ports) {
        Ok(json) => json,
        Err(e) => format!("{{ \"error\": \"Failed to serialize JSON: {}\" }}", e),
    }
}

/// 返回 Markdown 表格格式字符串输出
fn output_markdown(ports: &[PortInfo]) -> String {
    let mut output = String::new();

    output.push_str("| PORT | PID | PROCESS | SERVICE |\n");
    output.push_str("|------|-----|---------|---------|\n");

    for port_info in ports {
        let process = port_info.process.as_deref().unwrap_or("unknown");
        let service = port_info.service.as_deref().unwrap_or("unknown");
        let pid = port_info.pid.unwrap_or(0);

        writeln!(
            &mut output,
            "| {} | {} | `{}` | `{}` |",
            port_info.port, pid, process, service
        )
        .unwrap();
    }

    output
}
