
mod parser;
mod executor;
mod commands;

use colored::Colorize;
use std::io::{self, Write};

struct Command {
    name: String,
    args: Vec<String>
}

#[derive(PartialEq)]
pub enum ExecutionResult {
    Success,
    Failure,
}

fn main() {
    commands::helpers::welcome();
    commands::helpers::short_help();

    loop {
        
        let mut input: String = String::new();
        input.clear();
        print!("{}", "rust$ ".magenta());
        
        io::stdout().flush().expect("Cannot flush stdout");

        match std::io::stdin().read_line(&mut input, ) {
            Ok(_input) => (),
            Err(error) => println!("error: {error}"),
        }

        let command: Command = parser::parse_command_input(&input);
        let result: ExecutionResult = executor::execute_command(&command);

        match result {
            ExecutionResult::Failure => (),
            ExecutionResult::Success => (),
        }
    }
}