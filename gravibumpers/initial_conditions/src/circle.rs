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
    angular_velocity: f64,
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

    // We always start with a particle at 0 radians from the positive x axis.
    circle_particles.push(data_structure::IndividualParticle {
        intrinsic_values: common_intrinsics,
        variable_values: data_structure::ParticleVariables {
            horizontal_position: data_structure::HorizontalPositionUnit(circle_radius),
            vertical_position: data_structure::VerticalPositionUnit(0.0),
            horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
            vertical_velocity: data_structure::VerticalVelocityUnit(
                circle_radius * angular_velocity,
            ),
        },
    });

    if (circle_population % 2) == 0 {
        // If the number of particles is even, then there is a particle pi radians from the
        // positive x axis.
        circle_particles.push(data_structure::IndividualParticle {
            intrinsic_values: common_intrinsics,
            variable_values: data_structure::ParticleVariables {
                horizontal_position: data_structure::HorizontalPositionUnit(-circle_radius),
                vertical_position: data_structure::VerticalPositionUnit(0.0),
                horizontal_velocity: data_structure::HorizontalVelocityUnit(
                    -circle_radius * angular_velocity,
                ),
                vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
            },
        });
    }

    // Apart from the particle or pair of particles on the x axis, the rest of the particles come
    // in pairs with equal displacements above and below the x axis.
    let number_of_vertical_pairs = (circle_population - 1) / 2;

    if number_of_vertical_pairs > 0 {
        let angle_between_particles_in_radians =
            std::f64::consts::PI / (number_of_vertical_pairs + 1) as f64;
        let mut angle_from_horizontal_in_radians = 0.0;
        for _ in 0..number_of_vertical_pairs {
            angle_from_horizontal_in_radians += angle_between_particles_in_radians;
            let cosine_of_angle_times_radius =
                angle_from_horizontal_in_radians.cos() * circle_radius;
            let sine_of_angle_times_radius = angle_from_horizontal_in_radians.sin() * circle_radius;

            circle_particles.push(data_structure::IndividualParticle {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(
                        cosine_of_angle_times_radius,
                    ),
                    vertical_position: data_structure::VerticalPositionUnit(
                        sine_of_angle_times_radius,
                    ),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(
                        -sine_of_angle_times_radius * angular_velocity,
                    ),
                    vertical_velocity: data_structure::VerticalVelocityUnit(
                        cosine_of_angle_times_radius * angular_velocity,
                    ),
                },
            });

            circle_particles.push(data_structure::IndividualParticle {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(
                        cosine_of_angle_times_radius,
                    ),
                    vertical_position: data_structure::VerticalPositionUnit(
                        -sine_of_angle_times_radius,
                    ),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(
                        sine_of_angle_times_radius * angular_velocity,
                    ),
                    vertical_velocity: data_structure::VerticalVelocityUnit(
                        cosine_of_angle_times_radius * angular_velocity,
                    ),
                },
            });
        }
    }

    Ok(data_structure::wrap_particle_vector(circle_particles))
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

    fn new_test_configuration(
        test_radius: serde_json::Value,
        test_rotation: serde_json::Value,
    ) -> serde_json::Value {
        serde_json::json!({
            RADIUS_LABEL: test_radius,
            ROTATION_LABEL: test_rotation,
            MASS_LABEL: TEST_INTRINSICS.inertial_mass.0,
            GRAV_LABEL: TEST_INTRINSICS.attractive_charge.0,
            BUMP_LABEL: TEST_INTRINSICS.repulsive_charge.0,
            RED_LABEL: TEST_INTRINSICS.red_brightness.0,
            GREEN_LABEL: TEST_INTRINSICS.green_brightness.0,
            BLUE_LABEL: TEST_INTRINSICS.blue_brightness.0,
        })
    }

    fn new_test_particle_at(
        horizontal_position: data_structure::HorizontalPositionUnit,
        vertical_position: data_structure::VerticalPositionUnit,
        horizontal_velocity: data_structure::HorizontalVelocityUnit,
        vertical_velocity: data_structure::VerticalVelocityUnit,
    ) -> data_structure::IndividualParticle {
        data_structure::IndividualParticle {
            intrinsic_values: TEST_INTRINSICS,
            variable_values: data_structure::ParticleVariables {
                horizontal_position: horizontal_position,
                vertical_position: vertical_position,
                horizontal_velocity: horizontal_velocity,
                vertical_velocity: vertical_velocity,
            },
        }
    }

    #[test]
    fn check_reject_when_missing_attribute() -> Result<(), String> {
        let required_attributes = vec![
            ROTATION_LABEL,
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
            RADIUS_LABEL,
            POPULATION_LABEL,
            ROTATION_LABEL,
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
    #[test]
    fn check_reject_when_no_population() -> Result<(), String> {
        let configuration_without_population =
            new_test_configuration(serde_json::json!(9001.0), serde_json::json!(9002.0));
        let parsing_result = from_json(&configuration_without_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_population() -> Result<(), String> {
        let mut configuration_with_array_population =
            new_test_configuration(serde_json::json!(9001.0), serde_json::json!(9002.0));
        configuration_with_array_population[POPULATION_LABEL] = serde_json::json!([9001.0, 9002.0]);
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_zero_population() -> Result<(), String> {
        let mut test_configuration =
            new_test_configuration(serde_json::json!(9001.0), serde_json::json!(9002.0));
        test_configuration[POPULATION_LABEL] = serde_json::json!(0);
        let parsing_result = from_json(&test_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_population_is_one() -> Result<(), String> {
        let mut test_configuration =
            new_test_configuration(serde_json::json!(9001.0), serde_json::json!(9002.0));
        test_configuration[POPULATION_LABEL] = serde_json::json!(1);
        let parsing_result = from_json(&test_configuration);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_parse_two_points() -> Result<(), String> {
        let test_radius = 1.0;
        let test_speed = 2.0;
        let mut test_configuration = new_test_configuration(
            serde_json::json!(test_radius),
            serde_json::json!(test_speed),
        );
        test_configuration[POPULATION_LABEL] = serde_json::json!(2);
        let mut generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_radius),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(test_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(-test_radius),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(-test_speed),
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
        let test_radius = 2.0;

        // This time we will have zero angular velocity to keep the calculation simple.
        let mut test_configuration =
            new_test_configuration(serde_json::json!(test_radius), serde_json::json!(0.0));
        test_configuration[POPULATION_LABEL] = serde_json::json!(3);
        let mut generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let left_vertical_magnitude = 0.866;
        let left_horizontal_coordinate = data_structure::HorizontalPositionUnit(-0.5);
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(1.0),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(0.0),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::VerticalPositionUnit(left_vertical_magnitude),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(0.0),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::VerticalPositionUnit(-left_vertical_magnitude),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(0.0),
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
        let test_radius = 2.5;
        let test_speed = 0.1;
        let mut test_configuration = new_test_configuration(
            serde_json::json!(test_radius),
            serde_json::json!(test_speed),
        );
        test_configuration[POPULATION_LABEL] = serde_json::json!(4);
        let mut generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_radius),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(test_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(test_radius),
                data_structure::HorizontalVelocityUnit(-test_speed),
                data_structure::VerticalVelocityUnit(0.0),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(-test_radius),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(-test_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(0.0),
                data_structure::VerticalPositionUnit(-test_radius),
                data_structure::HorizontalVelocityUnit(test_speed),
                data_structure::VerticalVelocityUnit(0.0),
            ),
        ];

        data_structure::comparison::unordered_within_tolerance(
            &mut expected_particles.iter().cloned(),
            generated_particles.get(),
            &PARTICLE_TOLERANCE,
        )
    }
}
