use std::{env, process};

fn main() {
    let config = command_line_argument_parser::Config::new(env::args().collect::<Vec<_>>()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    for option in config.options {
        println!("{:?}", option);
    }
}
