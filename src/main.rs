use crate::parser::Parser;
use parser::Class;
use std::{
    env,
    fs::{self, ReadDir},
    process,
    time::SystemTime,
};
use tokenizer::Tokenizer;

mod git;
mod parser;
mod token;
mod tokenizer;

#[derive(Debug)]
struct File {
    path: String,
    class: Class,
    lines: usize,
    last_accessed: usize,
}

#[derive(Debug)]
struct RawFile {
    path: String,
    content: Vec<char>,
    last_accessed: usize,
}

impl File {
    fn new(path: &str, class: Class, lines: usize, last_accessed: usize) -> Self {
        Self {
            path: path.to_string(),
            class,
            lines,
            last_accessed,
        }
    }
}

fn read_dir(dir_entry: ReadDir, files: &mut Vec<RawFile>) {
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
            if path
                .as_path()
                .extension()
                .is_some_and(|extension| extension != "php")
            {
                println!("Skipping path: {:?}", path);
                return;
            }
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|err| {
                    eprintln!(
                        "ERROR: Failed to read file with path: {:?}, err: {:?}",
                        path, err
                    );
                    process::exit(1);
                })
                .chars()
                .collect::<Vec<_>>();
            println!("{:?}", &path);
            let now = SystemTime::now();
            let accessed = metadata.accessed().unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to read accessed date, {err}");
                process::exit(1);
            });
            let last_accessed = now.duration_since(accessed).unwrap().as_secs() / 3600;
            let file = RawFile {
                path: path.into_os_string().into_string().unwrap(),
                content,
                last_accessed: last_accessed as usize,
            };
            files.push(file);
        }
        if metadata.is_dir() {
            let path = entry.path();
            let dir_entry = fs::read_dir(path).unwrap_or_else(|err| {
                eprintln!("ERROR: Failed to read directory, {err}");
                process::exit(1);
            });
            read_dir(dir_entry, files);
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

    let top_files: usize = match args.get(2) {
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
    let mut raw_files: Vec<RawFile> = Vec::new();

    read_dir(dir_entry, &mut raw_files);

    raw_files.iter().for_each(|file| {
        let tokens = Tokenizer::new(&file.content).collect::<Vec<_>>();
        let parser = Parser::new(&tokens);
        files.push(File::new(
            file.path.as_str(),
            parser.class,
            match tokens.last() {
                Some(token) => token.line,
                None => 0,
            },
            file.last_accessed,
        ));
    });
    println!("Finished scanning.");

    files.sort_by(|a, b| {
        b.class
            .average_complexity()
            .total_cmp(&a.class.average_complexity())
    });

    println!();
    println!("Top files");
    println!("* ---------- *");
    for (i, file) in files.iter().take(top_files).enumerate() {
        let class = &file.class;
        println!("{i}. {}", class.name);
        println!("Last accessed {} hours ago", file.last_accessed);
        println!("Path: {}", file.path);
        println!("Lines: {}", file.lines);
        println!(
            "Average cyclomatic complexity: {}",
            class.average_complexity()
        );
        println!(
            "Max cyclomatic complexity: {}",
            class.highest_complexity_function()
        );
        println!("Functions: {}", class.functions.len());
        for function in class.functions.iter() {
            println!("* -------- *");
            println!("  Name: {}", function.name);
            let return_type = if function.name == "__construct" {
                "self".to_string()
            } else {
                match &function.return_type {
                    Some(return_type) => return_type.clone(),
                    None => "Not provided".to_string(),
                }
            };
            println!("  Return type: {return_type}");
            println!("  Param count: {}", function.params);
            println!("  Cyclomatic complexity: {}", function.complexity());
            for stmt in function.stmts.iter() {
                println!("  {:?}", stmt);
            }
        }
        println!("* ---------- *");
    }
}

fn error(msg: &str, line: usize) {
    eprintln!("ERR: line: {line} {msg}");
    process::exit(1);
}
