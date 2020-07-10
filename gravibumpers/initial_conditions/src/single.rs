/// This module provides a function to put a single particle in a vector.

const DISPLACEMENT_LABEL: &str = "displacement";
const VELOCITY_LABEL: &str = "velocity";
const MASS_LABEL: &str = "mass";
const GRAV_LABEL: &str = "grav";
const BUMP_LABEL: &str = "bump";
const RED_LABEL: &str = "red";
const GREEN_LABEL: &str = "green";
const BLUE_LABEL: &str = "blue";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
    let particle_displacement = super::parse_position(&given_configuration[DISPLACEMENT_LABEL])?;
    let particle_velocity = super::parse_velocity(&given_configuration[VELOCITY_LABEL])?;
    let inertial_mass = super::parse_f64(MASS_LABEL, given_configuration)?;
    let attractive_charge = super::parse_f64(GRAV_LABEL, given_configuration)?;
    let repulsive_charge = super::parse_f64(BUMP_LABEL, given_configuration)?;
    let red_brightness = super::parse_f64(RED_LABEL, given_configuration)?;
    let green_brightness = super::parse_f64(GREEN_LABEL, given_configuration)?;
    let blue_brightness = super::parse_f64(BLUE_LABEL, given_configuration)?;

    Ok(vec![data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(inertial_mass),
            attractive_charge: data_structure::AttractiveChargeUnit(attractive_charge),
            repulsive_charge: data_structure::RepulsiveChargeUnit(repulsive_charge),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(red_brightness),
                data_structure::GreenColorUnit(green_brightness),
                data_structure::BlueColorUnit(blue_brightness),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            horizontal_position: particle_displacement.horizontal_position,
            vertical_position: particle_displacement.vertical_position,
            horizontal_velocity: particle_velocity.horizontal_velocity,
            vertical_velocity: particle_velocity.vertical_velocity,
        },
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_missing_attribute() -> Result<(), String> {
        let required_attributes = vec![
            DISPLACEMENT_LABEL,
            VELOCITY_LABEL,
            MASS_LABEL,
            GRAV_LABEL,
            BUMP_LABEL,
            RED_LABEL,
            GREEN_LABEL,
            BLUE_LABEL,
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
            DISPLACEMENT_LABEL,
            VELOCITY_LABEL,
            MASS_LABEL,
            GRAV_LABEL,
            BUMP_LABEL,
            RED_LABEL,
            GREEN_LABEL,
            BLUE_LABEL,
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
