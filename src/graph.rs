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

pub fn build(cfg: &Config) -> Result<BuildGraph, BearError> {
    let mut sources = Vec::new();

    for pattern in &cfg.build.sources {
        let matched = expand_glob(pattern)?;

        if matched.is_empty() {
            return Err(BearError::Graph(format!("glob '{}' matched no files", pattern)));
        }

        for src in matched {
            let obj = src_to_obj(&src);
            sources.push(SourceFile { src, obj });
        }
    }

    let objects = sources.iter()
        .map(|sf| sf.obj.clone())
        .collect();

    Ok(BuildGraph { sources, objects })
}


