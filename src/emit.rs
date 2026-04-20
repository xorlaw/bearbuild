use std::fmt::Write as FmtWrite;
use std::fs;
use crate::config::Config;
use crate::detect::Env;
use crate::graph::BuildGraph;
use crate::error::BearError;

pub fn write(cfg: &Config, env: &Env, graph: &BuildGraph) -> Result<(), BearError> {
    fs::create_dir_all("build").map_err(|e| BearError::Emit(format!("could not create build/ directory: {}", e)))?;

    let mut out = String::new();

    write_header(cfg, env, &mut out);
    write_rules(&mut out);
    write_compile_edges(cfg, env, graph, &mut out)?;
    write_link_edge(cfg, env, graph, &mut out);

    fs::write("build.ninja", &out).map_err(|e| BearError::Emit(format!("could not write build.ninja: {}", e)))?;

    Ok(())
}


