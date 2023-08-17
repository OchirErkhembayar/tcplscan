
use colored::Colorize;

pub fn display_sucess(message: &str) {
    println!("\n{}\n", message.green());
}

pub fn display_danger(message: &str) {
    println!("\n{}\n", message.red());
}

pub fn display_title(message: &str) {
    println!("\n* --- {} --- *\n", message.yellow().underline());
}

pub fn display_underlined_colored(message: &str) {
    println!("{}", message.yellow().underline());
}

pub fn display_error(message: &str) {
    println!("\nError: {}\n", message.red());
}

pub fn display_list(items: &Vec<String>) {
    for (i, item) in items.iter().enumerate() {
        println!("  {}. {item}", i + 1);
    }
}

pub fn get_string_input(message: &str) -> Result<String, String> {
    println!("{}", message.blue());
    let mut input = String::new();
    let input = match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => return Err("Failed to read input".to_string()),
    };
    if input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    Ok(input)
}

pub fn get_usize_input(message: &str) -> Result<usize, String> {
    println!("{}", message.blue());
    let mut input = String::new();
    let input = match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => return Err("Failed to read input".to_string()),
    };
    let input = match input.parse::<usize>() {
        Ok(input) => input,
        Err(_) => return Err("Failed to parse input".to_string()),
    };
    Ok(input)
}

pub fn get_positive_f32_input(message: &str) -> Result<f32, String> {
    println!("{}", message.blue());
    let mut input = String::new();
    let input = match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => return Err("Failed to read input".to_string()),
    };
    let input = match input.parse::<f32>() {
        Ok(input) => input,
        Err(_) => return Err("Failed to parse input".to_string()),
    };
    if input <= 0.0 {
        return Err("Input must be positive".to_string());
    }
    Ok(input)
}
