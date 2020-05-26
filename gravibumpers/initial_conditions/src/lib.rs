extern crate data_structure;
extern crate serde_json;
pub mod circle;
use std::error::Error;

const GENERATOR_NAME_LABEL: &str = "generatorName";
const GENERATOR_CONFIGURATION_LABEL: &str = "generatorName";

#[derive(Debug)]
pub struct ConfigurationParseError {
    error_message: String,
}

impl ConfigurationParseError {
    pub fn new(error_message: &str) -> ConfigurationParseError {
        ConfigurationParseError {
            error_message: error_message.to_string(),
        }
    }
}

impl Error for ConfigurationParseError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl std::fmt::Display for ConfigurationParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing configuration: {}", self.error_message)
    }
}

pub struct ParsedConfiguration<'a> {
    pub generator_name: &'a str,
    pub generator_configuration: &'a serde_json::Value,
}

pub fn parse_deserialized_configuration<'a>(
    deserialized_configuration: &'a serde_json::Value,
) -> Result<ParsedConfiguration, Box<dyn std::error::Error>> {
    let generator_name = match deserialized_configuration[GENERATOR_NAME_LABEL].as_str() {
        Some(parsed_string) => parsed_string,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                GENERATOR_NAME_LABEL, deserialized_configuration
            ))))
        }
    };
    let generator_configuration = match deserialized_configuration.get("configuration") {
        Some(parsed_value) => parsed_value,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                GENERATOR_CONFIGURATION_LABEL, deserialized_configuration
            ))))
        }
    };
    Ok(ParsedConfiguration {
        generator_name: generator_name,
        generator_configuration: generator_configuration,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_no_generator_name() -> Result<(), String> {
        let nameless_configuration = serde_json::json!({
            "gneratrNmae": "typo",
            GENERATOR_CONFIGURATION_LABEL: {
                "internalNumber": 9001,
                "internalStringArray": ["we're", "the", "kids", "in", "America"]
            }
        });
        let parsing_result = parse_deserialized_configuration(&nameless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_generator_name() -> Result<(), String> {
        let nameless_configuration = serde_json::json!({
            "generatorName": [],
            GENERATOR_CONFIGURATION_LABEL: {
                "internalNumber": 9001,
                "internalStringArray": ["we're", "the", "kids", "in", "America"]
            }
        });
        let parsing_result = parse_deserialized_configuration(&nameless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_no_generator_configuration() -> Result<(), String> {
        let configurationless_configuration = serde_json::json!({
            "generatorName": "acceptable",
            format!("{}{}", GENERATOR_CONFIGURATION_LABEL, "x"): {
                "internalNumber": 9001,
                "internalStringArray": ["we're", "the", "kids", "in", "America"]
            }
        });
        let parsing_result = parse_deserialized_configuration(&configurationless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_parse_valid_configuration() -> Result<(), String> {
        Err(String::from("not implemented"))
    }
}
