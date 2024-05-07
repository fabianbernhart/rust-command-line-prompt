
use std::ffi::OsString;
use std::fmt::Error;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::{self, File, ReadDir};

use colored::Colorize;


struct Command {
    name: String,
    args: Vec<String>
}

//todo: Refactor to parser.rs
fn parse_command_input (input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();

    let name = match parts.next() {
        Some(first_part) => first_part.to_string(),
        None => panic!("No name found in input"),
    };

    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    Command {name, args}
}


pub enum ExecutionResult {
    Success,
    Failure,
}


// todo: Refactor to executor.rs
fn execute_command(command: &Command) -> ExecutionResult {

    let args: Vec<String> = command.args.clone();   

    match command.name.as_str() {
        "echo" => {
            let _ = exec_echo(args);
            ExecutionResult::Success
        },
        "ls" => {
            ls();
            ExecutionResult::Success
        },
        "cat" => {
            let _ = cat_command(args);
            ExecutionResult::Success
        },

        _ => {
            println!("Command not found: {}", command.name);
            ExecutionResult::Failure
        }
    }






}

fn welcome() {
    println!();
    println!();
    println!("Welcome to my Rust Command Line Prompt!"); 
}
fn short_help() {
    println!("{}", "Commands:".yellow());
    println!("echo: echo [...]");
    println!("cat: cat [filename]");
    println!("exit: exit the prompt");
    println!("ls: lists directory")

}


fn main() {

    welcome();
    short_help();

    loop {
        
        let mut input: String = String::new();
        input.clear();
        print!("{}", "$ rsp ".magenta());


        io::stdout().flush().expect("Cannot flush stdout");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");


        let command: Command = parse_command_input(&input);
        let result: ExecutionResult = execute_command(&command);

        match result {
            ExecutionResult::Success => println!("Command executed successfully"),
            ExecutionResult::Failure => println!("Command execution failed"),
        }


    }
}


fn exec_echo(args: Vec<String>) -> Result<String, Error> {
    let echo: String = args.join(" ");

    print!("{}", echo);
    Ok(echo)
}

#[cfg(test)]
mod echo_tests {
    use super::exec_echo;
    #[test]
    fn test_exec_echo() {
        let args: Vec<String> = vec!["Hello".to_string(), "World!".to_string(), 911.to_string(), "💖".to_string()];


        match exec_echo(args) {
            Ok(result) => assert_eq!("Hello World! 911 💖", result),
            Err(err) => panic!("Error: {}", err),
        }
    }
}

fn cat_command(args: Vec<String>) -> io::Result<String> {

    let os_string: OsString = match args.first() {
        Some(first_arg) => OsString::from(first_arg),
        None => OsString::new(), // or any default value you want to use
    };


    let path: &Path = Path::new(&os_string);
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
        let file_path = "test.txt";
        let contents = "This file exists for testing my rust cat command";
        fs::write(file_path, contents).expect("Failed to write test file");
        let result: Result<String, io::Error> = cat_command(vec![file_path.to_string()]);

        match result {
            Ok(result) => assert_eq!(result, contents),
            Err(err) => panic!("Error: {}", err),
        }


        
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_cat_nonexistent_file() {
        let result: Result<String, io::Error> = cat_command(vec!["nonexistent_file.txt".to_string()]);
        assert!(result.is_err());
    }
}
// todo: Refactor to ./commands/ls.rs
fn ls () {
    let paths: ReadDir = fs::read_dir("./").unwrap();

    for path in paths {

        let unwrapped_path: PathBuf = path.unwrap().path();

        let is_file: bool = unwrapped_path.is_file();
        let is_dir: bool = unwrapped_path.is_dir();
        let display_path: String = unwrapped_path.display().to_string();

        if is_file == true {
            print!( "{} ", display_path.bold().green() );
        } 

        if is_dir == true {
            print!( "{} ", display_path.green());
        }
    }
}

#[cfg(test)]
mod ls_tests {
    use super::ls;

    #[test]
    fn test_can_ls() {
        ls()
    }

    // todo: Add more tests with mocks
}
