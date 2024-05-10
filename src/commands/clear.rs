pub fn execute(args: Vec<String>) -> Result<(), ()> {

    let _ = args;

    print!("\x1B[2J\x1B[1;1H");
    Ok(())
}