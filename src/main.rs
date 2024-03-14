use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    match args.len()  {
        1 => {
            println!("No arguments provided. try --help for help");
            return;
        },
        _ => {},
    }

    let first_arg: String = args.nth(1).unwrap();
    let skipped_ags = args.skip(0);
    let mut result: String = "".to_string();
    match first_arg.as_str()  {
        "echo" => {
            result= exec_echo(skipped_ags);
        },
        _ => println!("invalid string"),
    }

    print!("Console: {}", result)
}

#[test]
fn test_exec_echo() {
    let vec: Vec<String> = vec!["Hello".to_owned(), "World!".to_owned()];

    let mut iter: std::slice::Iter<'_, String> = vec.iter();
    assert_eq!("Hello World!", exec_echo(iter))
}


fn exec_echo(skipped_args: impl Iterator <Item = String>) -> String {

    let mut echo_string: String = "".to_owned();


    for arg in skipped_args  {

        echo_string.push_str(&arg.to_owned())
        
    }

    return echo_string;

}