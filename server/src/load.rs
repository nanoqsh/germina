mod model;

use crate::load::model::Model;
use base::kit::{Asset, Kind};
use std::{
    fmt,
    io::{self, Read},
    path::Path,
};
use zip::{result::ZipError, ZipArchive};

#[derive(Default)]
pub struct Kit {
    pub model: Model,
}

impl Kit {
    pub fn load(path: &Path) -> Result<Self, Error> {
        use std::{fs::File, mem};

        let mut model = Model::default();

        let file = File::open(path)?;
        let mut arch = ZipArchive::new(file)?;
        let mut content = String::with_capacity(128);

        for i in 0..arch.len() {
            let mut file = arch.by_index(i)?;
            if !file.is_file() {
                continue;
            }

            let filename = file.name();
            let asset = match Asset::parse_path(filename) {
                Some(asset) => asset,
                None => {
                    let kitname = path.file_name().expect("filename");
                    log::info!("entry {filename} skipped in {kitname:?}");
                    continue;
                }
            };

            match asset {
                Asset {
                    name,
                    kind: Kind::Tile,
                } => {
                    content.clear();
                    file.read_to_string(&mut content)?;
                    let tile = json::from_str(&content).map_err(|err| Error::Json {
                        err,
                        src: mem::take(&mut content),
                        filename: file.name().into(),
                    })?;
                    model.tiles.insert(name, tile);
                }
            }
        }

        Ok(Self { model })
    }
}

pub enum Error {
    Arch(&'static str),
    Json {
        err: json::Error,
        src: String,
        filename: String,
    },
    NotFound,
    PermissionDenied,
    Other,
}

impl From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        match err {
            ZipError::Io(err) => err.into(),
            ZipError::InvalidArchive(arch) => Self::Arch(arch),
            ZipError::UnsupportedArchive(arch) => Self::Arch(arch),
            ZipError::FileNotFound => Self::NotFound,
        }
    }
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crossterm::style::Stylize;

        match self {
            Self::Arch(arch) => write!(f, "archive error: {arch}"),
            Self::Json {
                err: json::Error::Message { msg, location },
                src,
                filename,
            } => {
                const SHOW_LINES: usize = 3;

                write!(f, "while parsing {}", filename.as_str().bold())?;

                match location {
                    Some(location) => {
                        let n_line = location.line + 1;
                        writeln!(f, " at line {}:", n_line)?;

                        let start = n_line.saturating_sub(SHOW_LINES);
                        for line in src.lines().skip(start).take(n_line.min(SHOW_LINES)) {
                            writeln!(f, "{line}")?;
                        }

                        let column = location.column;
                        writeln!(f, "{:>column$}{}", "", '^'.yellow().bold())?;
                    }
                    None => writeln!(f, ":")?,
                }

                // Trim pest error format
                let msg = msg
                    .rsplit_once('=')
                    .map(|(_, right)| right.trim())
                    .unwrap_or(msg);

                write!(f, "{msg}")
            }
            Self::NotFound => write!(f, "file not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::Other => write!(f, "unknown file handling error"),
        }
    }
}
