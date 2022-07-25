mod model;

use crate::{
    error::{IoError, JsonError},
    load::model::Model,
};
use base::kit::{Asset, Key, Kind, ParseKeyError};
use std::{
    fmt,
    io::{self, Read},
    path::{Path, PathBuf},
};
use zip::{result::ZipError, ZipArchive};

pub struct KitSource {
    pub name: Key,
    pub model: Model,
}

impl KitSource {
    pub fn load(path: &Path) -> Result<Self, Error> {
        use std::{fs::File, mem};

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.rsplit_once('.'))
            .ok_or(Error::UndefinedName)?
            .0
            .parse()?;

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
            let Asset { name, kind } = match Asset::parse_path(filename) {
                Some(asset) => asset,
                None => {
                    let kitname = path.file_name().expect("filename");
                    log::info!("entry {filename} skipped in {kitname:?}");
                    continue;
                }
            };

            match kind {
                Kind::Tile => {
                    content.clear();
                    file.read_to_string(&mut content)?;
                    let tile = json::from_str(&content).map_err(|err| JsonError {
                        err,
                        src: mem::take(&mut content),
                        filename: Some(file.name().into()),
                    })?;
                    model.tiles.insert(name, tile);
                }
            }
        }

        let mut path_buf = String::with_capacity(32);
        let mut tile_sprites = Vec::with_capacity(32);
        for (_, tile) in model.tiles.iter() {
            tile.sprites(|key| tile_sprites.push(key.clone()));
        }

        for sprite_key in tile_sprites {
            model
                .tile_sprites
                .try_insert::<_, Error>(sprite_key, |key| {
                    path_buf.clear();
                    path_buf.push_str("sprites/tiles/");
                    path_buf.push_str(key);
                    path_buf.push_str(".png");

                    let mut file = arch.by_name(&path_buf).map_err(|err| match err {
                        ZipError::FileNotFound => Error::Io(IoError {
                            err: io::ErrorKind::NotFound.into(),
                            path: Some(PathBuf::from(&path_buf)),
                        }),
                        err => err.into(),
                    })?;

                    let mut buf = Vec::with_capacity(128);
                    file.read_to_end(&mut buf)?;
                    Ok(buf)
                })?;
        }

        Ok(Self { name, model })
    }
}

pub enum Error {
    UndefinedName,
    ParseKey(ParseKeyError),
    Io(IoError),
    Json(JsonError),
    Arch(&'static str),
}

impl From<ParseKeyError> for Error {
    fn from(err: ParseKeyError) -> Self {
        Self::ParseKey(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(IoError { err, path: None })
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Self::Json(err)
    }
}

impl From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        match err {
            ZipError::Io(err) => err.into(),
            ZipError::InvalidArchive(arch) => Self::Arch(arch),
            ZipError::UnsupportedArchive(arch) => Self::Arch(arch),
            ZipError::FileNotFound => Self::Arch("file not found"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UndefinedName => write!(f, "kit name is undefined"),
            Self::ParseKey(err) => write!(f, "failed parse a key: {err}"),
            Self::Io(io) => write!(f, "{io}"),
            Self::Json(json) => write!(f, "{json}"),
            Self::Arch(arch) => write!(f, "archive error: {arch}"),
        }
    }
}
