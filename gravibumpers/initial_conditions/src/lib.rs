extern crate data_structure;
extern crate serde_json;
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

    if let serde_json::Value::String(configuration_type) = &json_value["type"] {
        generate_from_type_and_configuration(&configuration_type, &json_value["configuration"])
    } else {
        Err(Box::new(ConfigurationParseError::new(&format!(
            "Type \"{}\" is not a known type of configuration",
            json_value["type"]
        ))))
    }
}

pub fn generate_from_type_and_configuration(
    configuration_type: &str,
    configuration_body: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    return match configuration_type {
        "circle" => generate_circle(configuration_body),
        _ => Err(Box::new(ConfigurationParseError::new(&format!(
            "Type \"{}\" is not a known type of configuration",
            configuration_type
        )))),
    };
}

fn generate_circle(
    given_configuration: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    Err(Box::new(ConfigurationParseError::new(&format!(
        "Not yet implemented"
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_generate_circle() {}
}
