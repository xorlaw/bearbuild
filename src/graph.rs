use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::error::BearError;

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

fn expand_glob(pattern: &str) -> Result<Vec<PathBuf>, BearError> {
    let path = Path::new(pattern);

    let dir = path.parent().ok_or_else(|| BearError::Graph(format!("glob '{}' has no parent directory", pattern)))?;

    let file_pattern = path.file_name().ok_or_else(|| BearError::Graph(format!("glob '{}' has no filename part", pattern)))?
        .to_string_lossy();

    let (prefix, suffix) = if let Some(star) = file_pattern.find('*') {
        (&file_pattern[..star], &file_pattern[star + 1..])
    } else {
        let p = PathBuf::from(pattern);
        if p.exists() {
            return Ok(vec![p]);
        } else {
            return Err(BearError::Graph(format!("source file '{}' does not exist", pattern)));
        }
    };

    let mut matches = Vec::new();

    let entries = std::fs::read_dir(dir).map_err(|e| BearError::Graph(format!("could not read directory '{}': {}", dir.display(), e)))?;

    for entry in entries {
        let entry = entry.map_err(|e|| BearError::Graph(format!("error reading directory entry: {}", e)))?;

        let fname = entry.file_name();
        let fname = fname.to_string_lossy();

        if fname.starts_with(prefix.as_ref())
            && fname.ends_with(suffix.as_ref())
            && fname.len() > prefix.len() + suffix.len()
        {
            matches.push(entry.path());
        }
    }

    matches.sort();

    Ok(matches)
}

fn src_to_obj(src: &Path) -> PathBuf {
    let flat: String = src.components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("_")
        .trim_end_matches(".c")
        .to_string();

    PathBuf::from(format!("build/{}..o", flat))
}


