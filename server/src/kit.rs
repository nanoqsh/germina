mod model;
mod resources;

use crate::kit::{
    model::Model,
    resources::{Key, Resources},
};
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
        use std::fs::File;

        let mut model = Model::default();

        let file = File::open(path)?;
        let mut arch = ZipArchive::new(file).expect("read archive");
        for i in 0..arch.len() {
            let mut file = arch.by_index(i)?;
            if !file.is_file() {
                continue;
            }

            let filename = file.name();
            let asset = match Asset::parse(filename) {
                Some(asset) => asset,
                None => {
                    println!("skipped {}", filename);
                    continue;
                }
            };

            match asset {
                Asset {
                    name,
                    kind: Kind::Tile,
                } => {
                    let mut content = String::with_capacity(64);
                    file.read_to_string(&mut content).expect("read");
                    let tile = toml::from_str(&content).expect("read toml");
                    model.tiles.insert(name, tile);
                }
            }
        }

        Ok(Self { model })
    }
}

struct Asset {
    name: Key,
    kind: Kind,
}

impl Asset {
    fn parse(str: &str) -> Option<Self> {
        let (kind, filename) = str.split_once('/')?;
        let (name, ext) = filename.split_once('.')?;

        let kind = match kind {
            "tiles" => Kind::Tile,
            _ => return None,
        };

        if ext != "toml" {
            return None;
        }

        Some(Self {
            name: Key::from_str(name).ok()?,
            kind,
        })
    }
}

enum Kind {
    Tile,
}

pub enum Error {
    Arch(&'static str),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Arch(arch) => write!(f, "archive error: {}", arch),
            Self::NotFound => write!(f, "file not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::Other => write!(f, "unknown file handling error"),
        }
    }
}
