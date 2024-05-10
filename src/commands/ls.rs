use std::fs::{DirEntry, ReadDir};
use std::fs;

use std::io;
use std::path::PathBuf;

use colored::Colorize;

pub fn execute() -> Result<(), io::Error> {
    let paths: ReadDir = fs::read_dir("./").unwrap();

    for path in paths {
        let unwrapped_path: DirEntry = path?;
        let unwrapped_path_buf: PathBuf = unwrapped_path.path();

        let is_file: bool = unwrapped_path_buf.is_file();
        let is_dir: bool = unwrapped_path_buf.is_dir();
        let display_path: String = unwrapped_path_buf.display().to_string();

        if is_file == true {
            print!( "{} ", display_path.bold().green() );
        } 

        if is_dir == true {
            print!( "{} ", display_path.green());
        }
    }
    Ok(())
}

#[cfg(test)]
mod ls_tests {
    use super::execute;

    #[test]
    fn test_can_ls() {
        match execute() {
            Ok(_) => assert_eq!(true, true),
            Err(err) => panic!("Error: {}", err),
        }
    }
}