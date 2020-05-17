extern crate data_structure;
extern crate serde_json;
mod circle;
use std::error::Error;

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

pub fn generate_from_configuration_string(
    configuration_json: &str,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    let json_value: serde_json::Value = serde_json::from_str(configuration_json)?;
    let configuration_type = match json_value["type"].as_str() {
        Some(parsed_string) => parsed_string,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"type\" from {}",
                configuration_json
            ))))
        }
    };
    let configuration_body = match json_value.get("configuration") {
        Some(parsed_value) => parsed_value,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"configuration\" from {}",
                configuration_json
            ))))
        }
    };
    generate_from_type_and_configuration(&configuration_type, &configuration_body)
}

pub fn generate_from_type_and_configuration(
    configuration_type: &str,
    configuration_body: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    return match configuration_type {
        "circle" => circle::from_json(configuration_body),
        _ => Err(Box::new(ConfigurationParseError::new(&format!(
            "Type \"{}\" is not a known type of configuration",
            configuration_type
        )))),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_no_type() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_malformed_type() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_unknown_type() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_no_configuration() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_malformed_configuration() -> Result<(), String> {
        Err(String::from("not implemented"))
    }
}
