use std::fmt::Error;

pub fn execute(args: Vec<String>) -> Result<String, Error> {
    let echo: String = args.join(" ");

    println!("{}", echo);
    Ok(echo)
}


#[cfg(test)]
mod echo_tests {
    use super::execute;
    #[test]
    fn test_exec_echo() {
        let args: Vec<String> = vec!["Hello".to_string(), "World!".to_string(), 911.to_string(), "💖".to_string()];


        match execute(args) {
            Ok(result) => assert_eq!("Hello World! 911 💖", result),
            Err(err) => panic!("Error: {}", err),
        }
    }
}