use std::{ffi::OsString, fs::File, io::Read, path::Path};
use std::io;

pub fn execute(args: Vec<String>) -> io::Result<String> {
    let os_string: OsString = match args.first() {
        Some(first_arg) => OsString::from(first_arg),
        None => OsString::new(), // or any default value you want to use
    };

    let path: &Path = Path::new(&os_string);
    let mut file: File = File::open(path)?;
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)?;
    print!("{}", contents);

    Ok(contents)
}

#[cfg(test)]
mod cat_tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_cat_existing_file() {
        let file_path = "test.txt";
        let contents = "This file exists for testing my rust cat command";
        fs::write(file_path, contents).expect("Failed to write test file");
        let result: Result<String, io::Error> = execute(vec![file_path.to_string()]);

        match result {
            Ok(result) => assert_eq!(result, contents),
            Err(err) => panic!("Error: {}", err),
        }
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_cat_nonexistent_file() {
        let result: Result<String, io::Error> = execute(vec!["nonexistent_file.txt".to_string()]);
        assert!(result.is_err());
    }
}