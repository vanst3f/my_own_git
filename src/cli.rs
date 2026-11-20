use clap::{Parser, Subcommand};
use std::path::{PathBuf, Path};
use std::error::Error;
use std::env;

#[derive(Parser)]
#[ command(version, about = "My own version of Git", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        dir_name: Option<String>,
    },
    HashObject {
        #[arg(required_unless_present = "stdin")]
        file: Option<PathBuf>,

        #[arg(short)]
        w: bool,

        #[arg(long)]
        stdin: bool,
    },
}

impl Command {
    pub fn check_init(&self) -> Result<(), Box<dyn Error>> {
        let project_exists = Path::new(".my_own_git").exists();

        match self {
            Command::Init { .. } => {
                if project_exists {
                    let current_path = env::current_dir()?;
                    println!("Reinitialized existing Git repository in {}/", current_path.display());
                }
            }
            Command::HashObject { w: true, .. } => {
                if !project_exists {
                    return Err("not a my_own_git repository (or any of the parent directories): .my_own_git".into());
                }
            } _ => {}
        }

        Ok(())
    }
}