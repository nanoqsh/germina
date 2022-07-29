use std::{
    fmt, fs,
    io::{self, ErrorKind},
    path::Path,
};

pub fn info(path: &Path) -> Result<Info, Error> {
    let _todo = fs::read_to_string(path)?;
    Ok(Info {})
}

pub struct Info {}

impl fmt::Display for Info {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

pub enum Error {
    NotFound,
    PermissionDenied,
    Other,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::Other,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::Other => write!(f, "unknown file handling error"),
        }
    }
}
