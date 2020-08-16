/// This module provides a set of functions which each test a case for an implementation of
/// ParticlesInTimeEvolver, so that each implementation can simply wrap the call in an actual test,
/// passing in an instance of the implementation.
fn create_test_tolerances() -> data_structure::IndividualParticle {
    let absolute_tolerance = 0.000001;
    data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(absolute_tolerance),
            inverse_squared_charge: data_structure::InverseSquaredChargeUnit(absolute_tolerance),
            inverse_fourth_charge: data_structure::InverseFourthChargeUnit(absolute_tolerance),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(absolute_tolerance),
                data_structure::GreenColorUnit(absolute_tolerance),
                data_structure::BlueColorUnit(absolute_tolerance),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(absolute_tolerance),
                vertical_component: data_structure::VerticalPositionUnit(absolute_tolerance),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(absolute_tolerance),
                vertical_component: data_structure::VerticalVelocityUnit(absolute_tolerance),
            },
        },
    }
}

fn compare_time_slices_to_expected<T, U, V, W, X, Y>(
    evolution_result: Result<V, Box<dyn std::error::Error>>,
    expected_sequence: Y,
    tolerances_as_particle: &impl data_structure::ParticleRepresentation,
) -> Result<(), String>
where
    T: data_structure::ParticleRepresentation,
    U: std::iter::ExactSizeIterator<Item = T>,
    V: std::iter::ExactSizeIterator<Item = U>,
    W: data_structure::ParticleRepresentation,
    X: std::iter::ExactSizeIterator<Item = W>,
    Y: std::iter::ExactSizeIterator<Item = X>,
{
    match evolution_result {
        Ok(actual_sequence) => {
            let number_of_time_slices = expected_sequence.len();
            if actual_sequence.len() == number_of_time_slices {
                return data_structure::comparison::ordered_sequences_match_unordered_particles_within_tolerance(
                expected_sequence, actual_sequence, tolerances_as_particle);
            } else {
                return Err(String::from(format!(
                    "Expected length = {}, actual length = {}",
                    number_of_time_slices,
                    actual_sequence.len()
                )));
            }
        }
        Err(evolution_error) => Err(String::from(format!("{:?}", evolution_error))),
    }
}

pub fn test_single_particle_at_rest_stays_at_rest<T, U>(
    tested_implementation: &mut T,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let expected_particle = data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.0),
            inverse_squared_charge: data_structure::InverseSquaredChargeUnit(2.0),
            inverse_fourth_charge: data_structure::InverseFourthChargeUnit(3.0),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(4.0),
                data_structure::GreenColorUnit(5.0),
                data_structure::BlueColorUnit(6.0),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(7.8),
                vertical_component: data_structure::VerticalPositionUnit(9.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };

    let initial_conditions = vec![expected_particle];

    let number_of_time_slices: usize = 8;
    let mut expected_sequence: std::vec::Vec<
        std::vec::IntoIter<data_structure::IndividualParticle>,
    > = vec![];
    for _ in 0..number_of_time_slices {
        let unchanged_state: std::vec::Vec<data_structure::IndividualParticle> =
            vec![expected_particle];
        expected_sequence.push(unchanged_state.into_iter());
    }
    let evolution_result = tested_implementation
        .create_time_sequence(initial_conditions.into_iter(), number_of_time_slices);
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
    );
}

pub fn test_single_particle_at_constant_speed<T, U>(
    tested_implementation: &mut T,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let particle_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(2.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(3.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };
    let initial_particle = data_structure::IndividualParticle {
        intrinsic_values: particle_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(7.8),
                vertical_component: data_structure::VerticalPositionUnit(9.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                vertical_component: data_structure::VerticalVelocityUnit(-2.2),
            },
        },
    };
    let expected_sequence = vec![
        vec![initial_particle].into_iter(),
        vec![data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(8.1),
                    vertical_component: data_structure::VerticalPositionUnit(6.8),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        }]
        .into_iter(),
        vec![data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(8.4),
                    vertical_component: data_structure::VerticalPositionUnit(4.6),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        }]
        .into_iter(),
        vec![data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(8.7),
                    vertical_component: data_structure::VerticalPositionUnit(2.4),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        }]
        .into_iter(),
        vec![data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(9.0),
                    vertical_component: data_structure::VerticalPositionUnit(0.2),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        }]
        .into_iter(),
        vec![data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(9.3),
                    vertical_component: data_structure::VerticalPositionUnit(-2.0),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        }]
        .into_iter(),
    ];

    let initial_conditions: std::vec::Vec<data_structure::IndividualParticle> =
        vec![initial_particle];

    let number_of_time_slices = expected_sequence.len();
    let evolution_result = tested_implementation
        .create_time_sequence(initial_conditions.into_iter(), number_of_time_slices);
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
    );
}

pub fn test_uncharged_particles_do_not_accelerate<T, U>(
    tested_implementation: &mut T,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let particle_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(0.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(0.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };
    let immobile_particle = data_structure::IndividualParticle {
        intrinsic_values: particle_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(2.6),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let initial_conditions = vec![
        data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(0.0),
                    vertical_component: data_structure::VerticalPositionUnit(0.0),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(1.3),
                    vertical_component: data_structure::VerticalVelocityUnit(0.0),
                },
            },
        },
        immobile_particle.clone(),
        data_structure::IndividualParticle {
            intrinsic_values: particle_intrinsics,
            variable_values: data_structure::ParticleVariables {
                position_vector: data_structure::PositionVector {
                    horizontal_component: data_structure::HorizontalPositionUnit(7.8),
                    vertical_component: data_structure::VerticalPositionUnit(9.0),
                },
                velocity_vector: data_structure::VelocityVector {
                    horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                    vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                },
            },
        },
    ];
    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(1.3),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(1.3),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            immobile_particle.clone(),
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(8.1),
                        vertical_component: data_structure::VerticalPositionUnit(6.8),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                        vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                    },
                },
            },
        ]
        .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(2.6),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(1.3),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            immobile_particle.clone(),
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(8.4),
                        vertical_component: data_structure::VerticalPositionUnit(4.6),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                        vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                    },
                },
            },
        ]
        .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(3.9),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(1.3),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            immobile_particle.clone(),
            data_structure::IndividualParticle {
                intrinsic_values: particle_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(8.7),
                        vertical_component: data_structure::VerticalPositionUnit(2.4),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(0.3),
                        vertical_component: data_structure::VerticalVelocityUnit(-2.2),
                    },
                },
            },
        ]
        .into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();
    let evolution_result = tested_implementation
        .create_time_sequence(initial_conditions.into_iter(), number_of_time_slices);
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
    );
}
