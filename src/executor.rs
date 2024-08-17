use crate::commands;

use crate::{Command, ExecutionResult};

pub fn execute_command(command: &Command) -> ExecutionResult {

    let args: Vec<String> = command.args.clone();   

    match command.name.as_str() {
        "echo" => {
            match  commands::echo::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("{error}");
                    ExecutionResult::Failure},
            }
        },
        "ls" => {
            match commands::ls::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("{error}");
                    ExecutionResult::Failure},
            }
        },
        "cat" => {
            match commands::cat::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("{error}");
                    ExecutionResult::Failure},
            }
        },
        "clear" => {
            match commands::clear::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(_) => {
                    println!("");
                    ExecutionResult::Failure},
            }
        }
        "find" => {
            match commands::find::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    println!("{error}");
                    ExecutionResult::Failure},
            }
        },
        "help" | "h" => {
            commands::helpers::help();
            ExecutionResult::Success
        },
        _ => {


            println!("{}: command not found", command.name);

            commands::helpers::help();

            
            ExecutionResult::Failure
        }
    }
}