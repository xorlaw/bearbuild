use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::error:::BearError;

#[derive(Debug)]
pub struct SourceFile {
    pub src:    PathBuf,
    pub obj:    PathBuf,
}

#[derive(Debug)]
pub struct BuildGraph {
    pub sources: Vec<SourceFile>,
    pub objects: Vec<PathBuf>,
}
