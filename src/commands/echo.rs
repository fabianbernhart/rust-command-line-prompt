use std::io::{self, Write};

pub fn execute(args: Vec<String>, mut io: impl Write) -> io::Result<()> {
    let echo: String = args.join(" ");

    io.write_all(echo.as_bytes())?;
    io.write_all(b"\n")?;

    Ok(())
}


#[cfg(test)]
mod echo_tests {
    use super::execute;
    #[test]
    fn test_echo_success() {
        let args: Vec<String> = vec!["Hello".to_string(), "World!".to_string(), 911.to_string()];
        let expect = b"Hello World! 911\n";
        let mut output = Vec::new();

        execute(args, &mut output).unwrap();
        assert_eq!(output, expect);
    }
    #[test]
    fn test_echo_no_arguments() {
        let args: Vec<String> = vec![];
        let mut output = Vec::new();

        execute(args, &mut output).unwrap();
        assert_eq!(output, b"\n");
    }
    #[test]
    fn test_echo_with_emoji() {
        let args = vec!["Hello".to_string(), "🌍".to_string(), "world!".to_string()];
        let mut output = Vec::new();

        execute(args, &mut output).unwrap();
        assert_eq!(output, "Hello 🌍 world!\n".as_bytes());
    }
    #[test]
    fn test_echo_with_multiple_emojis() {
        let args = vec!["🎉".to_string(), "🚀".to_string(), "💬".to_string()];
        let mut output = Vec::new();

        execute(args, &mut output).unwrap();
        assert_eq!(output, "🎉 🚀 💬\n".as_bytes());
    }
}