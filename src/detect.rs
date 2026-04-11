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


