use colored::Colorize;
use std::{fmt::Display, process, time::SystemTime};

use crate::{ClassDependencyIndex, File};

mod io;

pub enum SortType {
    ClassComplexity,
    Uses,
    Dependencies,
    FunctionComplexity,
}

pub struct ViewOptions {
    dependencies: bool,
    top_files: usize,
    num_functions: Option<usize>,
    function_stmts: bool,
    query: Option<String>,
}

impl ViewOptions {
    pub fn default() -> Self {
        Self {
            dependencies: true,
            top_files: 10,
            num_functions: None,
            function_stmts: true,
            query: None,
        }
    }
}

impl Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortType::Dependencies => "dependencies",
                SortType::Uses => "uses",
                SortType::ClassComplexity => "class complexity",
                SortType::FunctionComplexity => "function complexity",
            }
        )
    }
}

pub fn run_program(index: &ClassDependencyIndex, files: &mut [File]) {
    let sort_type = SortType::ClassComplexity;
    println!();
    println!("Sorting by: {sort_type}");
    let now = SystemTime::now();
    let diff = now.elapsed().unwrap().as_millis() as f64;
    sort_files(files, sort_type, index);
    println!(
        "Sorted {} files in {:.4} seconds.",
        files.len(),
        diff / 1000.0
    );

    let mut view_options = ViewOptions::default();

    io::display_title("TCPL Scanner");

    loop {
        println!("    ");
        println!("{}", "* ----------------------- *".red().bold());
        println!("    ");
        io::display_underlined_colored("Options");
        println!("1. View top files");
        println!("2. See view options");
        println!("3. Update view options");
        println!("4. Search for a file");
        println!("5. Re-sort files");
        println!("8. Exit\n");

        let option = match io::get_usize_input("Enter an option") {
            Ok(option) => option,
            Err(error) => {
                io::display_error(&error);
                continue;
            }
        };
        println!();

        match option {
            1 => display_files(files, index, &view_options),
            2 => display_view_options(&view_options),
            3 => update_view_options(&mut view_options),
            4 => search(files, index, &mut view_options),
            5 => re_sort(files, index),
            8 => exit(),
            _ => io::display_error("That's not right, try again!"),
        }
    }
}

