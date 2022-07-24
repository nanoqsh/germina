mod error;
mod info;
mod pack;

use crate::error::Error;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Shows a kit info
    Info {
        /// The kit's path
        path: String,
    },
    /// Pack a kit from source
    Pack {
        /// The source directory
        src: String,
        /// The kit's name
        #[clap(short, long)]
        name: Option<String>,
        /// Sets the flag whether to rewrite an old file
        #[clap(short, long)]
        rewrite: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli.command) {
        err.exit()
    }
}

fn run(command: Command) -> Result<(), Error> {
    match command {
        Command::Info { path } => {
            let path = PathBuf::from(path);
            let info = crate::info::info(&path).map_err(|err| Error::Info { err, path })?;
            println!("{info}");
        }
        Command::Pack { src, name, rewrite } => {
            use crate::pack::Options;
            use crossterm::style::Stylize;

            let path = PathBuf::from(src);
            let arch_path = crate::pack::pack(
                &path,
                Options {
                    name: name.as_deref(),
                    rewrite,
                },
            )
            .map_err(|err| Error::Pack { err, path })?;

            let filename = arch_path.file_name().expect("filename").to_string_lossy();
            println!("a kit saved in {}", filename.bold());
        }
    }

    Ok(())
}
