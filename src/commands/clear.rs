use std::io::Error;

pub fn execute(args: Vec<String>) -> Result<String, Error> {

    let _ = args;

    let clear: String = format!("\x1B[2J\x1B[1;1H");

    print!("{clear}");

    Ok(clear)
}