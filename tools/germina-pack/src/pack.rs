use std::{
    fmt, fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

pub fn pack(path: &Path) -> Result<(), Error> {
    const PACK_EXTENSIONS: [&str; 2] = ["toml", "png"];

    fn visit_dirs<F>(path: &Path, on_file: &mut F) -> Result<(), Error>
    where
        F: FnMut(PathBuf) -> Result<(), Error>,
    {
        for res in fs::read_dir(path)? {
            let entry = res?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, on_file)?;
            } else if path.is_file() {
                on_file(path)?;
            }
        }

        Ok(())
    }

    let path_os = path.as_os_str();
    let path_len = path_os.len() + '/'.len_utf8();
    let mut on_file = |filepath: PathBuf| {
        match filepath.extension() {
            Some(ext) if PACK_EXTENSIONS.iter().any(|&pack| ext == pack) => {}
            _ => return Ok(()),
        }

        let archive_path = match filepath.to_str() {
            Some(filepath) => &filepath[path_len..],
            None => return Err(Error::InvalidFileName(filepath)),
        };

        println!("path: {archive_path}");

        Ok(())
    };
    visit_dirs(path, &mut on_file)?;

    Ok(())
}

pub enum Error {
    NotFound,
    PermissionDenied,
    InvalidFileName(PathBuf),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            err => panic!("err: {err:?}"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::InvalidFileName(path) => write!(f, "invalid file name: {}", path.display()),
        }
    }
}
