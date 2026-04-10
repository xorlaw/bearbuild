use std::fmt;

#[derive(Debug)]
pub enum BearError {
    Config(String),
    Detect(String),
    Graph(String),
    Emit(String),
    Io(std::io::Error),
}

impl fmt::Display for BearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BearError::Config(msg)  => write!(f, "config error: {msg}"),
            BearError::Detect(msg)  => write!(f, "detect error: {msg}"),
            BearError::Graph(msg)   => write!(f, "graph error: {msg}"),
            BearError::Emit(msg)    => write!(f, "emit error: {msg}"),
            BearError::Io(e)        => write!(f, "io error: {e}"),
        }
    }
}

impl From<std::io::Error> for BearError {
    fn from(e: std::io::Error) -> Self {
        BearError::Io(e)
    }
}

