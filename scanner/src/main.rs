use std::{
    collections::HashMap,
    env,
    fs::{self, ReadDir},
    process,
    time::SystemTime,
};
use tokenizer::{TokenType, Tokenizer};

#[derive(Debug)]
struct File {
    path: String,
    complexity: HashMap<TokenType, u8>,
}

impl File {
    fn new(path: &str, complexity: HashMap<TokenType, u8>) -> Self {
        Self {
            path: path.to_string(),
            complexity,
        }
    }
}

fn read_dir(dir_entry: ReadDir, extension: Option<&str>, file: &mut File, files: &mut Vec<File>) {
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
                "Last accessed {:?} minutes ago",
                now.duration_since(accessed).unwrap().as_secs() / 60
            );
            let mut complexity: HashMap<TokenType, u8> = HashMap::new();
            for token in Tokenizer::new(&(content.chars().collect::<Vec<_>>())) {
                complexity
                    .entry(token.token_type)
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
            read_dir(dir_entry, extension, file, files);
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

    let dir_entry = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to read directory, {err}");
        process::exit(1);
    });

    let mut file = File::new("", HashMap::new());
    let mut files: Vec<File> = Vec::new();

    read_dir(dir_entry, extension, &mut file, &mut files);
    println!("Finished scanning.");

    files.sort_by(|a, b| b.complexity.values().sum::<u8>().cmp(&a.complexity.values().sum::<u8>()));

    for (i, file) in files.iter().take(3).enumerate() {
        println!("{} complexity file: {:?}", i + 1, file);
    }
}
