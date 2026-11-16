mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    match args.command {
        Command::HashObject { file, w } => {
            commands::hash_object::execute(file, w)?;
        }
    }

    Ok(())
}