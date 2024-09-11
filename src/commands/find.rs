use std::fs;

use std::path::{Path, PathBuf};
use std::io::{self, Result, Write};

pub fn execute(args: Vec<String>, mut io: impl Write) -> io::Result<()> {
    let first_arg: &String = match args.first() {
        Some(string) => string,
        None => &String::from(".")
    };

    let dir_entries: Vec<PathBuf> = list_entries(first_arg)?;

    for dir_entry in dir_entries {

        let path_str = dir_entry.to_string_lossy();

        io.write_all(path_str.as_bytes())?;
        io.write_all(b"\n")?;

    }
    Ok(())

}

fn list_entries(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
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


    #[ignore = "is not good tested"]
    #[test]
    fn test_execute_no_args() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let mut output = Vec::new();
        let _orig_dir = env::set_current_dir(&temp_dir);

        let result = execute(vec![], &mut output);
        assert!(result.is_ok());
    }
    #[ignore = "is not good tested"]
    #[test]
    fn test_execute_with_args() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let mut output = Vec::new();

        let result = execute(vec![temp_dir.path().to_str().unwrap().to_string()], &mut output);
        assert!(result.is_ok());
    }
    #[ignore = "is not good tested"]
    #[test]
    fn test_execute_with_empty_arg() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let mut output = Vec::new();
        let _orig_dir = env::set_current_dir(&temp_dir);

        let result = execute(vec!["".to_string()], &mut output);
        assert!(result.is_ok());
    }
}