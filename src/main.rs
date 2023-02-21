#![allow(unused)]

mod cli;

use std::fs::File;
use std::io::{BufRead, BufReader, self, Write, Seek};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use cli::Cli;
use clap::Parser;
use colored::Colorize;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;
use colored::*;

fn main() {
    let command = Cli::parse();
    let files = command.path();

    if files.is_file() {
        read_file(&files, command.pattern())
    }
    else if files.is_dir() {
        for file in WalkDir::new(files).into_iter().filter_map(|e| e.ok()) {
            if file.metadata().unwrap().is_file() {
                let path_buf = PathBuf::from(&file.path());
                read_file(&path_buf, command.pattern())
            }
        }
    }

}


fn read_file(path_buf: &PathBuf, pattern: &str) {
    let file = File::open(path_buf);

    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };
    
    for (i,line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return,
        };

        if line.contains(pattern) {
            let number_of_line = (i + 1).to_string().cyan();
            let file_path = path_buf.display().to_string().magenta();
            println!("Line {} of file {}", number_of_line, file_path);
            let line_to_print = line.replace(pattern, &pattern.green().to_string());
            println!("{}", line_to_print);
        }
    }
}


