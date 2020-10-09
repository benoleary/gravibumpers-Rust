/// This crate provides structs, traits, and functions for producing initial conditions based on
/// JSON configurations, represented using the structs and traits defined in data_structure and
/// serde_json.
extern crate configuration_parsing;
extern crate data_structure;
extern crate serde_json;
pub mod circle;
pub mod single;

const HORIZONTAL_LABEL: &str = "x";
const VERTICAL_LABEL: &str = "y";

pub fn parse_position(
    given_position: &serde_json::Value,
) -> Result<data_structure::position::DimensionfulVector, Box<dyn std::error::Error>> {
    let horizontal_position = configuration_parsing::parse_f64(HORIZONTAL_LABEL, given_position)?;
    let vertical_position = configuration_parsing::parse_f64(VERTICAL_LABEL, given_position)?;
    Ok(data_structure::position::DimensionfulVector {
        horizontal_component: data_structure::position::HorizontalUnit(horizontal_position),
        vertical_component: data_structure::position::VerticalUnit(vertical_position),
    })
}

pub fn parse_velocity(
    given_position: &serde_json::Value,
) -> Result<data_structure::velocity::DimensionfulVector, Box<dyn std::error::Error>> {
    let horizontal_velocity = configuration_parsing::parse_f64(HORIZONTAL_LABEL, given_position)?;
    let vertical_velocity = configuration_parsing::parse_f64(VERTICAL_LABEL, given_position)?;
    Ok(data_structure::velocity::DimensionfulVector {
        horizontal_component: data_structure::velocity::HorizontalUnit(horizontal_velocity),
        vertical_component: data_structure::velocity::VerticalUnit(vertical_velocity),
    })
}
