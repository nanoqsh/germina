use crate::error::JsonError;
use serde::Deserialize;
use std::{fmt, io, path::Path};

#[derive(Deserialize)]
pub struct Config {
    pub net: Net,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Error> {
        use std::fs;

        let content = fs::read_to_string(path)?;
        let config = json::from_str(&content).map_err(|err| JsonError {
            err,
            src: content,
            filename: None,
        })?;

        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct Net {
    ip: String,
    port: u16,
}

impl Net {
    pub fn addr(&self) -> (&str, u16) {
        (self.ip.as_str(), self.port)
    }
}

pub enum Error {
    Json(JsonError),
    NotFound,
    PermissionDenied,
    Other,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        use io::ErrorKind;

        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::Other,
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Self::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Json(json) => write!(f, "{json}"),
            Self::NotFound => write!(f, "file not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::Other => write!(f, "unknown file handling error"),
        }
    }
}
