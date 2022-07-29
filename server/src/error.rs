use {
    crate::{config, load},
    std::{fmt, io, path::PathBuf},
};

pub enum Error {
    Config { err: config::Error, path: PathBuf },
    Load { err: load::Error, path: PathBuf },
}

impl Error {
    pub fn exit(self) -> ! {
        use crossterm::style::{StyledContent, Stylize};

        eprint!("{} ", "error:".red().bold());
        match self {
            Self::Config { err, path } => {
                eprintln!(
                    "in file {}",
                    StyledContent::new(Default::default(), path.display()).bold()
                );
                eprint!("{err}");
            }
            Self::Load { err, path } => {
                eprintln!(
                    "in file {}",
                    StyledContent::new(Default::default(), path.display()).bold()
                );
                eprint!("{err}");
            }
        }

        std::process::exit(1)
    }
}

pub struct IoError {
    pub err: io::Error,
    pub path: Option<PathBuf>,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use {
            crossterm::style::{StyledContent, Stylize},
            io::ErrorKind,
        };

        let path = self
            .path
            .as_ref()
            .map(|path| StyledContent::new(Default::default(), path.display()).bold());

        match self.err.kind() {
            ErrorKind::NotFound => match path {
                Some(path) => write!(f, "file {path} not found"),
                None => write!(f, "file not found"),
            },
            ErrorKind::PermissionDenied => match path {
                Some(path) => write!(f, "permission of {path} denied"),
                None => write!(f, "permission denied"),
            },
            err => match path {
                Some(path) => write!(f, "io error {err} in file {path}"),
                None => write!(f, "io error {err}"),
            },
        }
    }
}

pub struct JsonError {
    pub err: json::Error,
    pub src: String,
    pub filename: Option<String>,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crossterm::style::Stylize;

        const SHOW_LINES: usize = 3;

        let Self {
            err: json::Error::Message { msg, location },
            src,
            filename,
        } = self;

        write!(f, "while parsing")?;
        if let Some(filename) = filename {
            write!(f, " {}", filename.as_str().bold())?;
        }

        match location {
            Some(location) if !src.trim().is_empty() => {
                let n_line = location.line;
                writeln!(f, " at line {n_line}:")?;

                let start = n_line.saturating_sub(SHOW_LINES);
                for line in src.lines().skip(start).take(n_line.min(SHOW_LINES)) {
                    writeln!(f, "{line}")?;
                }

                let column = location.column;
                writeln!(f, "{:>column$}{}", "", '^'.yellow().bold())?;
            }
            _ => writeln!(f, ":")?,
        }

        // Trim pest error format
        let msg = msg
            .rsplit_once('=')
            .map(|(_, right)| right.trim())
            .unwrap_or(msg);

        write!(f, "{msg}")
    }
}
