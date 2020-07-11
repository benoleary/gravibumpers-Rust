/// This module provides a function to put particles evenly around a circle, with a common angular
/// speed around the center.
use super::ConfigurationParseError;
use std::convert::TryInto;

const DISPLACEMENT_LABEL: &str = "displacement";
const VELOCITY_LABEL: &str = "velocity";
const RADIUS_LABEL: &str = "radius";
const POPULATION_LABEL: &str = "population";
const ANGULAR_VELOCITY_LABEL: &str = "rotation";
const MASS_LABEL: &str = "mass";
const GRAV_LABEL: &str = "grav";
const BUMP_LABEL: &str = "bump";
const RED_LABEL: &str = "red";
const GREEN_LABEL: &str = "green";
const BLUE_LABEL: &str = "blue";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
    let circle_displacement = super::parse_position(&given_configuration[DISPLACEMENT_LABEL])?;
    let circle_velocity = super::parse_velocity(&given_configuration[VELOCITY_LABEL])?;
    let circle_radius = super::parse_f64(RADIUS_LABEL, given_configuration)?;
    let circle_population = super::parse_i64(POPULATION_LABEL, given_configuration)?;
    let circle_rotation = super::parse_f64(ANGULAR_VELOCITY_LABEL, given_configuration)?;
    let inertial_mass = super::parse_f64(MASS_LABEL, given_configuration)?;
    let attractive_charge = super::parse_f64(GRAV_LABEL, given_configuration)?;
    let repulsive_charge = super::parse_f64(BUMP_LABEL, given_configuration)?;
    let red_brightness = super::parse_f64(RED_LABEL, given_configuration)?;
    let green_brightness = super::parse_f64(GREEN_LABEL, given_configuration)?;
    let blue_brightness = super::parse_f64(BLUE_LABEL, given_configuration)?;
    let common_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(inertial_mass),
        attractive_charge: data_structure::AttractiveChargeUnit(attractive_charge),
        repulsive_charge: data_structure::RepulsiveChargeUnit(repulsive_charge),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(red_brightness),
            data_structure::GreenColorUnit(green_brightness),
            data_structure::BlueColorUnit(blue_brightness),
        ),
    };
    particles_from_numbers(
        circle_displacement,
        circle_velocity,
        circle_radius,
        circle_population,
        circle_rotation,
        common_intrinsics,
    )
}

