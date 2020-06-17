/// This crate provides structs, traits, and functions for producing initial conditions based on
/// JSON configurations, represented using the structs and traits defined in data_structure and
/// serde_json.
extern crate data_structure;
extern crate serde_json;
pub mod circle;
use std::error::Error;

const GENERATOR_NAME_LABEL: &str = "generatorName";
const GENERATOR_CONFIGURATION_LABEL: &str = "generatorConfiguration";
const HORIZONTAL_LABEL: &str = "x";
const VERTICAL_LABEL: &str = "y";

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

#[derive(Debug)]
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
    let generator_configuration =
        match deserialized_configuration.get(GENERATOR_CONFIGURATION_LABEL) {
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

pub fn parse_f64(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<f64, Box<dyn std::error::Error>> {
    match given_configuration[attribute_label].as_f64() {
        Some(parsed_number) => Ok(parsed_number),
        _ => Err(Box::new(ConfigurationParseError::new(&format!(
            "Could not parse \"{}\" from {}",
            attribute_label, given_configuration
        )))),
    }
}

pub fn parse_i64(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<i64, Box<dyn std::error::Error>> {
    match given_configuration[attribute_label].as_i64() {
        Some(parsed_number) => Ok(parsed_number),
        _ => Err(Box::new(ConfigurationParseError::new(&format!(
            "Could not parse \"{}\" from {}",
            attribute_label, given_configuration
        )))),
    }
}

pub fn parse_position(
    given_position: &serde_json::Value,
) -> Result<data_structure::PositionVector, Box<dyn std::error::Error>> {
    let horizontal_position = parse_f64(HORIZONTAL_LABEL, given_position)?;
    let vertical_position = parse_f64(VERTICAL_LABEL, given_position)?;
    Ok(data_structure::PositionVector {
        horizontal_position: data_structure::HorizontalPositionUnit(horizontal_position),
        vertical_position: data_structure::VerticalPositionUnit(vertical_position),
    })
}

pub fn parse_velocity(
    given_position: &serde_json::Value,
) -> Result<data_structure::VelocityVector, Box<dyn std::error::Error>> {
    let horizontal_velocity = parse_f64(HORIZONTAL_LABEL, given_position)?;
    let vertical_velocity = parse_f64(VERTICAL_LABEL, given_position)?;
    Ok(data_structure::VelocityVector {
        horizontal_velocity: data_structure::HorizontalVelocityUnit(horizontal_velocity),
        vertical_velocity: data_structure::VerticalVelocityUnit(vertical_velocity),
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
            GENERATOR_NAME_LABEL: [],
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
            GENERATOR_NAME_LABEL: "acceptable",
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
        let expected_name = "acceptable";
        let expected_configuration = serde_json::json!({
            "internalNumber": 9001,
            "internalStringArray": ["we're", "the", "kids", "in", "America"],
        });
        let valid_configuration = serde_json::json!({
            GENERATOR_NAME_LABEL: expected_name,
            GENERATOR_CONFIGURATION_LABEL: expected_configuration,
        });
        let parsing_result = parse_deserialized_configuration(&valid_configuration)
            .expect("Should parse valid JSON object");
        if (parsing_result.generator_name == expected_name)
            && (parsing_result.generator_configuration == &expected_configuration)
        {
            Ok(())
        } else {
            Err(String::from(format!(
                "Expected name = {}, configuration = {}, actually parsed {:?}",
                expected_name, expected_configuration, parsing_result
            )))
        }
    }
}
