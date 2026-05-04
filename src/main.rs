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

fn print_help() {
    println!(
        "bearbuild v{ver} - a modern and simple build system

        USAGE:
            bearbuild [OPTIONS]

        OPTIONS:
            -h, --help      display this help message
            -v, --version   print the current version

        BEHAVIOUR:
            Reads bear.toml in the current directory, probes the environment and writes a build.ninja file to be run with ninja or ninja - compatible tools such as samurai.

        An example bear.toml is available on the official Github repository.",
        ver = env!("CARGO_PKG_VERSION")
    );
}

fn run() -> Result<(), BearError> {
    println!("bearbuild: loading bear.toml...");
    let cfg = config::load("bear.toml")?;

    println!("bearbuild: probing environment...");
    let env = detect::probe(&cfg)?;
    println!("bearbuild: compiler → {}", env.compiler_ver);

    println!("bearbuild: resolving sources...");
    let build_graph = graph::build(&cfg)?;
    println!("bearbuild: {} source file(s) found", build_graph.sources.len());

    println!("bearbuild: writing build.ninja...");
    emit::write(&cfg, &env, &build_graph)?;

    println!(
        "bearbuild: done. run 'ninja' to build '{}'.",
        cfg.output.binary
    );
    Ok(())
}


