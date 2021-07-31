# Rust Command Line Argument Parser

Parse command line arguments from raw strings into specific enum types.

Good for building basic CLI programs or CLI frameworks in rust.
___

# Examples

```rust
let arg_strings = vec!["/file/path".to_string(), "-ab".to_string(), "--myLongOption=value".to_string()];
let config = command_line_argument_parser::Config::new(&arg_strings).unwrap();
assert_eq!(config.options, vec![
    command_line_argument_parser::Arg::ShortOption(String::from("a")),
    command_line_argument_parser::Arg::ShortOption(String::from("b")),
    command_line_argument_parser::Arg::LongOption(String::from("myLongOption"), String::from("value")),
]);
```

Given this code:
```rust
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
```

If you ran:
```bash
./your_program -abc --key=value
```

You would see these values:
```sh
ShortOption("a")
ShortOption("b")
ShortOption("c")
LongOption("key", "value")
```


