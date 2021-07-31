# Rust Command Line Argument Parser

Parse command line arguments from raw strings into specific enum types.

Good for building basic CLI programs or CLI frameworks in rust.
___

# Examples

```rust
        fn command_request_will_contain_expected_arg_types() -> Result<(), String> {

        let cli_config = Config::new(vec![
            ('a', false),
            ('b', true),
            ('c', false),
            ('d', false),
            ('e', false),
            ('f', false)
        ])?;

        let cli = CommandLineInterface::new(cli_config)?;

        let input_arg_strings: Vec<String> = vec!["/file/path", "-a", "-bc", "-def"].into_iter().map(String::from).collect();

        let command_request = cli.create_command_request(&input_arg_strings)?;

        assert_eq!(command_request.options, vec![
            ShortOption::WithoutArg('a'),
            ShortOption::WithArg('b', "c".to_string()),
            ShortOption::WithoutArg('d'),
            ShortOption::WithoutArg('e'),
            ShortOption::WithoutArg('f'),
        ]);

        Ok(())
    }
```
