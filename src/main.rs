use clap::Parser;
use std::process;
use sublinex::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = sublinex::run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
