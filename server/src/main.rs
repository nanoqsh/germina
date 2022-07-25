mod config;
mod error;
mod load;

use crate::{config::Config, error::Error, load::KitSource};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// The config path
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Make a new world
    Make {
        /// A kit or assembly path
        path: String,
        /// A world name
        name: String,
    },
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        err.exit()
    }
}

fn run(cli: Cli) -> Result<(), Error> {
    let config = {
        let path = cli.config.as_deref().unwrap_or("config.json").as_ref();
        Config::load(path).map_err(|err| Error::Config {
            err,
            path: path.into(),
        })?
    };

    let _ = config.net.addr();

    match cli.command {
        Command::Make { path, .. } => {
            let kit = KitSource::load(path.as_ref()).map_err(|err| Error::Load {
                err,
                path: path.into(),
            })?;

            println!("kit: {}", kit.name);
            println!("tiles:");
            for (key, _) in kit.model.tiles.iter() {
                println!("    {key}");
            }

            println!("tile sprites:");
            for (key, _) in kit.model.tile_sprites.iter() {
                println!("    {key}");
            }

            Ok(())
        }
    }
}
