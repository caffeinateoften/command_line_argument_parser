# Rust Command Line Interface

Good for building basic CLI programs or CLI frameworks in rust.
___

# Examples

```rust
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
        let command_request = cli.create_command_request(&args)?;
        command_request.execute()?;

        Ok(())
    }
```
