use std::{env};
use command_line_interface::{Config, CommandLineInterface, CommandRequest};
fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    let cli_config = Config::new(vec![
        ('a', false),
        ('b', true),
        ('c', false),
        ('d', false),
        ('e', false),
        ('f', false)
    ])?;

    let cli = CommandLineInterface::new(cli_config)?;
    let command_request = CommandRequest::new(&args)?;

    for option in command_request.options {
        println!("{:?}", option);
    }

    Ok(())
}
