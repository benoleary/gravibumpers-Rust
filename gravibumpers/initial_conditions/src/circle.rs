use super::ConfigurationParseError;
use std::iter::FromIterator;

const RADIUS_LABEL: &str = "radius";
const POPULATION_LABEL: &str = "population";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
    let circle_radius = match given_configuration[RADIUS_LABEL].as_f64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                RADIUS_LABEL, given_configuration
            ))))
        }
    };
    let circle_population = match given_configuration[POPULATION_LABEL].as_i64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                POPULATION_LABEL, given_configuration
            ))))
        }
    };
    from_numbers(circle_radius, circle_population)
}

fn from_numbers(
    circle_radius: f64,
    circle_population: i64,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
    Err(Box::new(ConfigurationParseError::new(&format!(
        "Not yet implemented"
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTRINSICS_TOLERANCE: data_structure::ParticleIntrinsics =
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(0.01),
            attractive_charge: data_structure::AttractiveChargeUnit(0.01),
            repulsive_charge: data_structure::RepulsiveChargeUnit(0.01),
            red_brightness: data_structure::RedColorUnit(0.01),
            green_brightness: data_structure::GreenColorUnit(0.01),
            blue_brightness: data_structure::BlueColorUnit(0.01),
        };

    const VARIABLES_TOLERANCE: data_structure::ParticleVariables =
        data_structure::ParticleVariables {
            horizontal_position: data_structure::HorizontalPositionUnit(0.01),
            vertical_position: data_structure::VerticalPositionUnit(0.01),
            horizontal_velocity: data_structure::HorizontalVelocityUnit(0.01),
            vertical_velocity: data_structure::VerticalVelocityUnit(0.01),
        };

    const PARTICLE_TOLERANCE: data_structure::IndividualParticle =
        data_structure::IndividualParticle {
            intrinsic_values: INTRINSICS_TOLERANCE,
            variable_values: VARIABLES_TOLERANCE,
        };

    const TEST_INTRINSICS: data_structure::ParticleIntrinsics =
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.9),
            attractive_charge: data_structure::AttractiveChargeUnit(2.8),
            repulsive_charge: data_structure::RepulsiveChargeUnit(3.7),
            red_brightness: data_structure::RedColorUnit(4.6),
            green_brightness: data_structure::GreenColorUnit(5.5),
            blue_brightness: data_structure::BlueColorUnit(6.4),
        };

    fn new_test_particle_at(
        horizontal_position: data_structure::HorizontalPositionUnit,
        vertical_position: data_structure::VerticalPositionUnit,
    ) -> data_structure::IndividualParticle {
        data_structure::IndividualParticle {
            intrinsic_values: TEST_INTRINSICS,
            variable_values: data_structure::ParticleVariables {
                horizontal_position: horizontal_position,
                vertical_position: vertical_position,
                horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
            },
        }
    }

    #[test]
    fn check_reject_when_no_radius() -> Result<(), String> {
        let configuration_without_radius = serde_json::json!({
            POPULATION_LABEL: 9001
        });
        let parsing_result = from_json(&configuration_without_radius);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_radius() -> Result<(), String> {
        let configuration_with_string_radius = serde_json::json!({
            RADIUS_LABEL: "over nine thousand",
            POPULATION_LABEL: 9001
        });
        let parsing_result = from_json(&configuration_with_string_radius);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_no_population() -> Result<(), String> {
        let configuration_without_population = serde_json::json!({
            RADIUS_LABEL: 9001.0
        });
        let parsing_result = from_json(&configuration_without_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_population() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: [9001.0, 9002.0]
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_zero_population() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: 0,
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_population_is_one() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: 1,
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_parse_two_points() -> Result<(), String> {
        let two_point_configuration = serde_json::json!({
            RADIUS_LABEL: 1.0,
            POPULATION_LABEL: 2,
        });
        let generated_particles =
            from_json(&two_point_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(1.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(-1.0),
            ),
        ];

        data_structure::comparison::unordered_within_tolerance(
            &mut expected_particles.iter().cloned(),
            generated_particles.iter().cloned(),
            &PARTICLE_TOLERANCE,
        )
    }

    #[test]
    fn check_parse_three_points() -> Result<(), String> {
        let three_point_configuration = serde_json::json!({
            RADIUS_LABEL: 1.0,
            POPULATION_LABEL: 3,
        });
        let generated_particles =
            from_json(&three_point_configuration).expect("Valid configuration should be parsed.");
        let number_of_particles = (*generated_particles).len();
        if number_of_particles != 3 {
            return Err(String::from(format!(
                "Expected 3 points, got {}",
                number_of_particles
            )));
        }

        Ok(())
    }

    #[test]
    fn check_parse_four_points() -> Result<(), String> {
        let four_point_configuration = serde_json::json!({
            RADIUS_LABEL: 1.0,
            POPULATION_LABEL: 4,
        });
        let generated_particles =
            from_json(&four_point_configuration).expect("Valid configuration should be parsed.");
        let number_of_particles = (*generated_particles).len();
        if number_of_particles != 4 {
            return Err(String::from(format!(
                "Expected 4 points, got {}",
                number_of_particles
            )));
        }

        Ok(())
    }
}
