#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Arg {
    LongOption(String, String),
    ShortOption(String),
}
/**
 * For this to be POSIX compliant, Arg would need
 *   1.) to have NON_OPTION(String) such that each arg after "--" are assumed NON_OPTIONS
 *   2.) for SHORT_OPTION(String) to become SHORT_OPTION(String, Option<String>)
 */

pub struct Config {
    pub options: Vec<Arg>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() == 1 {
            return Ok(Config {
                options: vec![]
            })
        }
        let maybe_args: Result<Vec<_>, _> = args
            .into_iter()
            .skip(1)
            .map(|s|Self::create_args(&s))
            .collect();

        if let Err(e) = maybe_args {
            return Err(e);
        }

        let options = maybe_args.unwrap()
            .into_iter()
            .flatten()
            .collect();

        Ok(Config {
            options: options
        })
    }

    fn create_args(s: &str) -> Result<Vec<Arg>, String> {
        // "-a[bc..]" or "--key=value"
        lazy_static! {
            static ref SHORT_OPTION_RE: Regex = Regex::new(r"^-(\w+)$").unwrap();
            static ref LONG_OPTION_RE: Regex = Regex::new(r"^--(\w+)=(\w+)$").unwrap();
        }

        let is_short_option = SHORT_OPTION_RE.is_match(s);
        let is_long_option = LONG_OPTION_RE.is_match(s);
        let e_msg = Self::err_msg("Something went wrong when trying to derive option arg(s).",s);

        let mut derived_args: Vec<Arg> = Vec::new();

        if !is_short_option && !is_long_option { // guard against invalid patterns
            return Err(e_msg);
        }
        else if is_short_option { // example without using regex capture group
            for c in s.chars().skip(1) {
                derived_args.push(Arg::ShortOption(c.to_string()));
            }
            if derived_args.is_empty() {
                return Err(e_msg);
            }
            return Ok(derived_args);
        }
        else /* if long_option_pattern.is_match(s) */ { // example with regex capture group
            let caps = LONG_OPTION_RE.captures(s).ok_or(e_msg)?;
            let key = caps.get(1).unwrap().as_str().to_string();
            let value = caps.get(2).unwrap().as_str().to_string();
            derived_args.push(Arg::LongOption(key, value));
            return Ok(derived_args);
        }
    }

    fn err_msg(msg_prefix: &str, arg_value: &str) -> String {
        return format!("{} {} {}", msg_prefix, "Expecting arg in format of \"--key=value\" or \"-o\". Received:", arg_value)
    }
}

#[cfg(test)]
mod tests {
    use std::{process};

    use super::*;

    #[test]
    fn config_options_will_contain_appropriate_arg_types() {
        // input
        let mock_args = vec!["-ab".to_string(), "--myLongOption=value".to_string()];

        // expected output
        let expected_derived_args = vec![
            Arg::ShortOption(String::from("a")),
            Arg::ShortOption(String::from("b")),
            Arg::LongOption(String::from("myLongOption"), String::from("value")),
        ];

        // test
        let config = Config::new(mock_args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        assert_eq!(expected_derived_args.iter().eq(config.options.iter()), true);
    }
}
