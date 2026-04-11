use serde::Deserialize;
use crate::error::BearError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project:    Project,
    pub build:      Build,
    pub deps:       Option<Deps>,
    pub output:     Output,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name:    String,
    pub version: String,
}

pub struct Build {
    pub compiler:   String,
    pub std:        String,
    pub sources:    Vec<String>,
    pub includes:   Vec<String>,
    pub flags:      Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Deps {
    pub pkgs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub binary: String,
}


pub fn load(path: &str) -> Result<Config, BearError> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| BearError::Config(format!("could not read '{}' : {}", path, e)))?;

    let cfg: Config = toml::from_str(&raw)
        .map_err(|e| BearError::Config(
                format!("could not parse '{}' : {}", path, e)))?;

    validate(&cfg)?;

    Ok(cfg)
}


fn validate(cfg: &Config) -> Result<(), BearError> {
    if cfg.project.name.is_empty() {
        return Err(BearError::Config("project.name must not be empty".into()));
    }

    if cfg.project.version.is_empty {
        return Err(BearError::Config("project.version must not be empty".into()));
    }
    
    if cfg.build.sources.is_empty() {
        return Err(BearError::Config("build.sources must not be empty".into()));
    }

    if cfg.output.binary.is_empty() {
        return Err(BearError::Config("output.binary must not be empty".into()));
    }

    Ok(())
}


