use std::{collections::{HashMap, VecDeque}, fs::{ReadDir, self}, process, time::SystemTime};

use crate::indexing::{parser::Parser, tokenizer::Tokenizer};

use self::parser::Class;

mod token;
mod tokenizer;
mod parser;

pub type ClassDependencyIndex = HashMap<String, usize>;

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub class: Class,
    pub lines: usize,
    pub last_accessed: usize,
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


pub fn index(dir_entry: ReadDir) -> (ClassDependencyIndex, Vec<File>) {
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

    (index, files)
}
