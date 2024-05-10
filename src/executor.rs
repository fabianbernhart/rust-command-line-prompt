use crate::commands;

use crate::{Command, ExecutionResult};

pub fn execute_command(command: &Command) -> ExecutionResult {

    let args: Vec<String> = command.args.clone();   

    match command.name.as_str() {
        "echo" => {
            match  commands::echo::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("ERROR: {error}");
                    ExecutionResult::Failure},
            }
        },
        "ls" => {
            match commands::ls::execute() {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("ERROR: {error}");
                    ExecutionResult::Failure},
            }
        },
        "cat" => {
            match commands::cat::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("ERROR: {error}");
                    ExecutionResult::Failure},
            }
        },
        "clear" => {
            match commands::clear::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(_) => {
                    println!("ERROR: Cant clear console.");
                    ExecutionResult::Failure},
            }
        }
        _ => {
            println!("Command not found: {}", command.name);
            ExecutionResult::Failure
        }
    }
}