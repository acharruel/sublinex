use std::{env, process};
use sublinex::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = sublinex::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
