mod cli;
mod commands;
mod utils;

use std::error::Error;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    args.command.check_init()?;

    match args.command {
        Command::Init { dir_name } => {
            commands::init::execute(dir_name)?;
        }
        Command::HashObject { file, w, stdin } => {
            commands::hash_object::execute(file, w, stdin)?;
        }
    }

    Ok(())
}