fn particles_from_numbers(
    circle_displacement: data_structure::PositionVector,
    circle_velocity: data_structure::VelocityVector,
    circle_radius: f64,
    circle_population: i64,
    angular_velocity: f64,
    common_intrinsics: data_structure::ParticleIntrinsics,
) -> Result<std::vec::Vec<data_structure::IndividualParticle>, Box<dyn std::error::Error>> {
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
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(circle_radius)
                    + circle_displacement.horizontal_component,
                vertical_component: data_structure::VerticalPositionUnit(0.0)
                    + circle_displacement.vertical_component,
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0)
                    + circle_velocity.horizontal_component,
                vertical_component: data_structure::VerticalVelocityUnit(
                    circle_radius * angular_velocity,
                ) + circle_velocity.vertical_component,
            },
        },
    });

    if (circle_population % 2) == 0 {
        // If the number of particles is even, then there is a particle at pi radians from the
        // positive x axis.
        circle_particles.push(data_structure::IndividualParticle {
            intrinsic_values: common_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(-circle_radius)
                        + circle_displacement.horizontal_component,
                    vertical_component: data_structure::VerticalPositionUnit(0.0)
                        + circle_displacement.vertical_component,
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.0)
                        + circle_velocity.horizontal_component,
                    vertical_component: data_structure::VerticalVelocityUnit(
                        -circle_radius * angular_velocity,
                    ) + circle_velocity.vertical_component,
                },
            },
        });
    }

    // Apart from the particle or pair of particles on the x axis, the rest of the particles come
    // in pairs with equal displacements above and below the x axis.
    let number_of_vertical_pairs = (circle_population - 1) / 2;

    if number_of_vertical_pairs > 0 {
        let angle_between_particles_in_radians =
            (2.0 * std::f64::consts::PI) / (circle_population as f64);
        let mut angle_from_horizontal_in_radians = 0.0;
        for _ in 0..number_of_vertical_pairs {
            angle_from_horizontal_in_radians += angle_between_particles_in_radians;
            let cosine_of_angle_times_radius =
                angle_from_horizontal_in_radians.cos() * circle_radius;
            let sine_of_angle_times_radius = angle_from_horizontal_in_radians.sin() * circle_radius;

            circle_particles.push(data_structure::IndividualParticle {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            cosine_of_angle_times_radius,
                        ) + circle_displacement.horizontal_component,
                        vertical_component: data_structure::VerticalPositionUnit(
                            sine_of_angle_times_radius,
                        ) + circle_displacement.vertical_component,
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            -sine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.horizontal_component,
                        vertical_component: data_structure::VerticalVelocityUnit(
                            cosine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.vertical_component,
                    },
                },
            });

            circle_particles.push(data_structure::IndividualParticle {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            cosine_of_angle_times_radius,
                        ) + circle_displacement.horizontal_component,
                        vertical_component: data_structure::VerticalPositionUnit(
                            -sine_of_angle_times_radius,
                        ) + circle_displacement.vertical_component,
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            sine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.horizontal_component,
                        vertical_component: data_structure::VerticalVelocityUnit(
                            cosine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.vertical_component,
                    },
                },
            });
        }
    }

    Ok(circle_particles)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_intrinsics_tolerance() -> data_structure::ParticleIntrinsics {
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(0.01),
            attractive_charge: data_structure::AttractiveChargeUnit(0.01),
            repulsive_charge: data_structure::RepulsiveChargeUnit(0.01),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(0.01),
                data_structure::GreenColorUnit(0.01),
                data_structure::BlueColorUnit(0.01),
            ),
        }
    }

    fn new_variables_tolerance() -> data_structure::ParticleVariables {
        data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(0.01),
                vertical_component: data_structure::VerticalPositionUnit(0.01),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.01),
                vertical_component: data_structure::VerticalVelocityUnit(0.01),
            },
        }
    }

    fn new_particle_tolerance() -> data_structure::IndividualParticle {
        data_structure::IndividualParticle {
            intrinsic_values: new_intrinsics_tolerance(),
            variable_values: new_variables_tolerance(),
        }
    }

    fn new_test_intrinsics() -> data_structure::ParticleIntrinsics {
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.9),
            attractive_charge: data_structure::AttractiveChargeUnit(2.8),
            repulsive_charge: data_structure::RepulsiveChargeUnit(3.7),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(4.6),
                data_structure::GreenColorUnit(5.5),
                data_structure::BlueColorUnit(6.4),
            ),
        }
    }

    fn new_test_configuration(
        test_horizontal_displacement: serde_json::Value,
        test_vertical_displacement: serde_json::Value,
        test_horizontal_velocity: serde_json::Value,
        test_vertical_velocity: serde_json::Value,
        test_radius: serde_json::Value,
        test_rotation: serde_json::Value,
    ) -> serde_json::Value {
        let test_intrinsics = new_test_intrinsics();
        serde_json::json!({
            DISPLACEMENT_LABEL: {
                super::super::HORIZONTAL_LABEL: test_horizontal_displacement,
                super::super::VERTICAL_LABEL: test_vertical_displacement,
            },
            VELOCITY_LABEL: {
                super::super::HORIZONTAL_LABEL: test_horizontal_velocity,
                super::super::VERTICAL_LABEL: test_vertical_velocity,
            },
            RADIUS_LABEL: test_radius,
            ANGULAR_VELOCITY_LABEL: test_rotation,
            MASS_LABEL: test_intrinsics.inertial_mass.0,
            GRAV_LABEL: test_intrinsics.attractive_charge.0,
            BUMP_LABEL: test_intrinsics.repulsive_charge.0,
            RED_LABEL: test_intrinsics.color_brightness.get_red().0,
            GREEN_LABEL: test_intrinsics.color_brightness.get_green().0,
            BLUE_LABEL: test_intrinsics.color_brightness.get_blue().0,
        })
    }

    fn new_test_particle_at(
        horizontal_position: data_structure::HorizontalPositionUnit,
        vertical_position: data_structure::VerticalPositionUnit,
        horizontal_velocity: data_structure::HorizontalVelocityUnit,
        vertical_velocity: data_structure::VerticalVelocityUnit,
    ) -> data_structure::IndividualParticle {
        data_structure::IndividualParticle {
            intrinsic_values: new_test_intrinsics(),
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: horizontal_position,
                    vertical_component: vertical_position,
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: horizontal_velocity,
                    vertical_component: vertical_velocity,
                },
            },
        }
    }

    #[test]
    fn check_reject_when_missing_attribute() -> Result<(), String> {
        let required_attributes = vec![
            DISPLACEMENT_LABEL,
            VELOCITY_LABEL,
            ANGULAR_VELOCITY_LABEL,
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
            RADIUS_LABEL,
            POPULATION_LABEL,
            ANGULAR_VELOCITY_LABEL,
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
        let configuration_without_population = new_test_configuration(
            serde_json::json!(9001.0),
            serde_json::json!(9002.0),
            serde_json::json!(9003.0),
            serde_json::json!(9004.0),
            serde_json::json!(9005.0),
            serde_json::json!(9006.0),
        );
        let parsing_result = from_json(&configuration_without_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_population() -> Result<(), String> {
        let mut configuration_with_array_population = new_test_configuration(
            serde_json::json!(9001.0),
            serde_json::json!(9002.0),
            serde_json::json!(9003.0),
            serde_json::json!(9004.0),
            serde_json::json!(9005.0),
            serde_json::json!(9006.0),
        );
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
        let mut test_configuration = new_test_configuration(
            serde_json::json!(9001.0),
            serde_json::json!(9002.0),
            serde_json::json!(9003.0),
            serde_json::json!(9004.0),
            serde_json::json!(9005.0),
            serde_json::json!(9006.0),
        );
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
        let mut test_configuration = new_test_configuration(
            serde_json::json!(9001.0),
            serde_json::json!(9002.0),
            serde_json::json!(9003.0),
            serde_json::json!(9004.0),
            serde_json::json!(9005.0),
            serde_json::json!(9006.0),
        );
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
        let test_radius = 2.0;
        let test_angular_speed = 23.4;
        let test_linear_speed = test_angular_speed * test_radius;
        let test_horizontal_displacement = 1000.0;
        let test_vertical_displacement = 500.0;
        let mut test_configuration = new_test_configuration(
            serde_json::json!(test_horizontal_displacement),
            serde_json::json!(test_vertical_displacement),
            serde_json::json!(0.0),
            serde_json::json!(0.0),
            serde_json::json!(test_radius),
            serde_json::json!(test_angular_speed),
        );
        test_configuration[POPULATION_LABEL] = serde_json::json!(2);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement + test_radius),
                data_structure::VerticalPositionUnit(test_vertical_displacement),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement - test_radius),
                data_structure::VerticalPositionUnit(test_vertical_displacement),
                data_structure::HorizontalVelocityUnit(0.0),
                data_structure::VerticalVelocityUnit(-test_linear_speed),
            ),
        ];

        data_structure::comparison::unordered_particles_match_within_tolerance(
            &mut expected_particles.iter(),
            &mut generated_particles.iter(),
            &new_particle_tolerance(),
        )
    }

    #[test]
    fn check_parse_three_points() -> Result<(), String> {
        let test_radius = 1.0;
        let test_horizontal_velocity = 50.0;
        let test_vertical_velocity = 500.0;

        // This time we will have zero angular velocity to keep the calculation simple.
        let mut test_configuration = new_test_configuration(
            serde_json::json!(0.0),
            serde_json::json!(0.0),
            serde_json::json!(test_horizontal_velocity),
            serde_json::json!(test_vertical_velocity),
            serde_json::json!(test_radius),
            serde_json::json!(0.0),
        );
        test_configuration[POPULATION_LABEL] = serde_json::json!(3);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let left_vertical_magnitude = 0.866;
        let left_horizontal_coordinate = data_structure::HorizontalPositionUnit(-0.5);
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(1.0),
                data_structure::VerticalPositionUnit(0.0),
                data_structure::HorizontalVelocityUnit(test_horizontal_velocity),
                data_structure::VerticalVelocityUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::VerticalPositionUnit(left_vertical_magnitude),
                data_structure::HorizontalVelocityUnit(test_horizontal_velocity),
                data_structure::VerticalVelocityUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::VerticalPositionUnit(-left_vertical_magnitude),
                data_structure::HorizontalVelocityUnit(test_horizontal_velocity),
                data_structure::VerticalVelocityUnit(test_vertical_velocity),
            ),
        ];

        data_structure::comparison::unordered_particles_match_within_tolerance(
            &mut expected_particles.iter(),
            &mut generated_particles.iter(),
            &new_particle_tolerance(),
        )
    }

    #[test]
    fn check_parse_four_points() -> Result<(), String> {
        let test_radius = 2.5;
        let test_angular_speed = 0.1;
        let test_linear_speed = test_angular_speed * test_radius;
        let test_horizontal_displacement = 1000.0;
        let test_vertical_displacement = 500.0;
        let test_horizontal_velocity = 50.0;
        let test_vertical_velocity = 500.0;
        let mut test_configuration = new_test_configuration(
            serde_json::json!(test_horizontal_displacement),
            serde_json::json!(test_vertical_displacement),
            serde_json::json!(test_horizontal_velocity),
            serde_json::json!(test_vertical_velocity),
            serde_json::json!(test_radius),
            serde_json::json!(test_angular_speed),
        );
        test_configuration[POPULATION_LABEL] = serde_json::json!(4);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement + test_radius),
                data_structure::VerticalPositionUnit(test_vertical_displacement),
                data_structure::HorizontalVelocityUnit(test_horizontal_velocity),
                data_structure::VerticalVelocityUnit(test_vertical_velocity + test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement),
                data_structure::VerticalPositionUnit(test_vertical_displacement + test_radius),
                data_structure::HorizontalVelocityUnit(
                    test_horizontal_velocity - test_linear_speed,
                ),
                data_structure::VerticalVelocityUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement - test_radius),
                data_structure::VerticalPositionUnit(test_vertical_displacement),
                data_structure::HorizontalVelocityUnit(test_horizontal_velocity),
                data_structure::VerticalVelocityUnit(test_vertical_velocity - test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::HorizontalPositionUnit(test_horizontal_displacement),
                data_structure::VerticalPositionUnit(test_vertical_displacement - test_radius),
                data_structure::HorizontalVelocityUnit(
                    test_horizontal_velocity + test_linear_speed,
                ),
                data_structure::VerticalVelocityUnit(test_vertical_velocity),
            ),
        ];

        data_structure::comparison::unordered_particles_match_within_tolerance(
            &mut expected_particles.iter(),
            &mut generated_particles.iter(),
            &new_particle_tolerance(),
        )
    }
}
