use crate::core::{PortInfo, get_listening_ports};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

struct AppState {
    ports: Vec<PortInfo>,
    scroll_position: usize,
    horizontal_scroll: usize,
}

impl AppState {
    fn new(ports: Vec<PortInfo>) -> Self {
        AppState {
            ports,
            scroll_position: 0,
            horizontal_scroll: 0,
        }
    }
}

pub fn ui_main(cli: crate::Cli) -> Result<String, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let scan_method = cli.method;

    // Initialize state
    let ports = get_listening_ports(&scan_method)?;
    let mut app_state = AppState::new(ports);

    // Main loop
    let mut should_quit = false;
    while !should_quit {
        // Render UI
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(size);

            let list_height = chunks[0].height as usize;
            let visible_items = if list_height > 2 { list_height - 2 } else { 0 };
            if app_state.scroll_position + visible_items > app_state.ports.len() {
                app_state.scroll_position = app_state.ports.len().saturating_sub(visible_items);
            }

            let items: Vec<ListItem> = app_state
                .ports
                .iter()
                .skip(app_state.scroll_position)
                .take(visible_items)
                .map(|port| {
                    let full_command_name = port
                        .full_command
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    let process_name = port
                        .process
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    let service_name = port
                        .service
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    let is_ipv6 = port.is_ipv6;
                    let user_name = port.user.clone().unwrap_or_else(|| "unknown".to_string());

                    // Set style for each item
                    let display_text = vec![
                        Span::styled(
                            format!("Port: {:5} ", port.port),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::styled(
                            format!("| PID: {:7?} ", port.pid.unwrap_or(-1)),
                            Style::default().fg(Color::Green),
                        ),
                        Span::styled(
                            format!("| USER: {:7} ", user_name),
                            Style::default().fg(Color::Red),
                        ),
                        Span::styled(
                            format!("| IS_IPV6: {:5} ", is_ipv6),
                            Style::default().fg(Color::Blue),
                        ),
                        Span::styled(
                            format!("| Process: {:15} ", process_name),
                            Style::default().fg(Color::Blue),
                        ),
                        Span::styled(
                            format!("| Service: {:15} ", service_name),
                            Style::default().fg(Color::Magenta),
                        ),
                        Span::styled(
                            format!("| FULL CMD: {} ", full_command_name),
                            Style::default().fg(Color::Blue),
                        ),
                    ];

                    let display_text = if app_state.horizontal_scroll > 0 {
                        // Apply horizontal scroll by truncating the start of the text
                        let mut total_length = 0;
                        let truncated_spans: Vec<_> = display_text
                            .iter()
                            .skip_while(|span| {
                                let keep = total_length < app_state.horizontal_scroll;
                                total_length += span.content.len();
                                keep
                            })
                            .cloned()
                            .collect();
                        truncated_spans
                    } else {
                        display_text
                    };

                    ListItem::new(Spans::from(display_text))
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title("Listening Ports (↑/↓/←/→  to scroll, q to quit, r to refresh)")
                    .borders(Borders::ALL),
            );
            f.render_widget(list, chunks[0]);

            // Status line style
            let status_line = if app_state.ports.is_empty() {
                Span::styled("No ports found", Style::default().fg(Color::Red))
            } else {
                Span::styled(
                    format!(
                        "Showing {}-{} of {} items",
                        app_state.scroll_position + 1,
                        (app_state.scroll_position + visible_items).min(app_state.ports.len()),
                        app_state.ports.len()
                    ),
                    Style::default().bg(Color::Black).fg(Color::White),
                )
            };
            let status = Block::default().title(status_line);
            f.render_widget(status, chunks[1]);
        })?;
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => should_quit = true,
                    KeyCode::Char('r') => {
                        // Refresh port information
                        app_state.ports = get_listening_ports(&scan_method)?;
                        // Adjust scroll position if needed
                        if app_state.scroll_position >= app_state.ports.len() {
                            app_state.scroll_position = app_state.ports.len().saturating_sub(1);
                        }
                    }
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
                    KeyCode::Left => {
                        // Scroll left
                        app_state.horizontal_scroll = app_state.horizontal_scroll.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        // Scroll right
                        app_state.horizontal_scroll += 1;
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
    Ok("".to_string())
}
