#[derive(Debug, PartialEq)]
pub enum ShortOption {
    WithoutArg(char),
    WithArg(char, String)
}

struct ShortOptionConfig {
    name: char,
    expect_arg: bool
}

impl ShortOptionConfig {
    pub fn new(name: char, expect_arg: bool) -> Result<ShortOptionConfig, String> {
        Ok(ShortOptionConfig{
            name,
            expect_arg
        })
    }
}

struct Config {
    options: Vec<ShortOptionConfig>
}

impl Config {
    pub fn new (config_inputs: Vec<(char, bool)>) -> Result<Config, String> {
        let tuple_to_short_option_config = |t: (char, bool)|ShortOptionConfig::new(t.0, t.1);
        let short_option_configs: Result<Vec<ShortOptionConfig>, _> = config_inputs
            .into_iter()
            .map(tuple_to_short_option_config)
            .collect();
        
        Ok(Config {
            options: short_option_configs?
        })
    }
}
pub struct CommandLineInterface {
    config: Config
}

impl CommandLineInterface {
    pub fn new(config_inputs: Vec<(char, bool)>) -> Result<Self, String> {
        let config = Config::new(config_inputs)?;
        Ok(CommandLineInterface {
            config
        })
    }

    pub fn create_command_request(&self, input_args: &Vec<String>)-> Result<CommandRequest, String>{
        todo!();
    }

    fn parse_short_options() -> Result<Vec<ShortOption>, String> {
        todo!();
    }
}

pub struct CommandRequest {
    options: Vec<ShortOption>
}

impl CommandRequest {
    pub fn execute(&self) -> Result<(), String>{
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_request_will_contain_expected_arg_types() -> Result<(), String> {
        
        let cli = CommandLineInterface::new(vec![
            ('a', false),
            ('b', true),
            ('c', false),
            ('d', false),
            ('e', false),
            ('f', false)
        ])?;

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
}
