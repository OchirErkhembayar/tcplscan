use parser::StmtType;
use std::{
    collections::HashMap,
    env,
    fs::{self, ReadDir},
    process,
    time::SystemTime,
};
use tokenizer::Tokenizer;

use crate::parser::Parser;

mod git;
mod parser;
mod token;
mod tokenizer;

#[derive(Debug)]
struct File {
    path: String,
    complexity: HashMap<StmtType, u32>,
}

impl File {
    fn new(path: &str, complexity: HashMap<StmtType, u32>) -> Self {
        Self {
            path: path.to_string(),
            complexity,
        }
    }
}

fn read_dir(dir_entry: ReadDir, extension: Option<&str>, files: &mut Vec<File>) {
    dir_entry.for_each(|entry| {
        let entry = entry.unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to parse directory entry, {err}");
            process::exit(1);
        });
        let metadata = entry.metadata().unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to parse directory entry, {err}");
            process::exit(1);
        });
        if metadata.is_file() {
            let path = entry.path();
            if extension.is_some_and(|to_skip| {
                path.as_path()
                    .extension()
                    .is_some_and(|extension| extension != to_skip)
            }) {
                println!("Skipping path: {:?}", path);
                return;
            }
            let content = fs::read_to_string(&path).unwrap_or_else(|err| {
                eprintln!(
                    "ERROR: Failed to read file with path: {:?}, err: {:?}",
                    path, err
                );
                process::exit(1);
            });
            println!("{:?}", &path);
            let now = SystemTime::now();
            let accessed = metadata.accessed().unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to read accessed date, {err}");
                process::exit(1);
            });
            println!(
                "Last accessed {:?} hours ago",
                now.duration_since(accessed).unwrap().as_secs() / 60 / 60
            );
            let mut complexity: HashMap<StmtType, u32> = HashMap::new();
            let tokens = Tokenizer::new(&(content.chars().collect::<Vec<_>>())).collect::<Vec<_>>();
            let mut parser = Parser::new(&tokens);
            parser.parse();
            for stmt in parser.stmts {
                complexity
                    .entry(stmt.kind)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            println!("Complexity: {:?}", complexity);
            files.push(File::new(
                path.into_os_string().into_string().unwrap().as_str(),
                complexity,
            ));
        }
        if metadata.is_dir() {
            let path = entry.path();
            let dir_entry = fs::read_dir(&path).unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to read directory, {err}");
                process::exit(1);
            });
            read_dir(dir_entry, extension, files);
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let path = args.get(1).unwrap_or_else(|| {
        eprintln!("ERROR: Please input file path to scan");
        process::exit(1);
    });

    let extension = match args.get(2) {
        Some(ext) => {
            println!("Running on {} files", ext);
            Some(ext.as_str())
        }
        None => {
            println!("Running without file extension filter");
            None
        }
    };

    let top_files: usize = match args.get(3) {
        Some(num) => {
            println!("Getting top {num} files");
            num.parse().unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to parse {num} into a number, {err}");
                process::exit(1);
            })
        }
        None => 3,
    };

    let dir_entry = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to read directory, {err}");
        process::exit(1);
    });

    let mut files: Vec<File> = Vec::new();

    read_dir(dir_entry, extension, &mut files);
    println!("Finished scanning.");

    files.sort_by(|a, b| {
        b.complexity
            .values()
            .sum::<u32>()
            .cmp(&a.complexity.values().sum::<u32>())
    });

    for (i, file) in files.iter().take(top_files).enumerate() {
        println!("{} complexity file: {}", i + 1, file.path);
        let mut score = 0;
        for stmt in file.complexity.iter() {
            println!("{:?}", stmt);
            score += stmt.1;
        }
        println!("Overall score: {score}");
    }
}

fn error(msg: &str, line: usize) {
    eprintln!("ERR: line: {line} {msg}");
    process::exit(1);
}
