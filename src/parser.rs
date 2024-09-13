use crate::Command;

use std::str::SplitWhitespace;

pub fn parse_keyboard_input(input: AsRef<String>) {
    
}


pub fn parse_command_input (input: &str) -> Command {
    let mut parts: SplitWhitespace = input.trim().split_whitespace();

    let name = match parts.next() {
        Some(first_part) => first_part.to_string(),
        None => "".to_string(),
    };

    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    let command: Command = Command {name, args};

    command
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    fn test_parse_command_input(input: &str, expected: Command) {

        let test = parse_command_input(input);

        assert_eq!(test.name, expected.name);
        assert_eq!(test.args, expected.args);
    }

    #[test]
    fn test_parse_command_input_generic() {
        test_parse_command_input("", Command{ name: String::new(), args: vec![] });
        test_parse_command_input("cat", Command{ name: String::from("cat"), args: vec![] });
        test_parse_command_input(
            "echo Hello World!", 
        Command{
            name: String::from("echo"),
            args: vec![String::from("Hello"), String::from("World!")]
        }
        );
    }
}