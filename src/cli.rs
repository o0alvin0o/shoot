use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    //mode: Mode,
    path: PathBuf
}

impl Cli {
    fn new(pattern: &str, mode: &str) -> Self {
        // let mode = match mode {
        //     "strict" => Mode::Strict,
        //     "soft" => Mode::Soft,
        // };
        let pattern = pattern.to_owned();

        let path = match std::env::consts::OS {
            "windows" => PathBuf::from(r"C:\"),
            _ => PathBuf::from("/"),
        };
        Cli { pattern, path }
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

enum Mode {
    Strict,
    Soft,
}