use std::fmt::Display;
use std::io::{self};
use crate::{Command, ExecutionResult};




pub fn no_usage() -> impl Display {
    use colored::Colorize as _;
    let usage: colored::ColoredString = "No usage provided".dimmed().white();

    format!("\n{usage}")
}

pub fn execute_command(command: &Command) -> ExecutionResult {

    use crate::commands::{echo, ls, cat, clear, find, helpers};

    let args: Vec<String> = command.args.clone();

    fn handle_execution<T, E>(result: Result<T, E>, usage: String) -> ExecutionResult
    where
        E: ToString,
    {
        match result {
            Ok(_) => ExecutionResult::Success,
            Err(error) => ExecutionResult::Failure(error.to_string(), usage),
        }
    }

    let default_usage: String = no_usage().to_string();

    match command.name.as_str() {
        "echo" => handle_execution(echo::execute(args, io::stdout()), default_usage),
        "ls" => handle_execution(ls::execute(args), default_usage),
        "cat" => handle_execution(cat::execute(args, io::stdout()), cat::usage().to_string()),
        "find" => handle_execution(find::execute(args), default_usage),
        "clear" => handle_execution(clear::execute(args), default_usage),
        "help" | "h" => {
            println!("{}", helpers::help().to_string());
            ExecutionResult::Success
        },
        _ => return {
            let err_msg: String = format!(" command '{}'not found", command.name);
                    ExecutionResult::Failure(
                        err_msg, helpers::help().to_string()
                    )
        }
    }
}