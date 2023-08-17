use indexing::{ClassDependencyIndex, File};

use crate::{interface::run_program, indexing::index};
use std::{
    env,
    fs,
    process,
};

mod git;
mod interface;
mod indexing;

fn run() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap_or_else(|| {
        eprintln!("ERROR: Please input file path to scan");
        process::exit(1);
    });

    let dir_entry = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to read directory, {err}");
        process::exit(1);
    });

    let (index, mut files) = index(dir_entry);

    run_program(&index, &mut files);

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}

fn error(msg: &str, line: usize) {
    eprintln!("ERR: line: {line} {msg}");
    process::exit(1);
}

