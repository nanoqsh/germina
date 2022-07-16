mod error;
mod info;
mod pack;

use crate::error::Error;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Shows a box info
    Info {
        /// The box's path
        path: String,
    },
    /// Pack a box from source
    Pack {
        /// The source directory
        src: String,
        /// The box's name
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
        Command::Info { path } => todo!("{path:?}"),
        Command::Pack { src, name, rewrite } => {
            use pack::Options;

            let path = PathBuf::from(src);
            let arch_path = pack::pack(
                &path,
                Options {
                    name: name.as_deref(),
                    rewrite,
                },
            )
            .map_err(|err| Error::Pack { err, path })?;
            println!("written in {}", arch_path.display());
        }
    }

    Ok(())
}
