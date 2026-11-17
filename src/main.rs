mod cli;
mod commands;
mod dns;
mod error;
mod http;
mod output;
mod tls;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
