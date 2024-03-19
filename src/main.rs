use std::env::args;

fn main() {
    let vec_args: Vec<String> = args().collect();

    match vec_args.len() {
        1 => {
            help(vec_args);
            return;
        },
        _ => {}
    }


    println!("{}", vec_args.join(" "));
    

    if vec_args[0] == "echo" {
        let result: String = exec_echo(vec_args);
        println!("{}", result)
    }
}

fn help(args: Vec<String>) {

    if args[0] == "echo" {
        println!("NAME");
        println!("%5s\nECHO: echo [...]");
        
        println!("%5s\n EXAMPLE: echo [...]");
    }
}


#[test]
fn test_exec_echo() {
    let args: Vec<String> = vec!["echo".to_string(), "Hello".to_string(), "World!".to_string(), 911.to_string(), "💖".to_string()];
    assert_eq!("Hello World! 911 💖", exec_echo(args));
}


fn exec_echo(mut args: Vec<String>) -> String {
    args.remove(0);

    let echo_string = args.join(" ");
    return echo_string.to_string();
}