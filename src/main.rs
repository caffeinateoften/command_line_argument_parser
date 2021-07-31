use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = command_line_argument_parser::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    for option in config.options {
        println!("{:?}", option);
    }
}
