use core::fmt;

use std::fs;

use std::ffi::OsString;
use std::path::PathBuf;
use std::io::{self, Result};

struct DirectoryEntries {
    entries: Vec<PathBuf>,
}

impl DirectoryEntries {
    fn new(path: PathBuf) -> Result<Self> {
        let entries: Vec<PathBuf> = list_entries(path)?;
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

    print!("{}", dir_entries);

    Ok(())

}

fn list_entries(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry: fs::DirEntry = entry?;
        let entry_path: PathBuf = entry.path();

        entries.push(entry_path.clone());
        if entry_path.is_dir() {
            let sub_entries: Vec<PathBuf> = list_entries(entry_path)?;
            entries.extend(sub_entries)
        }
    }

    Ok(entries)
}

impl fmt::Display for DirectoryEntries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries {
            writeln!(f, "{}", entry.display())?;
        }
        Ok(())
    }
}
