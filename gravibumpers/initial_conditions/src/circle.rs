use super::ConfigurationParseError;
use std::convert::TryInto;

const RADIUS_LABEL: &str = "radius";
const POPULATION_LABEL: &str = "population";
const ROTATION_LABEL: &str = "rotation";
const MASS_LABEL: &str = "mass";
const GRAV_LABEL: &str = "grav";
const BUMP_LABEL: &str = "bump";
const RED_LABEL: &str = "red";
const GREEN_LABEL: &str = "green";
const BLUE_LABEL: &str = "blue";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleIteratorProvider>, Box<dyn std::error::Error>> {
    let circle_radius = parse_f64(RADIUS_LABEL, given_configuration)?;
    let circle_population = parse_i64(POPULATION_LABEL, given_configuration)?;
    let circle_rotation = parse_f64(ROTATION_LABEL, given_configuration)?;
    let inertial_mass = parse_f64(MASS_LABEL, given_configuration)?;
    let attractive_charge = parse_f64(GRAV_LABEL, given_configuration)?;
    let repulsive_charge = parse_f64(BUMP_LABEL, given_configuration)?;
    let red_brightness = parse_f64(RED_LABEL, given_configuration)?;
    let green_brightness = parse_f64(GREEN_LABEL, given_configuration)?;
    let blue_brightness = parse_f64(BLUE_LABEL, given_configuration)?;
    let common_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(inertial_mass),
        attractive_charge: data_structure::AttractiveChargeUnit(attractive_charge),
        repulsive_charge: data_structure::RepulsiveChargeUnit(repulsive_charge),
        red_brightness: data_structure::RedColorUnit(red_brightness),
        green_brightness: data_structure::GreenColorUnit(green_brightness),
        blue_brightness: data_structure::BlueColorUnit(blue_brightness),
    };
    particles_from_numbers(
        circle_radius,
        circle_population,
        circle_rotation,
        common_intrinsics,
    )
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

fn particles_from_numbers(
    circle_radius: f64,
    circle_population: i64,
    circle_rotation: f64,
    common_intrinsics: data_structure::ParticleIntrinsics,
) -> Result<Box<dyn data_structure::ParticleIteratorProvider>, Box<dyn std::error::Error>> {
    if circle_population < 2 {
        return Err(Box::new(ConfigurationParseError::new(&format!(
            "Population {} is not large enough (must be 2 or larger)",
            circle_population
        ))));
    }

    let mut circle_particles: std::vec::Vec<data_structure::IndividualParticle> =
        std::vec::Vec::with_capacity(circle_population.try_into()?);

    // We always start with a particle at north, the top of the circle.
    circle_particles.push(data_structure::IndividualParticle {
        intrinsic_values: common_intrinsics,
        variable_values: data_structure::ParticleVariables {
            horizontal_position: data_structure::HorizontalPositionUnit(0.0),
            vertical_position: data_structure::VerticalPositionUnit(circle_radius),
            horizontal_velocity: data_structure::HorizontalVelocityUnit(
                circle_radius * circle_rotation,
            ),
            vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
        },
    });
    if (circle_population % 2) == 0 {
        // If the number of particles is even, then there is a particle at south, the bottom of the
        // circle.
        circle_particles.push(data_structure::IndividualParticle {
            intrinsic_values: common_intrinsics,
            variable_values: data_structure::ParticleVariables {
                horizontal_position: data_structure::HorizontalPositionUnit(0.0),
                vertical_position: data_structure::VerticalPositionUnit(-circle_radius),
                horizontal_velocity: data_structure::HorizontalVelocityUnit(
                    -circle_radius * circle_rotation,
                ),
                vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
            },
        });
    }

    // Apart from the particle at north, and a possible particle at south, the particles come
    // in pairs reflected in the vertical axis.
    let number_of_horizontal_pairs = (circle_population / 2) - 1;

    if number_of_horizontal_pairs > 0 {
        let angle_between_particles_in_radians =
            std::f64::consts::PI / (number_of_horizontal_pairs + 1) as f64;
        let mut angle_from_north_in_radians = 0.0;
        for _ in 1..number_of_horizontal_pairs {
            angle_from_north_in_radians += angle_between_particles_in_radians;
            let horizontal_position_value = angle_from_north_in_radians.sin() * circle_radius;
            let vertical_position_value = angle_from_north_in_radians.cos() * circle_radius;

            circle_particles.push(data_structure::IndividualParticle {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(
                        horizontal_position_value,
                    ),
                    vertical_position: data_structure::VerticalPositionUnit(
                        vertical_position_value,
                    ),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(??),
                    vertical_velocity: data_structure::VerticalVelocityUnit(??),
                },
            });
        }
    }

    Ok(data_structure::wrap_particle_vector(circle_particles))
}

fn particle_from_numbers(
    circle_radius: f64,
    angle_from_north_in_radians: f64,
    circle_rotation: f64,
    common_intrinsics: data_structure::ParticleIntrinsics,
) -> data_structure::IndividualParticle {
    data_structure::IndividualParticle {
        intrinsic_values: common_intrinsics,
        variable_values: data_structure::ParticleVariables {
            horizontal_position: data_structure::HorizontalPositionUnit(0.0),
            vertical_position: data_structure::VerticalPositionUnit(circle_radius),
            horizontal_velocity: data_structure::HorizontalVelocityUnit(
                circle_radius * circle_rotation,
            ),
            vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
        },
    }
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
        let mut generated_particles =
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
            generated_particles.get(),
            &PARTICLE_TOLERANCE,
        )
    }

    #[test]
    fn check_parse_three_points() -> Result<(), String> {
        let three_point_configuration = serde_json::json!({
            RADIUS_LABEL: 1.0,
            POPULATION_LABEL: 3,
        });
        let mut generated_particles =
            from_json(&three_point_configuration).expect("Valid configuration should be parsed.");
        let lower_horizontal_magnitude = 0.866;
        let lower_vertical_coordinate = data_structure::VerticalPositionUnit(-0.5);
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(1.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(lower_horizontal_magnitude),
                lower_vertical_coordinate,
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(-lower_horizontal_magnitude),
                lower_vertical_coordinate,
            ),
        ];

        data_structure::comparison::unordered_within_tolerance(
            &mut expected_particles.iter().cloned(),
            generated_particles.get(),
            &PARTICLE_TOLERANCE,
        )
    }

    #[test]
    fn check_parse_four_points() -> Result<(), String> {
        let four_point_configuration = serde_json::json!({
            RADIUS_LABEL: 1.0,
            POPULATION_LABEL: 4,
        });
        let mut generated_particles =
            from_json(&four_point_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(1.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(1.0),
                data_structure::VerticalPositionUnit(0.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(-1.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(-1.0),
                data_structure::VerticalPositionUnit(0.0),
            ),
        ];

        data_structure::comparison::unordered_within_tolerance(
            &mut expected_particles.iter().cloned(),
            generated_particles.get(),
            &PARTICLE_TOLERANCE,
        )
    }
}
