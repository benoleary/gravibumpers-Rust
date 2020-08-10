/// This crate provides structs, traits, and functions for producing initial conditions based on
/// JSON configurations, represented using the structs and traits defined in data_structure and
/// serde_json.
extern crate data_structure;
extern crate serde_json;
use std::convert::TryInto;
use std::error::Error;

pub const SECONDS_PER_MILLISECOND: f64 = 0.001;
const MEMORY_LAYOUT_LABEL: &str = "memoryLayout";
const NUMBER_OF_STEPS_PER_FRAME_LABEL: &str = "numberOfStepsPerFrame";
const DEAD_ZONE_RADIUS_LABEL: &str = "deadZoneRadius";
const INVERSE_SQUARED_COUPLING_LABEL: &str = "inverseSquaredCoupling";
const INVERSE_FOURTH_COUPLING_LABEL: &str = "inverseFourthCoupling";
const MILLISECONDS_PER_FRAME_LABEL: &str = "millisecondsPerFrame";
const NUMBER_OF_FRAMES_LABEL: &str = "numberOfFrames";
const RIGHT_BORDER_COORDINATE_LABEL: &str = "rightBorderCoordinate";
const UPPER_BORDER_COORDINATE_LABEL: &str = "upperBorderCoordinate";
const LEFT_BORDER_COORDINATE_LABEL: &str = "leftBorderCoordinate";
const LOWER_BORDER_COORDINATE_LABEL: &str = "lowerBorderCoordinate";
const GENERATOR_CONFIGURATIONS_LABEL: &str = "generatorConfigurations";
const GENERATOR_NAME_LABEL: &str = "generatorName";
const GENERATOR_CONFIGURATION_LABEL: &str = "generatorConfiguration";

#[derive(Debug)]
pub struct ConfigurationParseError {
    error_message: String,
}

impl ConfigurationParseError {
    pub fn new(error_message: &str) -> Self {
        Self {
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

pub fn parse_str<'a>(
    attribute_label: &str,
    given_configuration: &'a serde_json::Value,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    match given_configuration[attribute_label].as_str() {
        Some(parsed_string) => Ok(parsed_string),
        _ => Err(Box::new(ConfigurationParseError::new(&format!(
            "Could not parse \"{}\" from {}",
            attribute_label, given_configuration
        )))),
    }
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

pub fn parse_i64_as_usize(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(parse_i64(attribute_label, given_configuration)?.try_into()?)
}

pub fn parse_i64_as_u32(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(parse_i64(attribute_label, given_configuration)?.try_into()?)
}

pub fn parse_i64_as_u16(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<u16, Box<dyn std::error::Error>> {
    Ok(parse_i64(attribute_label, given_configuration)?.try_into()?)
}

pub fn parse_i64_as_i32(
    attribute_label: &str,
    given_configuration: &serde_json::Value,
) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(parse_i64(attribute_label, given_configuration)?.try_into()?)
}

#[derive(Debug)]
pub struct EvolutionConfiguration {
    pub dead_zone_radius: f64,
    pub inverse_squared_coupling: f64,
    pub inverse_fourth_coupling: f64,
    pub milliseconds_per_time_slice: u16,
    pub number_of_time_slices: usize,
}

#[derive(Debug)]
pub struct EvolverConfiguration<'a> {
    pub memory_layout: &'a str,
    pub number_of_steps_per_time_slice: u32,
}

#[derive(Debug)]
pub struct InitialParticleGeneratorConfiguration<'a> {
    pub generator_name: &'a str,
    pub generator_configuration: &'a serde_json::Value,
}

#[derive(Debug)]
pub struct PictureConfiguration {
    pub right_border_coordinate: i32,
    pub upper_border_coordinate: i32,
    pub left_border_coordinate: i32,
    pub lower_border_coordinate: i32,
}

#[derive(Debug)]
pub struct ParsedConfiguration<'a> {
    pub evolver_configuration: EvolverConfiguration<'a>,
    pub evolution_configuration: EvolutionConfiguration,
    pub generator_configurations: std::vec::Vec<InitialParticleGeneratorConfiguration<'a>>,
    pub picture_configuration: PictureConfiguration,
}

