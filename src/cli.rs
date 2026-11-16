use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[ command(version, about = "Mijn eigen versie van Git", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    HashObject {
        file: PathBuf,

        #[arg(short)]
        w: bool,
    }
}