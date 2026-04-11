use std::process::Command;
use crate::config::Config;
use crate::error::BearError;

#[derive(Debug)]
pub struct Env {
    pub compiler:       String,
    pub compiler_ver:   String,
    pub pkg_cflags:     Vec<String>,
    pub pkg_libs:       Vec<String>,
}

pub fn probe(cfg: &Config) -> Result<Env, BearError> {
    let compiler     = find_compiler(&cfg.build.compiler)?;
    let compiler_ver = compiler_version(&compiler)?;

    let (pkg_cflags, pkg_libs) = match &cfg.deps {
        Some(deps) if !deps.pkgs.is_empty() => pkg_config(&deps.pkgs)?,
        _ => (vec![], vec![]),
    };

    Ok(Env {
        compiler,
        compiler_ver,
        pkg_cflags,
        pkg_libs,
    })
}
