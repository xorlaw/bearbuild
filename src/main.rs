// entry point

mod config;
mod detect;
mod emit;
mod error;
mod graph;

use error::BearError;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                println!("bearbuild {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            unknown => {
                eprintln!("bearbuild: unknown argument '{}' - try --help", unknown);
            }
        }
    }

    if let Err(e) = run() {
        eprintln!("bearbuild: error: {}", e);
        process::exit(1);
    }
}


