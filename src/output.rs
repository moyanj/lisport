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

/// Outputs port information in the specified format
pub fn output_ports(format: OutputFormat, ports: Vec<PortInfo>) -> String {
    match format {
        OutputFormat::Text => output_text(&ports),
        OutputFormat::Json => output_json(&ports),
        OutputFormat::Md => output_markdown(&ports),
    }
}

/// Returns tab-separated text format output
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

/// Returns JSON format string output
fn output_json(ports: &[PortInfo]) -> String {
    match to_string_pretty(ports) {
        Ok(json) => json,
        Err(e) => format!("{{ \"error\": \"Failed to serialize JSON: {}\" }}", e),
    }
}

/// Returns Markdown table format string output
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
