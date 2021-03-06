/// This module provides a function to put particles evenly around a circle, with a common angular
/// speed around the center.
use super::configuration_parsing::ConfigurationParseError;
use std::convert::TryInto;

const COMMON_DISPLACEMENT_IN_PIXELS_LABEL: &str = "commonDisplacementInPixels";
const LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL: &str = "linearVelocityInPixelsPerSecond";
const RADIUS_IN_PIXELS_LABEL: &str = "radiusInPixels";
const TOTAL_PARTICLES_ON_CIRCLE_LABEL: &str = "totalParticlesOnCircle";
const ANGULAR_VELOCITY_IN_PIXEL_RADIANS_PER_SECOND_LABEL: &str =
    "angularVelocityInPixelRadiansPerSecond";
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
) -> Result<std::vec::Vec<data_structure::particle::BasicIndividual>, Box<dyn std::error::Error>> {
    let circle_displacement =
        super::parse_position(&given_configuration[COMMON_DISPLACEMENT_IN_PIXELS_LABEL])?;
    let circle_velocity =
        super::parse_velocity(&given_configuration[LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL])?;
    let circle_radius =
        super::configuration_parsing::parse_f64(RADIUS_IN_PIXELS_LABEL, given_configuration)?;
    let circle_population = super::configuration_parsing::parse_i64(
        TOTAL_PARTICLES_ON_CIRCLE_LABEL,
        given_configuration,
    )?;
    let circle_rotation = super::configuration_parsing::parse_f64(
        ANGULAR_VELOCITY_IN_PIXEL_RADIANS_PER_SECOND_LABEL,
        given_configuration,
    )?;
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
    let common_intrinsics = data_structure::particle::IntrinsicPart {
        inertial_mass: data_structure::charge::InertialMassUnit(inertial_mass),
        inverse_squared_charge: data_structure::charge::InverseSquaredChargeUnit(
            inverse_squared_charge,
        ),
        inverse_fourth_charge: data_structure::charge::InverseFourthChargeUnit(
            inverse_fourth_charge,
        ),
        color_brightness: data_structure::color::new_triplet(
            data_structure::color::RedUnit(red_brightness),
            data_structure::color::GreenUnit(green_brightness),
            data_structure::color::BlueUnit(blue_brightness),
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
    circle_displacement: data_structure::position::DimensionfulVector,
    circle_velocity: data_structure::velocity::DimensionfulVector,
    circle_radius: f64,
    circle_population: i64,
    angular_velocity: f64,
    common_intrinsics: data_structure::particle::IntrinsicPart,
) -> Result<std::vec::Vec<data_structure::particle::BasicIndividual>, Box<dyn std::error::Error>> {
    if circle_population < 2 {
        return Err(Box::new(ConfigurationParseError::new(&format!(
            "Population {} is not large enough (must be 2 or larger)",
            circle_population
        ))));
    }

    let mut circle_particles: std::vec::Vec<data_structure::particle::BasicIndividual> =
        std::vec::Vec::with_capacity(circle_population.try_into()?);

    // We always start with a particle at 0 radians from the positive x axis.
    circle_particles.push(data_structure::particle::BasicIndividual {
        intrinsic_values: common_intrinsics,
        variable_values: data_structure::particle::VariablePart {
            position_vector: data_structure::position::DimensionfulVector {
                horizontal_component: data_structure::position::HorizontalUnit(circle_radius)
                    + circle_displacement.horizontal_component,
                vertical_component: data_structure::position::VerticalUnit(0.0)
                    + circle_displacement.vertical_component,
            },
            velocity_vector: data_structure::velocity::DimensionfulVector {
                horizontal_component: data_structure::velocity::HorizontalUnit(0.0)
                    + circle_velocity.horizontal_component,
                vertical_component: data_structure::velocity::VerticalUnit(
                    circle_radius * angular_velocity,
                ) + circle_velocity.vertical_component,
            },
        },
    });

    if (circle_population % 2) == 0 {
        // If the number of particles is even, then there is a particle at pi radians from the
        // positive x axis.
        circle_particles.push(data_structure::particle::BasicIndividual {
            intrinsic_values: common_intrinsics,
            variable_values: data_structure::particle::VariablePart {
                position_vector: data_structure::position::DimensionfulVector {
                    horizontal_component: data_structure::position::HorizontalUnit(-circle_radius)
                        + circle_displacement.horizontal_component,
                    vertical_component: data_structure::position::VerticalUnit(0.0)
                        + circle_displacement.vertical_component,
                },
                velocity_vector: data_structure::velocity::DimensionfulVector {
                    horizontal_component: data_structure::velocity::HorizontalUnit(0.0)
                        + circle_velocity.horizontal_component,
                    vertical_component: data_structure::velocity::VerticalUnit(
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

            circle_particles.push(data_structure::particle::BasicIndividual {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::particle::VariablePart {
                    position_vector: data_structure::position::DimensionfulVector {
                        horizontal_component: data_structure::position::HorizontalUnit(
                            cosine_of_angle_times_radius,
                        ) + circle_displacement.horizontal_component,
                        vertical_component: data_structure::position::VerticalUnit(
                            sine_of_angle_times_radius,
                        ) + circle_displacement.vertical_component,
                    },
                    velocity_vector: data_structure::velocity::DimensionfulVector {
                        horizontal_component: data_structure::velocity::HorizontalUnit(
                            -sine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.horizontal_component,
                        vertical_component: data_structure::velocity::VerticalUnit(
                            cosine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.vertical_component,
                    },
                },
            });

            circle_particles.push(data_structure::particle::BasicIndividual {
                intrinsic_values: common_intrinsics,
                variable_values: data_structure::particle::VariablePart {
                    position_vector: data_structure::position::DimensionfulVector {
                        horizontal_component: data_structure::position::HorizontalUnit(
                            cosine_of_angle_times_radius,
                        ) + circle_displacement.horizontal_component,
                        vertical_component: data_structure::position::VerticalUnit(
                            -sine_of_angle_times_radius,
                        ) + circle_displacement.vertical_component,
                    },
                    velocity_vector: data_structure::velocity::DimensionfulVector {
                        horizontal_component: data_structure::velocity::HorizontalUnit(
                            sine_of_angle_times_radius * angular_velocity,
                        ) + circle_velocity.horizontal_component,
                        vertical_component: data_structure::velocity::VerticalUnit(
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

    fn new_intrinsics_tolerance() -> data_structure::particle::IntrinsicPart {
        data_structure::particle::IntrinsicPart {
            inertial_mass: data_structure::charge::InertialMassUnit(0.01),
            inverse_squared_charge: data_structure::charge::InverseSquaredChargeUnit(0.01),
            inverse_fourth_charge: data_structure::charge::InverseFourthChargeUnit(0.01),
            color_brightness: data_structure::color::new_triplet(
                data_structure::color::RedUnit(0.01),
                data_structure::color::GreenUnit(0.01),
                data_structure::color::BlueUnit(0.01),
            ),
        }
    }

    fn new_variables_tolerance() -> data_structure::particle::VariablePart {
        data_structure::particle::VariablePart {
            position_vector: data_structure::position::DimensionfulVector {
                horizontal_component: data_structure::position::HorizontalUnit(0.01),
                vertical_component: data_structure::position::VerticalUnit(0.01),
            },
            velocity_vector: data_structure::velocity::DimensionfulVector {
                horizontal_component: data_structure::velocity::HorizontalUnit(0.01),
                vertical_component: data_structure::velocity::VerticalUnit(0.01),
            },
        }
    }

    fn new_particle_tolerance() -> data_structure::particle::BasicIndividual {
        data_structure::particle::BasicIndividual {
            intrinsic_values: new_intrinsics_tolerance(),
            variable_values: new_variables_tolerance(),
        }
    }

    fn new_test_intrinsics() -> data_structure::particle::IntrinsicPart {
        data_structure::particle::IntrinsicPart {
            inertial_mass: data_structure::charge::InertialMassUnit(1.9),
            inverse_squared_charge: data_structure::charge::InverseSquaredChargeUnit(2.8),
            inverse_fourth_charge: data_structure::charge::InverseFourthChargeUnit(3.7),
            color_brightness: data_structure::color::new_triplet(
                data_structure::color::RedUnit(4.6),
                data_structure::color::GreenUnit(5.5),
                data_structure::color::BlueUnit(6.4),
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
            COMMON_DISPLACEMENT_IN_PIXELS_LABEL: {
                super::super::HORIZONTAL_LABEL: test_horizontal_displacement,
                super::super::VERTICAL_LABEL: test_vertical_displacement,
            },
            LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL: {
                super::super::HORIZONTAL_LABEL: test_horizontal_velocity,
                super::super::VERTICAL_LABEL: test_vertical_velocity,
            },
            RADIUS_IN_PIXELS_LABEL: test_radius,
            ANGULAR_VELOCITY_IN_PIXEL_RADIANS_PER_SECOND_LABEL: test_rotation,
            INERTIAL_MASS_IN_MASS_UNITS_LABEL: test_intrinsics.inertial_mass.0,
            INVERSE_SQUARED_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL: test_intrinsics.inverse_squared_charge.0,
            INVERSE_FOURTH_CHARGE_IN_DIMENSIONLESS_UNITS_LABEL: test_intrinsics.inverse_fourth_charge.0,
            RED_PIXEL_STRENGTH_LABEL: test_intrinsics.color_brightness.get_red().0,
            GREEN_PIXEL_STRENGTH_LABEL: test_intrinsics.color_brightness.get_green().0,
            BLUE_PIXEL_STRENGTH_LABEL: test_intrinsics.color_brightness.get_blue().0,
        })
    }

    fn new_test_particle_at(
        horizontal_position: data_structure::position::HorizontalUnit,
        vertical_position: data_structure::position::VerticalUnit,
        horizontal_velocity: data_structure::velocity::HorizontalUnit,
        vertical_velocity: data_structure::velocity::VerticalUnit,
    ) -> data_structure::particle::BasicIndividual {
        data_structure::particle::BasicIndividual {
            intrinsic_values: new_test_intrinsics(),
            variable_values: data_structure::particle::VariablePart {
                position_vector: data_structure::position::DimensionfulVector {
                    horizontal_component: horizontal_position,
                    vertical_component: vertical_position,
                },
                velocity_vector: data_structure::velocity::DimensionfulVector {
                    horizontal_component: horizontal_velocity,
                    vertical_component: vertical_velocity,
                },
            },
        }
    }

    #[test]
    fn check_reject_when_missing_attribute() -> Result<(), String> {
        let required_attributes = vec![
            COMMON_DISPLACEMENT_IN_PIXELS_LABEL,
            LINEAR_VELOCITY_IN_PIXELS_PER_SECOND_LABEL,
            ANGULAR_VELOCITY_IN_PIXEL_RADIANS_PER_SECOND_LABEL,
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
            RADIUS_IN_PIXELS_LABEL,
            TOTAL_PARTICLES_ON_CIRCLE_LABEL,
            ANGULAR_VELOCITY_IN_PIXEL_RADIANS_PER_SECOND_LABEL,
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
        configuration_with_array_population[TOTAL_PARTICLES_ON_CIRCLE_LABEL] =
            serde_json::json!([9001.0, 9002.0]);
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
        test_configuration[TOTAL_PARTICLES_ON_CIRCLE_LABEL] = serde_json::json!(0);
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
        test_configuration[TOTAL_PARTICLES_ON_CIRCLE_LABEL] = serde_json::json!(1);
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
        test_configuration[TOTAL_PARTICLES_ON_CIRCLE_LABEL] = serde_json::json!(2);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::position::HorizontalUnit(
                    test_horizontal_displacement + test_radius,
                ),
                data_structure::position::VerticalUnit(test_vertical_displacement),
                data_structure::velocity::HorizontalUnit(0.0),
                data_structure::velocity::VerticalUnit(test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::position::HorizontalUnit(
                    test_horizontal_displacement - test_radius,
                ),
                data_structure::position::VerticalUnit(test_vertical_displacement),
                data_structure::velocity::HorizontalUnit(0.0),
                data_structure::velocity::VerticalUnit(-test_linear_speed),
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
        test_configuration[TOTAL_PARTICLES_ON_CIRCLE_LABEL] = serde_json::json!(3);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let left_vertical_magnitude = 0.866;
        let left_horizontal_coordinate = data_structure::position::HorizontalUnit(-0.5);
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::position::HorizontalUnit(1.0),
                data_structure::position::VerticalUnit(0.0),
                data_structure::velocity::HorizontalUnit(test_horizontal_velocity),
                data_structure::velocity::VerticalUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::position::VerticalUnit(left_vertical_magnitude),
                data_structure::velocity::HorizontalUnit(test_horizontal_velocity),
                data_structure::velocity::VerticalUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                left_horizontal_coordinate,
                data_structure::position::VerticalUnit(-left_vertical_magnitude),
                data_structure::velocity::HorizontalUnit(test_horizontal_velocity),
                data_structure::velocity::VerticalUnit(test_vertical_velocity),
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
        test_configuration[TOTAL_PARTICLES_ON_CIRCLE_LABEL] = serde_json::json!(4);
        let generated_particles =
            from_json(&test_configuration).expect("Valid configuration should be parsed.");
        let expected_particles = vec![
            new_test_particle_at(
                data_structure::position::HorizontalUnit(
                    test_horizontal_displacement + test_radius,
                ),
                data_structure::position::VerticalUnit(test_vertical_displacement),
                data_structure::velocity::HorizontalUnit(test_horizontal_velocity),
                data_structure::velocity::VerticalUnit(test_vertical_velocity + test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::position::HorizontalUnit(test_horizontal_displacement),
                data_structure::position::VerticalUnit(test_vertical_displacement + test_radius),
                data_structure::velocity::HorizontalUnit(
                    test_horizontal_velocity - test_linear_speed,
                ),
                data_structure::velocity::VerticalUnit(test_vertical_velocity),
            ),
            new_test_particle_at(
                data_structure::position::HorizontalUnit(
                    test_horizontal_displacement - test_radius,
                ),
                data_structure::position::VerticalUnit(test_vertical_displacement),
                data_structure::velocity::HorizontalUnit(test_horizontal_velocity),
                data_structure::velocity::VerticalUnit(test_vertical_velocity - test_linear_speed),
            ),
            new_test_particle_at(
                data_structure::position::HorizontalUnit(test_horizontal_displacement),
                data_structure::position::VerticalUnit(test_vertical_displacement - test_radius),
                data_structure::velocity::HorizontalUnit(
                    test_horizontal_velocity + test_linear_speed,
                ),
                data_structure::velocity::VerticalUnit(test_vertical_velocity),
            ),
        ];

        data_structure::comparison::unordered_particles_match_within_tolerance(
            &mut expected_particles.iter(),
            &mut generated_particles.iter(),
            &new_particle_tolerance(),
        )
    }
}
