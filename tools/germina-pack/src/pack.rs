use std::{
    env, fmt,
    fs::{self, File},
    io::{self, BufWriter, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};
use zip::result::ZipError;

pub struct Options<'a> {
    pub name: Option<&'a str>,
    pub rewrite: bool,
}

pub fn pack(path: &Path, options: Options) -> Result<PathBuf, Error> {
    let create_arch = || -> Result<_, Error> {
        let name = options
            .name
            .or_else(|| path.file_name().and_then(|name| name.to_str()))
            .ok_or(Error::KitNameNotSet)?;

        let mut path = env::current_dir()?;
        path.push(name);
        path.set_extension("kit");

        if !options.rewrite && path.exists() {
            return Err(Error::AlreadyExists(path));
        }

        let arch = File::create(&path)?;
        let arch = zip::ZipWriter::new(BufWriter::new(arch));
        Ok((path, arch))
    };

    let mut arch = None;
    let options = zip::write::FileOptions::default();
    let mut buf = Vec::with_capacity(64);
    list_files(path, |entry| {
        let mut file = File::open(entry.fs_path)?;
        buf.clear();
        file.read_to_end(&mut buf)?;

        let (_, arch) = match &mut arch {
            Some(arch) => arch,
            None => arch.insert(create_arch()?),
        };

        arch.start_file(entry.arch_path, options)?;
        arch.write_all(&buf)?;
        Ok(())
    })?;

    if let Some((_, arch)) = &mut arch {
        arch.finish()?;
    }

    arch.map(|(path, _)| path).ok_or(Error::NothingToWrite)
}

fn list_files<F>(path: &Path, mut on_entry: F) -> Result<(), Error>
where
    F: FnMut(Entry) -> Result<(), Error>,
{
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
    let mut on_file = |fs_path: PathBuf| {
        match fs_path.extension() {
            Some(ext) if PACK_EXTENSIONS.iter().any(|&pack| ext == pack) => {}
            _ => return Ok(()),
        }

        let arch_path = match fs_path.to_str() {
            Some(path) => &path[path_len..],
            None => return Err(Error::InvalidFileName(fs_path)),
        };

        on_entry(Entry {
            fs_path: &fs_path,
            arch_path,
        })?;

        Ok(())
    };
    visit_dirs(path, &mut on_file)?;

    Ok(())
}

struct Entry<'a> {
    fs_path: &'a Path,
    arch_path: &'a str,
}

pub enum Error {
    NothingToWrite,
    KitNameNotSet,
    AlreadyExists(PathBuf),
    InvalidFileName(PathBuf),
    Arch(&'static str),
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
            Self::NothingToWrite => write!(f, "nothing to write"),
            Self::KitNameNotSet => write!(f, "a kit name not set"),
            Self::AlreadyExists(path) => write!(f, "already exists: {}", path.display()),
            Self::InvalidFileName(path) => write!(f, "invalid file name: {}", path.display()),
            Self::Arch(arch) => write!(f, "archive error: {}", arch),
            Self::Other => write!(f, "unknown file handling error"),
            Self::NotFound => write!(f, "file not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
        }
    }
}
