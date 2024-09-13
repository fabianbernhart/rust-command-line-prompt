use std::fmt::Display;
use std::io::{self, Error, Write};
use std::{ffi::OsString, fs::File, path::Path};

pub fn usage() -> impl Display {
    use colored::Colorize as _;
    let usage: colored::ColoredString = "Usage:".green().bold();

    format!("\n{usage} cat [OPTION]... [FILE]...\n")
}

pub fn execute(args: Vec<String>, mut io: impl Write) -> io::Result<()> {
    let os_string: OsString = match args.first() {
        Some(first_arg) => OsString::from(first_arg),
        None => return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid Input")),
    };
    let path: &Path = Path::new(&os_string);

    let mut file: File = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    io::copy(&mut file, &mut io)?;

    Ok(())
}

#[cfg(test)]
mod cat_tests {
    use std::{fs, vec};

    use io::Cursor;

    use super::*;

    #[test]
    fn test_cat_existing_file() {
        let file_path = "test.txt";
        let contents = "This file exists for testing my rust cat command";

        let mut buf: Vec<u8> = Vec::new();
        let cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut buf);
        let args: Vec<String> = vec![file_path.to_string()];

        fs::write(file_path, contents).expect("Failed to write test file");

        let result = execute(args, cursor);
        let output: String = String::from_utf8(buf).unwrap();

        assert!(result.is_ok());
        assert_eq!(output, contents);

        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_cat_nonexistent_file() {
        let nonexistent_file_path = "nonexistent_file.txt";

        let mut buf = Vec::new();
        let cursor = Cursor::new(&mut buf);

        let args: Vec<String> = vec![nonexistent_file_path.to_string()];

        let result = execute(args, cursor);

        assert!(result.is_err());
    }
}
