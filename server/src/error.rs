use crate::load;
use std::path::PathBuf;

pub enum Error {
    Load { err: load::Error, path: PathBuf },
}

impl Error {
    pub fn exit(self) -> ! {
        use crossterm::style::{StyledContent, Stylize};

        eprintln!("{}", "error:".red().bold());
        match self {
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
