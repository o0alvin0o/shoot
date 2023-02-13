#![allow(unused)]

mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path()).expect("could not read file");
    for (i,line) in content.lines().enumerate() {
        if line.contains(args.pattern()) {
            println!("Found in line {i}:");
            println!("{line}");
        }
    }

}
