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



