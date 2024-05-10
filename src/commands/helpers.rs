use colored::Colorize;

pub fn welcome() {
    println!();
    println!();
    println!("Welcome to my Rust Command Line Prompt!"); 
}
pub fn short_help() {
    println!("{}", "Commands:".yellow());
    println!("echo: echo [...]");
    println!("cat: cat [filename]");
    println!("exit: exit the prompt");
    println!("ls: lists directory")

}