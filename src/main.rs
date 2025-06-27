use clap::Parser;
mod core;
mod output;
mod services;
mod ui;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum)]
    format: Option<output::OutputFormat>,
    #[arg(short, long, default_value_t = String::from("/dev/stdout"))]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ports = core::get_listening_ports()?;

    let text = cli
        .format
        .map_or_else(ui::ui_main, |fmt| Ok(output::output_ports(fmt, ports)))?;

    std::fs::write(cli.output, text)?;
    Ok(())
}
