use clap::{Parser, Subcommand};

use crate::{
    commands::{dns, inspect, tls},
    error::AppError,
    output,
};

#[derive(Debug, Parser)]
#[command(
    name = "site-scout",
    version,
    about = "Inspect websites and network services"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Fetch a page and report its response details.
    Inspect {
        /// HTTP or HTTPS URL to inspect.
        url: String,
        /// Print the result as formatted JSON.
        #[arg(long)]
        json: bool,
    },
    /// Resolve a hostname to IP addresses.
    Dns {
        /// Hostname or IP address to resolve.
        host: String,
        /// Print the result as formatted JSON.
        #[arg(long)]
        json: bool,
    },
    /// Connect to a TLS service and inspect its certificate.
    Tls {
        /// Hostname of the TLS service.
        host: String,
        /// Port of the TLS service.
        #[arg(long, default_value_t = 443)]
        port: u16,
        /// Print the result as formatted JSON.
        #[arg(long)]
        json: bool,
    },
}

pub fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Inspect { url, json } => output::emit(&inspect::run(&url)?, json),
        Command::Dns { host, json } => output::emit(&dns::run(&host)?, json),
        Command::Tls { host, port, json } => output::emit(&tls::run(&host, port)?, json),
    }
}
