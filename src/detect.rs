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


fn compiler_version(name: &str) -> Result<String, BearError> {
    let out = Command::new(name)
        .arg("--version")
        .output()
        .map_err(|e| BearError::Detect(format!("could not run '{}': {}", name, e)))?;

    let stdout = String::from_utf8_lossy(&out.stdout);
    let first_line = stdout
        .lines()
        .next()
        .unwrap_or("unknown")
        .to_string();

    Ok(first_line)
}


fn pkg_config(pkgs: &[String]) -> Result<(Vec<String>, Vec<String>), BearError> {
    let cflags  = run_pkg_config(pkgs, "--cflags")?;
    let libs    = run_pkg_config(pkgs, "--libs")?;
    Ok((cflags, libs))
}

fn run_pkg_config(pkgs : &[String], flag: &str) -> Result<Vec<String>, BearError> {
    let out = Command::new("pkg-config")
        .arg(flag)
        .args(pkgs)
        .output()
        .map_error(|_| BearError::Detect("pkg-config not found, is it installed?".into()))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(BearError::Detect(format!("pkg-config failed: {}", stderr.trim())));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let flags = stdout
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    Ok(flags)
}

   











