mod config;
mod detect;
mod emit;
mod error;
mod graph;

use error::BearError;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("bb: error: {e}");
        process::exit(1)
    }
}

fn run() -> Result<(), BearError> {
    let cfg = config::load("bear.toml")?;
    println!("bb: loaded project '{}'", cfg.project.name);

    let env = detect::probe(&cfg)?;
    println!("bb: compiler -> {}", env.compiler);

    let graph = graph::build(&cfg)?;
    println!("bb: found {} source file(s)", graph.sources.len());

    emit::write(&cfg, &env, &graph)?;
    println!("bb: wrote build.ninja");

    Ok(())
}


