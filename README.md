# Rust Command Line Interface

Good for building basic CLI programs or CLI frameworks in rust.
___

# Examples

```rust
    use std::{env};
    use command_line_interface::{CommandLineInterface};
    fn main() -> Result<(), String> {
        let args = env::args().collect::<Vec<String>>();
        
        let cli = CommandLineInterface::new(vec![
            ('a', false),
            ('b', true),
            ('c', false),
            ('d', false),
            ('e', false),
            ('f', false)
        ])?;
        let command_request = cli.create_command_request(&args)?;
        command_request.execute()?;

        Ok(())
    }
```
