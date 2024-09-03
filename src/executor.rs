use std::io;

use crate::commands;

use crate::{Command, ExecutionResult};


pub fn no_usage() {
    use colored::Colorize as _;
    let usage: colored::ColoredString = "No usage provided".dimmed().white();


    println!("{usage}");
}

pub fn execute_command(command: &Command) -> ExecutionResult {

    let args: Vec<String> = command.args.clone();   

    match command.name.as_str() {
        "echo" => {
            match  commands::echo::execute(args) {
                Ok(_) => super::ExecutionResult::Success,
                Err(error) => {
                    let err_msg: String = error.to_string();
                    ExecutionResult::Failure(
                        err_msg, no_usage
                    )
                },
            }
        },
        "ls" => {
            match commands::ls::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    let err_msg: String = error.to_string();
                    ExecutionResult::Failure(
                        err_msg, no_usage
                    )
                },
            }
        },
        "cat" => {
            match commands::cat::execute(args, io::stdout()) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    let err_msg: String = format!("Error: {error}");
                    ExecutionResult::Failure(
                        err_msg, commands::cat::usage
                    )
                },
            }
        },
        "clear" => {
            match commands::clear::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(()) => {
                    let err_msg: String = format!("Cant Clear");
                    ExecutionResult::Failure(
                        err_msg, no_usage
                    )
                },
            }
        }
        "find" => {
            match commands::find::execute(args) {
                Ok(_) => ExecutionResult::Success,
                Err(error) => {
                    let err_msg: String = error.to_string();
                    ExecutionResult::Failure(
                        err_msg, no_usage
                    )
                },
            }
        },
        "help" | "h" => {
            commands::helpers::help();
            ExecutionResult::Success
        },
        _ => return {
            println!();
            let err_msg: String = format!(" command '{}'not found", command.name);
                    ExecutionResult::Failure(
                        err_msg, commands::helpers::help
                    )
        }
    }
}