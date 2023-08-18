use indexing::{ClassDependencyIndex, File};

use crate::{indexing::index, interface::run_program};
use std::{env, fs, process};

mod indexing;
mod interface;

fn main() {
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
}

fn error(msg: &str, line: usize) {
    eprintln!("ERR: line: {line} {msg}");
    process::exit(1);
}
