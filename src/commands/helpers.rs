use colored::Colorize;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct CommandInfo {
    name: &'static str,
    alias: Option<&'static str>,
    description: &'static str,
}

impl fmt::Display for CommandInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(alias) = self.alias {
            write!(
                f,
                "  {}, {:<7} {}",
                self.name.cyan().bold(),
                alias.cyan().bold(),
                self.description
            )  
        } else {
            write!(
                f,
                "  {:<14}{}",
                self.name.cyan().bold(),
                self.description
            )
        }
    }
}

pub fn welcome() -> impl Display {
    format!("Welcome to my Rust Command Line Prompt!\n")
}

pub fn usage() -> impl Display {
    format!("{} {}", "Usage".green().bold(), "[COMMAND] [ARGS]\n" )
}


pub fn help() -> impl Display  {

    let welcome_format = welcome();
    let usasge_format = usage();

    let commands: Vec<CommandInfo> = vec![
        CommandInfo {
            name: "echo",
            description: "returns the innput args as output",
            alias: None,
        },
        CommandInfo {
            name: "cat",
            description: "displays the content of an file, cat",
            alias: None,
        },
        CommandInfo {
            name: "ls",
            description: "shows the list of files and folders in the directory",
            alias: None,
        },
        CommandInfo {
            name: "find",
            description: "shows the all the folders, files and subfolders/-files of the directory",
            alias: None,
        },
        CommandInfo {
            name: "help",
            description: "shows the list of help commands and usage of the tool",
            alias: Some("h"),
        },
        CommandInfo {
            name: "exit",
            description: "exits the command prompt",
            alias: None,
        },
    ];
    
    let commands_title = format!("{}", "Commands:".green().bold());

    let formatted_commands = commands.iter()
        .map(|command| format!("{}", command))
        .collect::<Vec<String>>()
        .join("\n");


    format!("{}\n{}\n{}\n{}\n", welcome_format, usasge_format, commands_title, formatted_commands )
}