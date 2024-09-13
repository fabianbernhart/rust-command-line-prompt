
mod parser;
mod executor;
mod commands;

use std::env;
use std::io;
use std::io::Write;

struct Command {
    name: String,
    args: Vec<String>
}

#[derive(PartialEq)]
pub enum ExecutionResult {
    Success,
    Failure(String, String),
}





fn main() {
    commands::helpers::help();

    loop {
        let current_path: colored::ColoredString = match env::current_dir() {
            Ok(result) => result.display().to_string().blue(),
            Err(err) => todo!("{}", err), 
        };

        let mut input: String = String::new();
        input.clear();
        print!("{} {} $ ", "rcm".magenta(), current_path);
        
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => println!("Error: {err}"),
        }

        match io::stdin().read_line(&mut input, ) {
            Ok(_) => (),
            Err(err) => println!("Error: {err}"),
        }

        let command: Command = parser::parse_command_input(&input);
        let result: ExecutionResult = executor::execute_command(&command);

        use colored::Colorize as _;
        let error: colored::ColoredString = "Error:".red().bold();

        match result {
            ExecutionResult::Success => (),
            ExecutionResult::Failure(err_msg, usage_fn) => {
                println!("{error} {}", err_msg);
                println!("{}", usage_fn);
            }
        }



    }
}