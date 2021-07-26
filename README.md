# Rust Command Line Argument Parser

Parse command line arguments from raw strings into specific enum types.

Good for building basic CLI programs or CLI frameworks in rust.
___

# Examples

In your code:
```rust
fn main() {
    let raw_args = env::args().collect::<Vec<_>>();
    let config = command_line_argument_parser::Config::new(raw_args).unwrap_or_else(|err| {
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

You would get these enum debug values:
```sh
ShortOption("a")
ShortOption("b")
ShortOption("c")
LongOption("key", "value")
```

You use `config.options` to access these arg types and values.

**TODO to mature this from POC into publishable crate:**
- improve this Readme to show example of actual enum usage
- Add appropriate rust documentation to functions
- handle -- option terminator
- handle normal args
- Remove regex usage and parse chars in way where index of bad character is remembered so error diagnostic message can be very precise
- Make sure no bugs with ASCII vs UTF-8
- More tests
- Enable default parsing settings to be overridden by user (e.g. enable/disable short or long options or args, etc)

# How to recommend improvements?
Create a github issue that explains your ask.
I like Unix's Design Philosophy, so ideally different capabilties would live in different utilities: https://homepage.cs.uri.edu/~thenry/resources/unix_art/ch01s06.html

This crate should do nothing more than turn raw strings from input args into enum types and throw errors if input comes in unexpected form.

**Example improvement:**

To better abide by Rule of Exensibility, perhaps this utility could accept an Enum type from the consumer, as well as a pattern to match to it.