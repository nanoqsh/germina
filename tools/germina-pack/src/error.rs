use crate::{info, pack};
use std::{
    fmt,
    io::{self, Stderr, Write},
    path::PathBuf,
};

pub enum Error {
    Pack { err: pack::Error, path: PathBuf },
    Info { err: info::Error, path: PathBuf },
}

impl Error {
    pub fn exit(self) -> ! {
        let report = Report::start();

        match self {
            Self::Pack { err, path } => {
                report.print(&path.display());
                report.print(&"\n");
                report.print(&err);
            }
            Self::Info { err, path } => {
                report.print(&path.display());
                report.print(&"\n");
                report.print(&err);
            }
        }

        std::process::exit(1)
    }
}

struct Report {
    out: Stderr,
}

impl Report {
    fn start() -> Self {
        use crossterm::{style, QueueableCommand};

        Self {
            out: {
                let mut out = io::stderr();
                out.queue(style::SetForegroundColor(style::Color::Red))
                    .expect("queue")
                    .queue(style::Print("error:\n"))
                    .expect("queue")
                    .queue(style::ResetColor)
                    .expect("queue");
                out
            },
        }
    }

    fn print(&self, display: &dyn fmt::Display) {
        write!(&self.out, "{display}").expect("write");
    }
}

impl Drop for Report {
    fn drop(&mut self) {
        self.out.flush().expect("flush");
    }
}
