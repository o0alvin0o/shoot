use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    #[arg(long, short)]
    pattern: String,
    /// The path to the file to read
    path: Option<PathBuf>,
    /// Sub-command
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {

    pub fn is_dir_search(&self) -> bool {
        match &self.path {
            Some(p) => {
                p.is_dir()
            },
            None => false,
        }
    }

    pub fn is_file_search(&self) -> bool {
        match &self.path {
            Some(p) => {
                p.is_file()
            },
            None => false,
        }
    }

    pub fn path(&self) -> PathBuf {
        match &self.path {
            Some(p) => p.clone(),
            None => match env::consts::OS {
                windows=> PathBuf::from("/"),
                _ => PathBuf::from(r"C:\"),
            },
        }
    }

    pub fn pattern (&self) -> &str {
        &self.pattern
    }

    pub fn validate_command(&mut self) {
        // TODO
    }
}

#[derive(Subcommand)]
enum Commands {
    Strict,
    Soft,
    Directory,
    File,
}
