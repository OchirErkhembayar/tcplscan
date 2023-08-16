use crate::parser::Parser;
use parser::Class;
use std::{
    collections::{HashMap, VecDeque},
    env,
    fmt::Display,
    fs::{self, ReadDir},
    process,
    time::SystemTime,
};
use tokenizer::Tokenizer;

mod git;
mod parser;
mod token;
mod tokenizer;
mod types;

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
            match path.as_path().extension() {
                Some(extension) => {
                    if extension != "php" {
                        return;
                    }
                }
                None => return,
            }
            let content = match fs::read_to_string(&path) {
                Ok(content) => content.chars().collect::<Vec<_>>(),
                Err(err) => {
                    eprintln!(
                        "ERROR: Failed to read file with path: {:?}, err: {:?}",
                        path, err
                    );
                    return;
                }
            };
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

type ClassDependencyIndex = HashMap<String, usize>;

enum SortType {
    Complexity,
    Uses,
    Dependencies,
}

impl Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortType::Dependencies => "dependencies",
                SortType::Uses => "uses",
                SortType::Complexity => "complexity",
            }
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap_or_else(|| {
        eprintln!("ERROR: Please input file path to scan");
        process::exit(1);
    });

    // handle these options properly
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

    let sort_type = match args.get(3) {
        Some(sort_type) => match sort_type.as_str() {
            "c" => SortType::Complexity,
            "u" => SortType::Uses,
            "d" => SortType::Dependencies,
            _ => {
                eprintln!("ERROR: {sort_type} is not a valid sort type. Options are:\n\tc - Complexity\n\tu - Usage count\n\td - Dependency count");
                process::exit(1);
            }
        },
        None => SortType::Dependencies,
    };

    let dir_entry = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to read directory, {err}");
        process::exit(1);
    });

    let mut files: Vec<File> = Vec::new();
    let mut raw_files: Vec<RawFile> = Vec::new();

    let now = SystemTime::now();
    read_dir(dir_entry, &mut raw_files);
    let diff = now.elapsed().unwrap().as_millis() as f64;
    println!(
        "Filtered out and read {} files in {:.4} seconds.",
        raw_files.len(),
        diff / 1000.0
    );

    let mut parser = Parser::new();
    let now = SystemTime::now();
    raw_files.iter().for_each(|file| {
        let tokens = Tokenizer::new(&file.content).collect::<VecDeque<_>>();
        let line = match tokens.back() {
            Some(token) => token.line,
            None => 0,
        };
        if let Some(class) = parser.parse_file(tokens) {
            let file = File::new(file.path.as_str(), class, line, file.last_accessed);
            files.push(file);
        }
    });
    let diff = now.elapsed().unwrap().as_millis() as f64;
    println!(
        "Finished scanning and parsing {} files in {:.4} seconds.",
        files.len(),
        diff / 1000.0
    );

    let now = SystemTime::now();
    let mut index = ClassDependencyIndex::new();
    for file in files.iter() {
        let class = &file.class;
        index.entry(class.name.to_owned()).or_insert(0);
        for dependency in class.dependencies.iter() {
            index
                .entry(dependency.to_owned())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    let diff = now.elapsed().unwrap().as_millis() as f64;
    println!("Indexed classes in {:.4} seconds", diff / 1000.0);

    println!();
    println!("Sorting by: {sort_type}");
    let now = SystemTime::now();
    let diff = now.elapsed().unwrap().as_millis() as f64;
    sort_by(&mut files, sort_type, &index);
    println!(
        "Sorted {} files in {:.4} seconds.",
        files.len(),
        diff / 1000.0
    );

    println!();
    println!("Top files");
    println!("* ---------- *");
    for (i, file) in files.iter().take(top_files).enumerate() {
        let class = &file.class;
        println!("{}. {}", i + 1, class.name);
        println!("Last accessed {} hours ago", file.last_accessed);
        println!("Path: {}", file.path);
        println!("Lines: {}", file.lines);
        println!("Used in {} places", index.get(&class.name).unwrap());
        if class.dependencies.is_empty() {
            println!("No dependencies");
        } else {
            println!("{} dependencies", class.dependencies.len());
            println!("* ------ *");
            for dependency in &class.dependencies {
                println!("Dependency: {dependency}");
            }
        }
        println!(
            "Average cyclomatic complexity: {}",
            class.average_complexity()
        );
        println!(
            "Max cyclomatic complexity: {}",
            class.highest_complexity_function()
        );
        println!("Functions: {}", class.functions.len());
        let extends = match class.extends.to_owned() {
            Some(extends) => extends,
            None => "None".to_string(),
        };
        let implements = match class.implements.to_owned() {
            Some(implements) => implements,
            None => "None".to_string(),
        };
        println!("Extends: {}", extends);
        println!("Implements: {}", implements);
        println!("Abstract: {}", class.is_abstract);
        for function in class.functions.iter() {
            println!("* -------- *");
            println!("  Name: {}", function.name);
            println!("  Visibility: {}", function.visibility);
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

fn sort_by(files: &mut [File], sort_type: SortType, index: &ClassDependencyIndex) {
    match sort_type {
        SortType::Complexity => {
            files.sort_by(|a, b| {
                b.class
                    .average_complexity()
                    .total_cmp(&a.class.average_complexity())
            });
        }
        SortType::Uses => {
            files.sort_by(|a, b| index.get(&b.class.name).cmp(&index.get(&a.class.name)));
        }
        SortType::Dependencies => {
            files.sort_by(|a, b| b.class.dependencies.cmp(&a.class.dependencies));
        }
    }
}
