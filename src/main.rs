use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use procfs::{
    net::TcpState,
    process::{FDTarget, Process},
};
use services::get_service;
use std::{collections::HashMap, io, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

mod services;

#[derive(Debug)]
struct PortInfo {
    port: u16,
    pid: Option<i32>,
    process_name: Option<String>,
    service_name: Option<String>,
}

struct AppState {
    ports: Vec<PortInfo>,
    scroll_position: usize,
}

impl AppState {
    fn new(ports: Vec<PortInfo>) -> Self {
        AppState {
            ports,
            scroll_position: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize state
    let ports = get_listening_ports()?;
    let mut app_state = AppState::new(ports);

    // Main loop
    let mut should_quit = false;
    while !should_quit {
        // Get port information (refresh if needed)
        app_state.ports = get_listening_ports()?;

        // Render UI
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(size);

            // Get the height of the list area
            let list_height = chunks[0].height as usize;

            // Calculate how many items we can display
            let visible_items = if list_height > 2 { list_height - 2 } else { 0 }; // Subtract 2 for borders

            // Adjust scroll position if needed
            if app_state.scroll_position + visible_items > app_state.ports.len() {
                app_state.scroll_position = app_state.ports.len().saturating_sub(visible_items);
            }

            let items: Vec<ListItem> = app_state
                .ports
                .iter()
                .skip(app_state.scroll_position)
                .take(visible_items)
                .map(|port| {
                    let process_name = port
                        .process_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    let service_name = port
                        .service_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    let display_text = format!(
                        "Port: {:5} | PID: {:5?} | Process: {:15} | Service: {}",
                        port.port,
                        port.pid.unwrap_or(0),
                        process_name,
                        service_name
                    );
                    ListItem::new(display_text)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title("Listening Ports (↑/↓ to scroll, q to quit, r to refresh)")
                    .borders(Borders::ALL),
            );
            f.render_widget(list, chunks[0]);

            // Status line showing scroll position
            let status_line = if app_state.ports.is_empty() {
                Span::raw("No ports found")
            } else {
                Span::raw(format!(
                    "Showing {}-{} of {} items (↑/↓ to scroll)",
                    app_state.scroll_position + 1,
                    (app_state.scroll_position + visible_items).min(app_state.ports.len()),
                    app_state.ports.len()
                ))
            };
            let status = Block::default().borders(Borders::TOP).title(status_line);
            f.render_widget(status, chunks[1]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => should_quit = true,
                    KeyCode::Char('r') => (), // Refresh will happen automatically
                    KeyCode::Down => {
                        // Scroll down
                        if app_state.scroll_position + 1 < app_state.ports.len() {
                            app_state.scroll_position += 1;
                        }
                    }
                    KeyCode::Up => {
                        // Scroll up
                        app_state.scroll_position = app_state.scroll_position.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        // Page down
                        let list_height = terminal.size()?.height as usize;
                        let visible_items = if list_height > 2 { list_height - 2 } else { 0 };
                        app_state.scroll_position = (app_state.scroll_position + visible_items)
                            .min(app_state.ports.len().saturating_sub(visible_items));
                    }
                    KeyCode::PageUp => {
                        // Page up
                        let list_height = terminal.size()?.height as usize;
                        let visible_items = if list_height > 2 { list_height - 2 } else { 0 };
                        app_state.scroll_position =
                            app_state.scroll_position.saturating_sub(visible_items);
                    }
                    KeyCode::Home => {
                        // Jump to start
                        app_state.scroll_position = 0;
                    }
                    KeyCode::End => {
                        // Jump to end
                        let list_height = terminal.size()?.height as usize;
                        let visible_items = if list_height > 2 { list_height - 2 } else { 0 };
                        app_state.scroll_position =
                            app_state.ports.len().saturating_sub(visible_items);
                    }
                    _ => {}
                }
            }
        }
    }

    // Clean up terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// Other functions (get_listening_ports, create_inode_map) remain the same

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