fn update_view_options(view_options: &mut ViewOptions) {
    loop {
        display_view_options(view_options);
        io::display_title("Choose option to update");
        println!("1. Update number of files");
        println!("2. Toggle display dependencies");
        println!("3. Max number of methods per class");
        println!("4. Toggle display function statements");
        println!("8. Done\n");

        let option = match io::get_usize_input("Enter an option") {
            Ok(option) => option,
            Err(error) => {
                io::display_error(&error);
                continue;
            }
        };

        match option {
            1 => {
                let num =
                    match io::get_usize_input("Choose how many of the top files you want to see") {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                view_options.top_files = num;
                break;
            }
            2 => {
                let message = if view_options.dependencies {
                    "Dependencies disabled"
                } else {
                    "Dependencies enabled"
                };
                view_options.dependencies = !view_options.dependencies;
                io::display_sucess(message);
                break;
            }
            3 => {
                println!();
                println!("1. Yes");
                println!("2. No (see all functions)");
                let num =
                    match io::get_usize_input("Would you a fixed number of methods per class?") {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                match num {
                    2 => {
                        view_options.num_functions = None;
                        continue;
                    }
                    1 => (),
                    _ => {
                        io::display_error("Wrong option. Try again");
                        continue;
                    }
                }
                let num =
                    match io::get_usize_input("Would you a fixed number of methods per class?") {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                view_options.num_functions = Some(num);
                break;
            }
            4 => {
                let message = if view_options.function_stmts {
                    "Function statements disabled"
                } else {
                    "Function statements enabled"
                };
                view_options.function_stmts = !view_options.function_stmts;
                io::display_sucess(message);
                break;
            }
            8 => break,
            _ => io::display_error("Incorrect option. Try again."),
        }
    }
}

fn display_view_options(view_options: &ViewOptions) {
    io::display_title("View Options");
    println!("Number of files: {}", view_options.top_files);
    println!("Display dependencies: {}", view_options.dependencies);
    let fn_per_class = match view_options.num_functions {
        Some(num) => num.to_string(),
        None => "All".to_string(),
    };
    println!("Functions per class: {}", fn_per_class);
    println!(
        "Display function statements: {}",
        view_options.function_stmts
    );
}

fn search(files: &[File], index: &ClassDependencyIndex, view_options: &mut ViewOptions) {
    let query = match io::get_string_input("Enter query") {
        Ok(query) => query,
        Err(_) => return,
    };
    view_options.query = Some(query);
    display_files(files, index, view_options);
    view_options.query = None;
}

pub fn display_files(files: &[File], index: &ClassDependencyIndex, view_options: &ViewOptions) {
    println!();
    io::display_title("Top Files");
    for (i, file) in files
        .iter()
        .filter(|file| {
            if let Some(query) = &view_options.query {
                file.class
                    .name
                    .to_lowercase()
                    .contains(query.to_lowercase().as_str())
            } else {
                true
            }
        })
        .take(view_options.top_files)
        .enumerate()
    {
        let class = &file.class;
        io::display_underlined_colored(format!("{}. {}", i + 1, class.name).as_str());
        println!("Last accessed {} hours ago", file.last_accessed);
        println!("Path: {}", file.path);
        println!("Lines: {}", file.lines);
        println!("Used in {} places", index.get(&class.name).unwrap());
        if class.dependencies.is_empty() {
            println!("No dependencies");
        } else {
            println!("Dependencies: {}", class.dependencies.len());
            if view_options.dependencies {
                println!("* ------ *");
                io::display_list(&class.dependencies);
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
        println!("Extends: {}", extends);
        if class.implements.is_empty() {
            println!("Implements: None");
        } else {
            println!("Implements:");
            for (i, interface) in class.implements.iter().enumerate() {
                println!(" {}. {interface}", i + 1);
            }
        }
        println!("Abstract: {}", class.is_abstract);
        let functions = match view_options.num_functions {
            Some(num) => {
                if class.functions.len() >= num {
                    &class.functions[..num]
                } else {
                    &class.functions
                }
            }
            None => &class.functions,
        };
        for function in functions {
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
            if view_options.function_stmts {
                for stmt in function.stmts.iter() {
                    println!("  {:?}", stmt);
                }
            }
        }
        println!("* ---------- *");
    }
}

fn re_sort(files: &mut [File], index: &ClassDependencyIndex) {
    io::display_title("Sort Options");
    println!("  1. Average cyclomatic complexity of a class");
    println!("  2. Usages of a class");
    println!("  3. Number of dependencies of a class");
    println!("  4. Maximum method complexity");

    let input = match io::get_usize_input("Choose a sorting option") {
        Ok(num) => num,
        Err(_) => return,
    };

    match input {
        1 => sort_files(files, SortType::ClassComplexity, index),
        2 => sort_files(files, SortType::Uses, index),
        3 => sort_files(files, SortType::Dependencies, index),
        4 => sort_files(files, SortType::FunctionComplexity, index),
        _ => io::display_error("Wrong input"),
    }
}

fn sort_files(files: &mut [File], sort_type: SortType, index: &ClassDependencyIndex) {
    match sort_type {
        SortType::ClassComplexity => {
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
            files.sort_by(|a, b| b.class.dependencies.len().cmp(&a.class.dependencies.len()));
        }
        SortType::FunctionComplexity => {
            files.sort_by(|a, b| {
                b.class
                    .highest_complexity_function()
                    .cmp(&a.class.highest_complexity_function())
            });
        }
    }
}

fn exit() {
    io::display_sucess("Bye!");
    process::exit(0);
}
