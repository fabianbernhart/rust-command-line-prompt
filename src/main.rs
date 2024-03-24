

use std::io::{self, Read, Write};
use std::path::Path;
use std::fs::File;

use colored::Colorize;

fn main() {
    println!();
    println!();
    println!("Welcome to my Rust Command Line Prompt!");

    println!("{}", "Commands:".yellow());
    println!("echo: echo [...]");
    println!("cat: cat [filename]");
    println!("exit: exit the prompt");

    'command_prompt: loop {
        
        let mut input: String = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut error_string: String = String::new();
        let mut result_string: String = String::new();
        let mut command: &str = &String::new();

        input.clear();
        error_string.clear();
        result_string.clear();
        args.clear();




        print!("{}", "$ rsp ".magenta());


        io::stdout().flush().expect("Cannot flush stdout");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");

        let mut splitted_input: std::str::SplitWhitespace<'_> = input.trim().split_whitespace();

        if let Some(cmd) = splitted_input.next() {
            command = cmd;
            args.extend(splitted_input.map(String::from));
        }


        match command {
            "exit" => break 'command_prompt,
            "echo" => {
                result_string = exec_echo(args.clone());
            },
            "cat" => {
                if let Some(filename) = args.get(0) {
                    match cat_command(filename) {
                        Ok(contents) => {
                            result_string = contents;
                        }
                        Err(e) => error_string = e.to_string(),
                    }
                } else {
                    error_string = "No filename provided".to_string();
                }
            }

            _ => error_string = "Command not found type help".to_string(),
        }
        
        if !error_string.is_empty() {
            println!("{} {}", "Error:".red().bold(), error_string)
        } else {
            println!("{} {}", "Result: \n".green().bold(), result_string)
        }
    }
    println!("Closing Command Prompt!");
}


fn exec_echo(args: Vec<String>) -> String {
    args.join(" ")
}

#[cfg(test)]
mod echo_tests {
    use super::exec_echo;
    #[test]
    fn test_exec_echo() {
        let args: Vec<String> = vec!["Hello".to_string(), "World!".to_string(), 911.to_string(), "💖".to_string()];
    
        assert_eq!("Hello World! 911 💖", exec_echo(args));
    }
}

fn cat_command(file_path: &str) -> io::Result<String> {
    let path: &Path = Path::new(file_path);
    let mut file: File = File::open(path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod cat_tests {
    use super::*;

    #[test]
    fn test_cat_existing_file() {
        let file_path = "test_file.txt";
        let contents = "Hello, world!";
        std::fs::write(file_path, contents).expect("Failed to write test file");
        let result = cat_command(file_path);
        assert_eq!(result.unwrap(), contents);
        std::fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_cat_nonexistent_file() {
        let result = cat_command("nonexistent_file.txt");
        assert!(result.is_err());
    }
}