pub fn parse_deserialized_configuration<'a>(
    deserialized_configuration: &'a serde_json::Value,
) -> Result<ParsedConfiguration<'a>, Box<dyn std::error::Error>> {
    let memory_layout = parse_str(MEMORY_LAYOUT_LABEL, &deserialized_configuration)?;
    let number_of_steps_per_time_slice =
        parse_i64_as_u32(NUMBER_OF_STEPS_PER_FRAME_LABEL, &deserialized_configuration)?;
    let dead_zone_radius = parse_f64(DEAD_ZONE_RADIUS_LABEL, &deserialized_configuration)?;
    let inverse_squared_coupling =
        parse_f64(INVERSE_SQUARED_COUPLING_LABEL, &deserialized_configuration)?;
    let inverse_fourth_coupling =
        parse_f64(INVERSE_FOURTH_COUPLING_LABEL, &deserialized_configuration)?;
    let milliseconds_per_time_slice =
        parse_i64_as_u16(MILLISECONDS_PER_FRAME_LABEL, &deserialized_configuration)?;
    let number_of_time_slices =
        parse_i64_as_usize(NUMBER_OF_FRAMES_LABEL, &deserialized_configuration)?;
    let right_border_coordinate =
        parse_i64_as_i32(RIGHT_BORDER_COORDINATE_LABEL, &deserialized_configuration)?;
    let upper_border_coordinate =
        parse_i64_as_i32(UPPER_BORDER_COORDINATE_LABEL, &deserialized_configuration)?;
    let left_border_coordinate =
        parse_i64_as_i32(LEFT_BORDER_COORDINATE_LABEL, &deserialized_configuration)?;
    let lower_border_coordinate =
        parse_i64_as_i32(LOWER_BORDER_COORDINATE_LABEL, &deserialized_configuration)?;

    let mut particle_generators: std::vec::Vec<InitialParticleGeneratorConfiguration> = vec![];
    let configuration_objects =
        match deserialized_configuration[GENERATOR_CONFIGURATIONS_LABEL].as_array() {
            Some(parsed_array) => parsed_array,
            _ => {
                return Err(Box::new(ConfigurationParseError::new(&format!(
                    "Could not parse \"{}\" from {} as a JSON array.",
                    GENERATOR_CONFIGURATIONS_LABEL, deserialized_configuration
                ))))
            }
        };

    for configuration_object in configuration_objects {
        let generator_name = match configuration_object[GENERATOR_NAME_LABEL].as_str() {
            Some(parsed_string) => parsed_string,
            _ => {
                return Err(Box::new(ConfigurationParseError::new(&format!(
                    "Could not parse \"{}\" from {} in {}",
                    GENERATOR_NAME_LABEL, configuration_object, deserialized_configuration
                ))))
            }
        };
        let generator_configuration = match configuration_object.get(GENERATOR_CONFIGURATION_LABEL)
        {
            Some(parsed_value) => parsed_value,
            _ => {
                return Err(Box::new(ConfigurationParseError::new(&format!(
                    "Could not parse \"{}\" from {} in {}",
                    GENERATOR_CONFIGURATION_LABEL, configuration_object, deserialized_configuration
                ))))
            }
        };
        particle_generators.push(InitialParticleGeneratorConfiguration {
            generator_name: generator_name,
            generator_configuration: generator_configuration,
        });
    }
    Ok(ParsedConfiguration {
        evolver_configuration: EvolverConfiguration {
            memory_layout: memory_layout,
            number_of_steps_per_time_slice: number_of_steps_per_time_slice,
        },
        evolution_configuration: EvolutionConfiguration {
            dead_zone_radius: dead_zone_radius,
            inverse_squared_coupling: inverse_squared_coupling,
            inverse_fourth_coupling: inverse_fourth_coupling,
            milliseconds_per_time_slice: milliseconds_per_time_slice,
            number_of_time_slices: number_of_time_slices,
        },
        generator_configurations: particle_generators,
        picture_configuration: PictureConfiguration {
            right_border_coordinate: right_border_coordinate,
            upper_border_coordinate: upper_border_coordinate,
            left_border_coordinate: left_border_coordinate,
            lower_border_coordinate: lower_border_coordinate,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_not_an_array() -> Result<(), String> {
        let generator_name = "acceptable";
        let generator_configuration = serde_json::json!(
            {
                "internalNumber": 9001,
                "internalStringArray": ["we're", "the", "kids", "in", "America"],
            }
        );
        let valid_configuration_element = serde_json::json!(
            {
                GENERATOR_NAME_LABEL: generator_name,
                GENERATOR_CONFIGURATION_LABEL: generator_configuration,
            }
        );
        let parsing_result = parse_deserialized_configuration(&valid_configuration_element);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }
    #[test]
    fn check_reject_when_no_generator_name() -> Result<(), String> {
        let nameless_configuration = serde_json::json!(
            {
                MEMORY_LAYOUT_LABEL: "VecOfPureStruct",
                NUMBER_OF_STEPS_PER_FRAME_LABEL: 10,
                DEAD_ZONE_RADIUS_LABEL: 1.0,
                INVERSE_SQUARED_COUPLING_LABEL: -1.0,
                INVERSE_FOURTH_COUPLING_LABEL: 1.0,
                MILLISECONDS_PER_FRAME_LABEL: 100,
                NUMBER_OF_FRAMES_LABEL: 40,
                RIGHT_BORDER_COORDINATE_LABEL: 10,
                UPPER_BORDER_COORDINATE_LABEL: 10,
                LEFT_BORDER_COORDINATE_LABEL: -10,
                LOWER_BORDER_COORDINATE_LABEL: -10,
                GENERATOR_CONFIGURATIONS_LABEL:
                [
                    {
                        "gneratrNmae": "typo",
                        GENERATOR_CONFIGURATION_LABEL:
                        {
                            "internalNumber": 9001,
                            "internalStringArray": ["we're", "the", "kids", "in", "America"]
                        }
                    }
                ]
            }
        );
        let parsing_result = parse_deserialized_configuration(&nameless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_generator_name() -> Result<(), String> {
        let nameless_configuration = serde_json::json!(
            {
                MEMORY_LAYOUT_LABEL: "VecOfPureStruct",
                NUMBER_OF_STEPS_PER_FRAME_LABEL: 10,
                DEAD_ZONE_RADIUS_LABEL: 1.0,
                INVERSE_SQUARED_COUPLING_LABEL: -1.0,
                INVERSE_FOURTH_COUPLING_LABEL: 1.0,
                MILLISECONDS_PER_FRAME_LABEL: 100,
                NUMBER_OF_FRAMES_LABEL: 40,
                RIGHT_BORDER_COORDINATE_LABEL: 10,
                UPPER_BORDER_COORDINATE_LABEL: 10,
                LEFT_BORDER_COORDINATE_LABEL: -10,
                LOWER_BORDER_COORDINATE_LABEL: -10,
                GENERATOR_CONFIGURATIONS_LABEL:
                [
                    {
                        GENERATOR_NAME_LABEL: [],
                        GENERATOR_CONFIGURATION_LABEL:
                        {
                            "internalNumber": 9001,
                            "internalStringArray": ["we're", "the", "kids", "in", "America"]
                        }
                    }
                ]
            }
        );
        let parsing_result = parse_deserialized_configuration(&nameless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_no_generator_configuration() -> Result<(), String> {
        let configurationless_configuration = serde_json::json!(
            {
                MEMORY_LAYOUT_LABEL: "VecOfPureStruct",
                NUMBER_OF_STEPS_PER_FRAME_LABEL: 10,
                DEAD_ZONE_RADIUS_LABEL: 1.0,
                INVERSE_SQUARED_COUPLING_LABEL: -1.0,
                INVERSE_FOURTH_COUPLING_LABEL: 1.0,
                MILLISECONDS_PER_FRAME_LABEL: 100,
                NUMBER_OF_FRAMES_LABEL: 40,
                RIGHT_BORDER_COORDINATE_LABEL: 10,
                UPPER_BORDER_COORDINATE_LABEL: 10,
                LEFT_BORDER_COORDINATE_LABEL: -10,
                LOWER_BORDER_COORDINATE_LABEL: -10,
                GENERATOR_CONFIGURATIONS_LABEL:
                [
                    {
                        GENERATOR_NAME_LABEL: "acceptable",
                        format!("{}{}", GENERATOR_CONFIGURATION_LABEL, "x"):
                        {
                            "internalNumber": 9001,
                            "internalStringArray": ["we're", "the", "kids", "in", "America"]
                        }
                    }
                ]
            }
        );
        let parsing_result = parse_deserialized_configuration(&configurationless_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_parse_valid_single_configuration() -> Result<(), String> {
        let expected_name = "acceptable";
        let expected_configuration = serde_json::json!(
            {
                "internalNumber": 9001,
                "internalStringArray": ["we're", "the", "kids", "in", "America"],
            }
        );
        let valid_configuration = serde_json::json!(
            {
                MEMORY_LAYOUT_LABEL: "VecOfPureStruct",
                NUMBER_OF_STEPS_PER_FRAME_LABEL: 10,
                DEAD_ZONE_RADIUS_LABEL: 1.0,
                INVERSE_SQUARED_COUPLING_LABEL: -1.0,
                INVERSE_FOURTH_COUPLING_LABEL: 1.0,
                MILLISECONDS_PER_FRAME_LABEL: 100,
                NUMBER_OF_FRAMES_LABEL: 40,
                RIGHT_BORDER_COORDINATE_LABEL: 10,
                UPPER_BORDER_COORDINATE_LABEL: 10,
                LEFT_BORDER_COORDINATE_LABEL: -10,
                LOWER_BORDER_COORDINATE_LABEL: -10,
                GENERATOR_CONFIGURATIONS_LABEL:
                [
                    {
                        GENERATOR_NAME_LABEL: expected_name,
                        GENERATOR_CONFIGURATION_LABEL: expected_configuration,
                    }
                ]
            }
        );
        let parsing_result = parse_deserialized_configuration(&valid_configuration)
            .expect("Should parse valid JSON object");
        if parsing_result.generator_configurations.len() != 1 {
            return Err(String::from(format!(
                "Expected vector of 1 element, actually parsed {:?}",
                parsing_result
            )));
        }
        let actual_single_configuration = &parsing_result.generator_configurations[0];
        if (actual_single_configuration.generator_name == expected_name)
            && (actual_single_configuration.generator_configuration == &expected_configuration)
        {
            Ok(())
        } else {
            Err(String::from(format!(
                "Expected name = {}, configuration = {}, actually parsed {:?}",
                expected_name, expected_configuration, actual_single_configuration
            )))
        }
    }

    #[test]
    fn check_parse_two_valid_configurations() -> Result<(), String> {
        let expected_names = ["acceptable", "unproblematic"];
        let expected_configurations = [
            serde_json::json!(
                {
                    "internalNumber": 9001,
                    "internalStringArray": ["we're", "the", "kids", "in", "America"],
                }
            ),
            serde_json::json!(
                {
                    "internalNumber": 9002,
                    "internalStringArray": ["oh-wo-oh"],
                }
            ),
        ];
        let valid_configuration = serde_json::json!(
            {
                MEMORY_LAYOUT_LABEL: "VecOfPureStruct",
                NUMBER_OF_STEPS_PER_FRAME_LABEL: 10,
                DEAD_ZONE_RADIUS_LABEL: 1.0,
                INVERSE_SQUARED_COUPLING_LABEL: -1.0,
                INVERSE_FOURTH_COUPLING_LABEL: 1.0,
                MILLISECONDS_PER_FRAME_LABEL: 100,
                NUMBER_OF_FRAMES_LABEL: 40,
                RIGHT_BORDER_COORDINATE_LABEL: 10,
                UPPER_BORDER_COORDINATE_LABEL: 10,
                LEFT_BORDER_COORDINATE_LABEL: -10,
                LOWER_BORDER_COORDINATE_LABEL: -10,
                GENERATOR_CONFIGURATIONS_LABEL:
                [
                    {
                        GENERATOR_NAME_LABEL: expected_names[0],
                        GENERATOR_CONFIGURATION_LABEL: expected_configurations[0],
                    },
                    {
                        GENERATOR_NAME_LABEL: expected_names[1],
                        GENERATOR_CONFIGURATION_LABEL: expected_configurations[1],
                    }
                ]
            }
        );
        let parsing_result = parse_deserialized_configuration(&valid_configuration)
            .expect("Should parse valid JSON object");
        if parsing_result.generator_configurations.len() != 2 {
            return Err(String::from(format!(
                "Expected vector of 2 element, actually parsed {:?}",
                parsing_result
            )));
        }
        let mut error_messages: std::vec::Vec<String> = vec![];
        for comparison_index in 0..2 {
            let actual_configuration_element =
                &parsing_result.generator_configurations[comparison_index];
            if (actual_configuration_element.generator_name != expected_names[comparison_index])
                || (actual_configuration_element.generator_configuration
                    != &expected_configurations[comparison_index])
            {
                error_messages.push(String::from(format!(
                    "Expected name = {}, configuration = {}, actually parsed {:?}",
                    expected_names[comparison_index],
                    expected_configurations[comparison_index],
                    actual_configuration_element
                )));
            }
        }

        if error_messages.is_empty() {
            Ok(())
        } else {
            Err(String::from(format!(
                "Failed comparisons: {:?}",
                error_messages
            )))
        }
    }
}
