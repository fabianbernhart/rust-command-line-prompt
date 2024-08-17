use colored::Colorize;
use std::fmt;

struct CommandInfo {
    name: &'static str,
    alias: Option<&'static str>,
    description: &'static str,
}

pub fn welcome() {
    println!("Welcome to my Rust Command Line Prompt!");

    println!()
}
pub fn help() {

    welcome();

    let usage: colored::ColoredString = "Usage".green().bold();


    println!("{usage} {}" , "[COMMAND] [ARGS]");

    println!("");

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

    println!("{}", "Commands:".green().bold());


    for command in commands {
        println!("{}", command);
    }

}

impl fmt::Display for CommandInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(alias) = self.alias {
            write!(f, "  {}, {:<7} {}", self.name.cyan().bold(), alias.cyan().bold(), self.description)  
        } else {
            write!(f, "  {:<14}{}", self.name.cyan().bold(), self.description)
        }
    }
}
