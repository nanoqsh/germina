use {
    crate::{info, pack},
    std::path::PathBuf,
};

pub enum Error {
    Pack { err: pack::Error, path: PathBuf },
    Info { err: info::Error, path: PathBuf },
}

impl Error {
    pub fn exit(self) -> ! {
        use crossterm::style::{ContentStyle, StyledContent, Stylize};

        eprint!("{} ", "error:".red().bold());
        match self {
            Self::Pack { err, path } => {
                eprintln!(
                    "in file {}",
                    StyledContent::new(ContentStyle::default(), path.display()).bold()
                );
                eprint!("{err}");
            }
            Self::Info { err, path } => {
                eprintln!(
                    "in file {}",
                    StyledContent::new(ContentStyle::default(), path.display()).bold()
                );
                eprint!("{err}");
            }
        }

        std::process::exit(1)
    }
}
