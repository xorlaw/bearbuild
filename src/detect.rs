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

fn find_compiler(name: &str) -> Result<String, BearError> {
    let result = Command::new(name)
        .arg("--version")
        .output();

    match result {
        Ok(out) if out.status.success() => Ok(name.to_string()),
        Ok(_) => Err(BearError::Detect(format!("compiler '{} returned an error on --version", name))),
        Err(_) => Err(BearError::Detect(format!("compiler '{}' not found - is it installed and in PATH?", name))),
    }
}
