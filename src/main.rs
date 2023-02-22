#![allow(unused)]

mod cli;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::thread;
use cli::Cli;
use clap::Parser;
use colored::Colorize;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;
use colored::*;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_THREADS: usize = 4;
fn main() {
    let max_thread_warning = "Max thead".red();
    let command = Cli::parse();
    let files = command.path();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    if files.is_file() {
        read_file(&files, command.pattern())
    }
    else if files.is_dir() {
        for file in WalkDir::new(files).into_iter().filter_map(|e| e.ok()) {
            if handles.len() == MAX_THREADS {
                for _ in 0..MAX_THREADS {
                    handles.pop().unwrap().join().unwrap();
                }
            }

            let cloned_keyword = String::from(command.pattern());
            let path_buf = PathBuf::from(&file.path());

            if file.metadata().unwrap().is_file() {
                let handle = thread::spawn(move || {
                    read_file(&path_buf, &cloned_keyword);
                });
                handles.push(handle);
            }


        }
        for handle in handles {
            handle.join().unwrap();
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


