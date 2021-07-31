#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Arg {
    LongOption(String, String),
    ShortOption(String), // TODO: support short option values with ShortOption(String, Option<String>),
    // TODO: support NonOption(String)
}

pub struct Config {
    pub options: Vec<Arg>,
}

impl Config {
    /// creates Config struct with derived Arg kinds from vector of strings
    ///
    /// # Examples
    /// ```
    /// let arg_strings = vec!["/file/path".to_string(), "-ab".to_string(), "--myLongOption=value".to_string()];
    /// let config = command_line_argument_parser::Config::new(&arg_strings).unwrap();
    /// assert_eq!(config.options, vec![
    ///     command_line_argument_parser::Arg::ShortOption(String::from("a")),
    ///     command_line_argument_parser::Arg::ShortOption(String::from("b")),
    ///     command_line_argument_parser::Arg::LongOption(String::from("myLongOption"), String::from("value")),
    /// ]);
    ///```
    pub fn new(args: &Vec<String>) -> Result<Config, String> {
        match Self::create_args(args) {
            Ok(args) => Ok(Config {
                options: args
            }), 
            Err(e) => Err(e)
        }
    }

    /// Creates a list of Arg variants according to kind of argument detected
    fn create_args(args: &Vec<String>) -> Result<Vec<Arg>, String> {
        if args.len() == 1 {
            return Ok(vec![])
        }

        lazy_static! {
            // only alphanumeric characters
            // TODO: Replace regex usage w/ loops
            static ref SHORT_OPTION_RE: Regex = Regex::new(r"^-([a-zA-Z0-9]+)$").unwrap();
            static ref LONG_OPTION_RE: Regex = Regex::new(r"^--([a-zA-Z0-9]+)=([a-zA-Z0-9]+)$").unwrap();
            static ref NON_OPTION_RE: Regex = Regex::new(r"^([a-zA-Z0-9]+)$").unwrap();
        }

        let mut parsed_args: Vec<Arg> = Vec::new();
        let e_msg = "Something went wrong when trying to derive arg(s).";

        for s in args.into_iter().skip(1) {
            let is_short_option = SHORT_OPTION_RE.is_match(s);
            let is_long_option = LONG_OPTION_RE.is_match(s);
            let is_option_terminator = s == "--";
            if !is_short_option && !is_long_option && !is_option_terminator { // guard against invalid patterns
                return Err(e_msg.to_string());
            }
            else if is_short_option {             // example without using regex capture group
                for c in s.chars().skip(1) { // "-ab" skip the dash
                parsed_args.push(Arg::ShortOption(c.to_string()));
                }
            }
            else if is_long_option { // example with regex capture group
                let caps = LONG_OPTION_RE.captures(s).ok_or(e_msg)?;
                let key = caps.get(1).unwrap().as_str().to_string();
                let value = caps.get(2).unwrap().as_str().to_string();
                parsed_args.push(Arg::LongOption(key, value));
            }
        }


        if parsed_args.is_empty() {
            return Err(e_msg.to_string());
        }
        else {
            return Ok(parsed_args);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{process};

    use super::*;

    #[test]
    fn config_options_will_contain_appropriate_arg_types() {
        // input
        let arg_strings = vec!["/file/path".to_string(), "-ab".to_string(), "--myLongOption=value".to_string()];

        // test
        let config = Config::new(&arg_strings).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        assert_eq!(config.options, vec![
            Arg::ShortOption(String::from("a")),
            Arg::ShortOption(String::from("b")),
            Arg::LongOption(String::from("myLongOption"), String::from("value")),
        ])
    }
}
