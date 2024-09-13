use core::fmt;
use std::fs;
use std::io;
use std::ffi::OsString;
use std::path::PathBuf;
use std::io::Result;

use colored::Colorize;

struct DirectoryEntries {
    entries: Vec<PathBuf>,
}

impl DirectoryEntries {
    fn new(path: PathBuf) -> Result<Self> {
        let mut entries: Vec<PathBuf> = list_entries(path)?;
        entries.sort_by_key(|entry| if entry.is_dir() { 1 } else { 0 });
        Ok(DirectoryEntries { entries })
    }
}

pub fn execute(args: Vec<String>) -> io::Result<()> {
    let os_string: OsString = match args.first() {
        Some(first_arg) => OsString::from(first_arg),
        None => OsString::new(),
    };

    let default_path: String = String::from(".");
    let path: String = match os_string.to_str() {
        Some(str) => {
            if str.len() >= 1 {
                String::from(str)
            } else {
                default_path
            }
        }
        None => {
            default_path
        }
    };

    
    let dir_entries: DirectoryEntries = DirectoryEntries::new(path.into())?;

    println!("{}", dir_entries);

    Ok(())

}


fn list_entries(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
        let path: PathBuf = entry?.path();
        entries.push(path.clone());
    }
    

    Ok(entries)
}

impl fmt::Display for DirectoryEntries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries {
            let path: &std::path::Path = entry.as_path();
            
            let path_str: String = path.display().to_string();
            let stripped_path: &str = path_str.strip_prefix("./").unwrap_or(&path_str);

            if path.is_dir() {
                write!(f, "{} ", stripped_path.bold().green())?;
            } else if path.is_file() {
                write!(f, "{} ", stripped_path)?;
            }
        }
        Ok(())
    }
}