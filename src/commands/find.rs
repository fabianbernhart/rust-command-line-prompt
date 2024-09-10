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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::env;

    #[test]
    fn test_list_entries_single_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let entries = list_entries(temp_dir.path().to_path_buf()).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries.contains(&file_path));
    }

    #[test]
    fn test_list_entries_nested_dirs() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nested_dir = temp_dir.path().join("nested");
        let file_path = nested_dir.join("test_file.txt");

        fs::create_dir(&nested_dir).unwrap();
        File::create(&file_path).unwrap();

        let entries = list_entries(temp_dir.path().to_path_buf()).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&nested_dir));
        assert!(entries.contains(&file_path));
    }

    #[test]
    fn test_directory_entries_new() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let dir_entries = DirectoryEntries::new(temp_dir.path().to_path_buf()).unwrap();
        assert_eq!(dir_entries.entries.len(), 1);
        assert!(dir_entries.entries.contains(&file_path));
    }

    #[test]
    fn test_execute_no_args() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let _orig_dir = env::set_current_dir(&temp_dir);

        let result = execute(vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_args() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let result = execute(vec![temp_dir.path().to_str().unwrap().to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_empty_arg() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let _orig_dir = env::set_current_dir(&temp_dir);

        let result = execute(vec!["".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_directory_entries() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let dir_entries = DirectoryEntries::new(temp_dir.path().to_path_buf()).unwrap();
        let mut output = String::new();

        use std::fmt::Write;
        write!(output, "{}", dir_entries).unwrap();

        assert!(output.contains(file_path.to_str().unwrap()));
    }
}