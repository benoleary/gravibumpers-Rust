/// This module provides a function to put a single particle in a vector.

const COMMON_DISPLACEMENT_IN_PIXELS_LABEL: &str = "commonDisplacementInPixels";
const LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL: &str = "linearVelocityInPixelsPerSecond";
const INERTIAL_MASS_IN_MASS_UNITS_LABEL: &str = "inertialMassInMassUnits";
const INVERSE_SQUARED_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL: &str =
    "inverseSquaredChargeInDimensionlessUnits";
const INVERSE_FOURTH_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL: &str =
    "inverseFourthChargeInDimensionlessUnits";
const RED_PIXEL_STRENGTH_LABEL: &str = "redPixelStrength";
const GREEN_PIXEL_STRENGTH_LABEL: &str = "greenPixelStrength";
const BLUE_PIXEL_STRENGTH_LABEL: &str = "bluePixelStrength";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
    let particle_displacement =
        super::parse_position(&given_configuration[COMMON_DISPLACEMENT_IN_PIXELS_LABEL])?;
    let particle_velocity =
        super::parse_velocity(&given_configuration[LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL])?;
    let inertial_mass = super::configuration_parsing::parse_f64(
        INERTIAL_MASS_IN_MASS_UNITS_LABEL,
        given_configuration,
    )?;
    let inverse_squared_charge = super::configuration_parsing::parse_f64(
        INVERSE_SQUARED_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
        given_configuration,
    )?;
    let inverse_fourth_charge = super::configuration_parsing::parse_f64(
        INVERSE_FOURTH_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
        given_configuration,
    )?;
    let red_brightness =
        super::configuration_parsing::parse_f64(RED_PIXEL_STRENGTH_LABEL, given_configuration)?;
    let green_brightness =
        super::configuration_parsing::parse_f64(GREEN_PIXEL_STRENGTH_LABEL, given_configuration)?;
    let blue_brightness =
        super::configuration_parsing::parse_f64(BLUE_PIXEL_STRENGTH_LABEL, given_configuration)?;

    Ok(vec![data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(inertial_mass),
            inverse_squared_charge: data_structure::InverseSquaredChargeUnit(
                inverse_squared_charge,
            ),
            inverse_fourth_charge: data_structure::InverseFourthChargeUnit(inverse_fourth_charge),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(red_brightness),
                data_structure::GreenColorUnit(green_brightness),
                data_structure::BlueColorUnit(blue_brightness),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            position_vector: particle_displacement,
            velocity_vector: particle_velocity,
        },
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_missing_attribute() -> Result<(), String> {
        let required_attributes = vec![
            COMMON_DISPLACEMENT_IN_PIXELS_LABEL,
            LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL,
            INERTIAL_MASS_IN_MASS_UNITS_LABEL,
            INVERSE_SQUARED_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
            INVERSE_FOURTH_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
            RED_PIXEL_STRENGTH_LABEL,
            GREEN_PIXEL_STRENGTH_LABEL,
            BLUE_PIXEL_STRENGTH_LABEL,
        ];

        let mut failed_cases: std::vec::Vec<String> = vec![];
        for missing_attribute in &required_attributes {
            let mut configuration_without_attribute = serde_json::json!({});
            for present_attribute in &required_attributes {
                // Every attribute of the configuration is numeric, even though one of them should
                // be integer.
                if present_attribute != missing_attribute {
                    configuration_without_attribute[present_attribute] = serde_json::json!(9001.0);
                }
            }

            let parsing_result = from_json(&configuration_without_attribute);
            if !parsing_result.is_err() {
                failed_cases.push(missing_attribute.to_string());
            }
        }

        if failed_cases.is_empty() {
            Ok(())
        } else {
            Err(String::from(format!(
                "Did not get an error from the following: {:?}",
                failed_cases
            )))
        }
    }

    #[test]
    fn check_reject_when_malformed_attribute() -> Result<(), String> {
        let required_attributes = vec![
            COMMON_DISPLACEMENT_IN_PIXELS_LABEL,
            LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL,
            INERTIAL_MASS_IN_MASS_UNITS_LABEL,
            INVERSE_SQUARED_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
            INVERSE_FOURTH_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL,
            RED_PIXEL_STRENGTH_LABEL,
            GREEN_PIXEL_STRENGTH_LABEL,
            BLUE_PIXEL_STRENGTH_LABEL,
        ];

        let mut failed_cases: std::vec::Vec<String> = vec![];
        for malformed_attribute in &required_attributes {
            let mut configuration_without_attribute = serde_json::json!({});
            for present_attribute in &required_attributes {
                // Every attribute of the configuration is numeric, even though one of them should
                // be integer.
                if present_attribute != malformed_attribute {
                    configuration_without_attribute[present_attribute] = serde_json::json!(9001.0);
                } else {
                    configuration_without_attribute[present_attribute] =
                        serde_json::json!("over nine thousand");
                }
            }

            let parsing_result = from_json(&configuration_without_attribute);
            if !parsing_result.is_err() {
                failed_cases.push(malformed_attribute.to_string());
            }
        }

        if failed_cases.is_empty() {
            Ok(())
        } else {
            Err(String::from(format!(
                "Did not get an error from the following: {:?}",
                failed_cases
            )))
        }
    }
}
