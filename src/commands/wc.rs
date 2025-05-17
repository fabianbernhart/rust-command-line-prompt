use std::fmt::Display;
use std::fs;
use std::io::{self, Error, Write};
use std::{ffi::OsString, path::Path};

pub fn usage() -> impl Display {
    use colored::Colorize as _;
    let usage: colored::ColoredString = "Usage:".green().bold();

    format!("\n{usage} wc [OPTION]... [FILE]...\n")
}


pub fn execute(args: Vec<String>, mut io: impl Write) -> io::Result<()> {
    let os_string: OsString = match args.first() {
        Some(first_arg) => OsString::from(first_arg),
        None => return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid Input")),
    };
    let path: &Path = Path::new(&os_string);


    let data = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let word_count = data.split(" ").count();

    io.write_all(word_count.to_string().as_bytes())?;
    io.write_all(b"\n")?;

    Ok(())
}


