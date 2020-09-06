/// This module provides a set of functions which each test a case for an implementation of
/// ParticlesInTimeEvolver, so that each implementation can simply wrap the call in an actual test,
/// passing in an instance of the implementation.
const TEST_DEFAULT_TOLERANCE: f64 = 0.01;

const NO_ADDITIONAL_CHECK: Option<
    fn(&std::vec::Vec<data_structure::IndividualParticle>) -> Result<(), String>,
> = None;

fn create_test_tolerance_with_separate_for_values(
    horizontal_position_tolerance: f64,
    vertical_position_tolerance: f64,
    horizontal_velocity_tolerance: f64,
    vertical_velocity_tolerance: f64,
) -> data_structure::IndividualParticle {
    data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(TEST_DEFAULT_TOLERANCE),
            inverse_squared_charge: data_structure::InverseSquaredChargeUnit(
                TEST_DEFAULT_TOLERANCE,
            ),
            inverse_fourth_charge: data_structure::InverseFourthChargeUnit(TEST_DEFAULT_TOLERANCE),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(TEST_DEFAULT_TOLERANCE),
                data_structure::GreenColorUnit(TEST_DEFAULT_TOLERANCE),
                data_structure::BlueColorUnit(TEST_DEFAULT_TOLERANCE),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(
                    horizontal_position_tolerance,
                ),
                vertical_component: data_structure::VerticalPositionUnit(
                    vertical_position_tolerance,
                ),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(
                    horizontal_velocity_tolerance,
                ),
                vertical_component: data_structure::VerticalVelocityUnit(
                    vertical_velocity_tolerance,
                ),
            },
        },
    }
}

fn create_test_tolerances() -> data_structure::IndividualParticle {
    create_test_tolerance_with_separate_for_values(
        TEST_DEFAULT_TOLERANCE,
        TEST_DEFAULT_TOLERANCE,
        TEST_DEFAULT_TOLERANCE,
        TEST_DEFAULT_TOLERANCE,
    )
}

/// It is easiest to work out expected values for whole 1-second time slices, so 1000 milliseconds.
fn create_test_evolution_configuration(
    number_of_time_slices: usize,
) -> super::configuration_parsing::EvolutionConfiguration {
    super::configuration_parsing::EvolutionConfiguration {
        dead_zone_radius: 1.0,
        inverse_squared_coupling: -1.0,
        inverse_fourth_coupling: 1.0,
        milliseconds_per_time_slice: 1000,
        number_of_time_slices: number_of_time_slices,
    }
}

fn apply_check_then_compare_time_slices<T, U, V, W, X, Y, Z>(
    actual_sequence: V,
    expected_sequence: Y,
    tolerances_as_particle: &impl data_structure::ParticleRepresentation,
    additional_check: Z,
) -> Result<(), String>
where
    T: data_structure::ParticleRepresentation,
    U: std::iter::ExactSizeIterator<Item = T>,
    V: std::iter::ExactSizeIterator<Item = U>,
    W: data_structure::ParticleRepresentation,
    X: std::iter::ExactSizeIterator<Item = W>,
    Y: std::iter::ExactSizeIterator<Item = X>,
    Z: Fn(&std::vec::Vec<data_structure::IndividualParticle>) -> Result<(), String>,
{
    let mut copied_sequence = vec![];

    for actual_time_slice in actual_sequence {
        let copied_time_slice: std::vec::Vec<data_structure::IndividualParticle> =
            actual_time_slice
                .map(|x| data_structure::create_individual_from_representation(&x))
                .collect();

        let additional_check_result = additional_check(&copied_time_slice);

        if additional_check_result.is_err() {
            return additional_check_result;
        }

        copied_sequence.push(copied_time_slice.into_iter());
    }
    return data_structure::comparison::ordered_sequences_match_unordered_particles(
        expected_sequence,
        copied_sequence.into_iter(),
        tolerances_as_particle,
    );
}

/// The optional additional check needs to operate on a Vec of IndividualParticle because that is
/// what will hold a copy of the actual time slice and will be passed as the parameter. The actual
/// functions passed as optional_additional_check are free to use trait bounds or whatever.
fn compare_time_slices_to_expected<T, U, V, W, X, Y, Z>(
    evolution_result: Result<super::ParticleSetEvolution<T, U, V>, Box<dyn std::error::Error>>,
    expected_sequence: Y,
    tolerances_as_particle: &impl data_structure::ParticleRepresentation,
    optional_additional_check: Option<Z>,
) -> Result<(), String>
where
    T: data_structure::ParticleRepresentation,
    U: std::iter::ExactSizeIterator<Item = T>,
    V: std::iter::ExactSizeIterator<Item = U>,
    W: data_structure::ParticleRepresentation,
    X: std::iter::ExactSizeIterator<Item = W>,
    Y: std::iter::ExactSizeIterator<Item = X>,
    Z: Fn(&std::vec::Vec<data_structure::IndividualParticle>) -> Result<(), String>,
{
    match evolution_result {
        Ok(actual_evolution) => {
            let number_of_time_slices = expected_sequence.len();
            let actual_sequence = actual_evolution.particle_configurations;
            if actual_sequence.len() == number_of_time_slices {
                if let Some(additional_check) = optional_additional_check {
                    return apply_check_then_compare_time_slices(
                        actual_sequence,
                        expected_sequence,
                        tolerances_as_particle,
                        additional_check,
                    );
                } else {
                    return data_structure::comparison::ordered_sequences_match_unordered_particles(
                        expected_sequence,
                        actual_sequence,
                        tolerances_as_particle,
                    );
                }
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

trait PotentialEnergyCalculator {
    fn total_for_both(
        &self,
        first_particle: &impl data_structure::ParticleRepresentation,
        second_particle: &impl data_structure::ParticleRepresentation,
    ) -> Result<f64, String>;
}

#[derive(Clone, Copy, Debug)]
struct InverseSquaredAndFourthPotential {
    inverse_squared_coupling_constant: f64,
    inverse_fourth_coupling_constant: f64,
    dead_zone_radius: data_structure::SeparationUnit,
}

impl PotentialEnergyCalculator for InverseSquaredAndFourthPotential {
    fn total_for_both(
        &self,
        first_particle: &impl data_structure::ParticleRepresentation,
        second_particle: &impl data_structure::ParticleRepresentation,
    ) -> Result<f64, String> {
        let inverse_separation = data_structure::get_capped_inverse_separation(
            &first_particle.read_variables().position_vector,
            &second_particle.read_variables().position_vector,
            &self.dead_zone_radius,
        );

        // The potential energy for the pair is the integral over total separation of the force
        // felt by one of the particles. Hence for the inverse-fourth part, the 3 factors of the
        // inverse separation and a division by 3, while the inverse-square part just has a single
        // inverse power of the separation and no extra factor (because it is 1).
        // (Equivalently, the potential energy is the sum of the integrals of both forces over the
        // parts of the separations divided between the particles according to mass.)
        let inverse_fourth_part = (self.inverse_fourth_coupling_constant
            * first_particle.read_intrinsics().inverse_fourth_charge.0
            * second_particle.read_intrinsics().inverse_fourth_charge.0
            * inverse_separation.get_value()
            * inverse_separation.get_value()
            * inverse_separation.get_value())
            / 3.0;
        let inverse_square_part = self.inverse_squared_coupling_constant
            * first_particle.read_intrinsics().inverse_squared_charge.0
            * second_particle.read_intrinsics().inverse_squared_charge.0
            * inverse_separation.get_value();
        Ok(inverse_fourth_part + inverse_square_part)
    }
}

fn check_energy_given_potential(
    expected_number_of_particles: usize,
    expected_energy_in_implicit_units: f64,
    relative_tolerance: f64,
    particle_list: &std::vec::Vec<impl data_structure::ParticleRepresentation>,
    potential_energy_of_pair: impl PotentialEnergyCalculator,
) -> Result<(), String> {
    if particle_list.len() != expected_number_of_particles {
        return Err(String::from(format!(
            "Expected exactly {} particles for checking energy, instead received {}",
            expected_number_of_particles,
            particle_list.len()
        )));
    }
    let mut total_energy = 0.0;
    for particle_index in 0..expected_number_of_particles {
        let current_particle = &particle_list[particle_index];
        let current_variables = current_particle.read_variables();
        let current_kinetic = 0.5
            * current_particle.read_intrinsics().inertial_mass.0
            * ((current_variables.velocity_vector.horizontal_component.0
                * current_variables.velocity_vector.horizontal_component.0)
                + (current_variables.velocity_vector.vertical_component.0
                    * current_variables.velocity_vector.vertical_component.0));
        total_energy += current_kinetic;
        for other_index in (particle_index + 1)..expected_number_of_particles {
            let other_particle = &particle_list[other_index];
            total_energy +=
                potential_energy_of_pair.total_for_both(current_particle, other_particle)?;
        }
    }

    if !data_structure::comparison::within_relative_tolerance(
        expected_energy_in_implicit_units,
        total_energy,
        relative_tolerance,
    ) {
        Err(String::from(format!(
            "Expected energy = {}, actual energy = {}",
            expected_energy_in_implicit_units, total_energy
        )))
    } else {
        Ok(())
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
    let evolution_configuration = create_test_evolution_configuration(number_of_time_slices);
    let mut expected_sequence: std::vec::Vec<
        std::vec::IntoIter<data_structure::IndividualParticle>,
    > = vec![];
    for _ in 0..number_of_time_slices {
        let unchanged_state: std::vec::Vec<data_structure::IndividualParticle> =
            vec![expected_particle];
        expected_sequence.push(unchanged_state.into_iter());
    }
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        NO_ADDITIONAL_CHECK,
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
    let evolution_configuration = create_test_evolution_configuration(number_of_time_slices);
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        NO_ADDITIONAL_CHECK,
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
    let evolution_configuration = create_test_evolution_configuration(number_of_time_slices);
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        NO_ADDITIONAL_CHECK,
    );
}

pub fn test_immobile_repelling_particles_within_dead_zone_stay_at_rest<T, U>(
    tested_implementation: &mut T,
    dead_zone_radius: &data_structure::SeparationUnit,
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
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(1.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };

    let left_particle = data_structure::IndividualParticle {
        intrinsic_values: particle_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(
                    0.2 * dead_zone_radius.0,
                ),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let right_particle = data_structure::IndividualParticle {
        intrinsic_values: particle_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(
                    0.7 * dead_zone_radius.0,
                ),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };

    let initial_conditions = vec![left_particle.clone(), right_particle.clone()];
    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        vec![left_particle.clone(), right_particle.clone()].into_iter(),
        vec![left_particle.clone(), right_particle.clone()].into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();
    let evolution_configuration = create_test_evolution_configuration(number_of_time_slices);
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());
    let test_tolerances = create_test_tolerances();
    return compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        NO_ADDITIONAL_CHECK,
    );
}

/// This test checks against a special case where there is an analytical solution for the motion of
/// two equal masses under an attractive inverse-fourth force which have just enough kinetic energy
/// to come to rest infinitely far apart from each other.
pub fn test_equal_masses_attracting_inverse_fourth_critical_escape<T, U>(
    tested_implementation: &mut T,
    dead_zone_radius: &data_structure::SeparationUnit,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let test_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(0.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(1.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };

    // We work backwards from a nice solution for the horizontal displacement from the origin of
    // the particle on the right, denoted as x, thus with the particle on the left at -x.
    // The solution is x = t^(2/5).
    // Then v = dx/dt = (2/5) t^(-3/5) and a = dv/dt = (-6/25) t^(-8/5) = (-6/25) x^(-4).
    // In terms of the separation r = 2x and mass m, the force is ma = (-96m/25) r^(-4).
    // The test starts at t = 1, and it doesn't actually matter what m is as long as it is not 0.
    // Hence x = 1.0, so the particles are at +1.0 and at -1.0, and the velocities are +0.2 and
    // -0.2 respectively.
    let left_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(-1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(-0.4),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let right_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.4),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let initial_conditions = vec![left_particle.clone(), right_particle.clone()];

    let second_right_position = (2.0_f64).powf(0.4);
    let second_right_speed = 0.4 * (2.0_f64).powf(-0.6);
    let third_right_position = (3.0_f64).powf(0.4);
    let third_right_speed = 0.4 * (3.0_f64).powf(-0.6);
    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            -second_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            -second_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            second_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            second_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
        ]
        .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            -third_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            -third_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            third_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            third_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
        ]
        .into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();

    // As calculated above, the force each particle experiences is (-96m/25) r^(-4) and each has
    // mass 1.0 in this test.
    let evolution_configuration = super::configuration_parsing::EvolutionConfiguration {
        dead_zone_radius: dead_zone_radius.0,
        inverse_squared_coupling: 0.0,
        inverse_fourth_coupling: -3.84,
        milliseconds_per_time_slice: 1000,
        number_of_time_slices: number_of_time_slices,
    };
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());

    let inverse_fourth_potential_of_pair = InverseSquaredAndFourthPotential {
        inverse_squared_coupling_constant: 0.0,
        inverse_fourth_coupling_constant: evolution_configuration.inverse_fourth_coupling,
        dead_zone_radius: *dead_zone_radius,
    };

    let test_tolerances = create_test_tolerances();
    // The total energy should be 0.0 constantly.
    compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        Some(
            |particle_list: &std::vec::Vec<data_structure::IndividualParticle>| {
                check_energy_given_potential(
                    2,
                    0.0,
                    TEST_DEFAULT_TOLERANCE,
                    particle_list,
                    inverse_fourth_potential_of_pair,
                )
            },
        ),
    )
}

pub fn test_equal_masses_repelling_inverse_fourth_accelerate_away_equally<T, U>(
    tested_implementation: &mut T,
    dead_zone_radius: &data_structure::SeparationUnit,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let left_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(0.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(1.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };
    let left_particle = data_structure::IndividualParticle {
        intrinsic_values: left_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(2.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let right_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(0.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(2.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };
    let right_particle = data_structure::IndividualParticle {
        intrinsic_values: right_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(5.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };

    // Exactly solving d^2r/dt^2 = constant * r^(-p) where p is 2 or 4 is not easy when I want
    // dr/dt = 0 at t = 0. Hence we use a tolerance in positions which determines that they have
    // accelerated away from each other less than if the force remained constant, but more than
    // if the force dropped off very quickly. Other quantities can be checked more precisely,
    // like conservation of energy. Also, the circular orbit test can check positions at given
    // times against exact solutions.
    let initial_conditions = vec![left_particle.clone(), right_particle.clone()];

    // The force will drop off as the particles separate, so the initial force is an upper bound
    // on the force that they will experience.
    // Therefore the distance under that force for one second provides an upper bound on the
    // distance that each particle should travel in one second.
    // The initial force is (coupling=100)*(product of charges=1*2)*(initial distance=3)^(-4)
    // = 200/81, and since the mass value of each is 1, the acceleration upper bound is
    // 200/81 pixels seconds^(-2), so in 1 second the particle's travel distance is bounded from
    // above by 100/81.
    // The force which would be experienced at twice this separation (as both particles are moving)
    // plus the initial separation is a lower bound on the force experienced by the particles as
    // their separation will always be less than that. The lower bound on the force and thus
    // acceleration is then 200*([3 + (2*(100/81))]^(-4)), so the lower bound on how far each
    // particle will travel is 100*([3 + (200/81)]^(-4)).
    let upper_bound_on_final_speed = 200.0 / 81.0;
    let upper_bound_on_travel_distance = 0.5 * upper_bound_on_final_speed;
    let upper_bound_on_separation = 3.0 + (2.0 * upper_bound_on_travel_distance);
    let lower_bound_on_final_speed = 200.0
        / (upper_bound_on_separation
            * upper_bound_on_separation
            * upper_bound_on_separation
            * upper_bound_on_separation);
    let lower_bound_on_travel_distance = 0.5 * lower_bound_on_final_speed;
    let mean_of_speed_bounds = 0.5 * (upper_bound_on_final_speed + lower_bound_on_final_speed);
    let half_of_speed_range = 0.5 * (upper_bound_on_final_speed - lower_bound_on_final_speed);

    // Since the bounds on the travel distance are exactly half of the bounds on the final speed,
    // the mean and range could also be obtained by multplying by half.
    let mean_of_travel_bounds =
        0.5 * (upper_bound_on_travel_distance + lower_bound_on_travel_distance);
    let half_of_travel_range =
        0.5 * (upper_bound_on_travel_distance - lower_bound_on_travel_distance);
    let test_tolerances = create_test_tolerance_with_separate_for_values(
        half_of_travel_range,
        TEST_DEFAULT_TOLERANCE,
        half_of_speed_range,
        TEST_DEFAULT_TOLERANCE,
    );
    let mean_of_right_travel_bounds_as_position = data_structure::PositionVector {
        horizontal_component: data_structure::HorizontalPositionUnit(mean_of_travel_bounds),
        vertical_component: data_structure::VerticalPositionUnit(0.0),
    };
    let mut left_mean_of_position_bounds = left_particle.variable_values.position_vector.clone();
    left_mean_of_position_bounds -= mean_of_right_travel_bounds_as_position;
    let mut right_mean_of_position_bounds = right_particle.variable_values.position_vector.clone();
    right_mean_of_position_bounds += mean_of_right_travel_bounds_as_position;
    let mean_of_right_speed_bounds_as_velocity = data_structure::VelocityVector {
        horizontal_component: data_structure::HorizontalVelocityUnit(mean_of_speed_bounds),
        vertical_component: data_structure::VerticalVelocityUnit(0.0),
    };
    let mut left_mean_of_velocity_bounds = left_particle.variable_values.velocity_vector.clone();
    left_mean_of_velocity_bounds -= mean_of_right_speed_bounds_as_velocity;
    let mut right_mean_of_velocity_bounds = right_particle.variable_values.velocity_vector.clone();
    right_mean_of_velocity_bounds += mean_of_right_speed_bounds_as_velocity;
    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: left_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: left_mean_of_position_bounds,
                    velocity_vector: left_mean_of_velocity_bounds,
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: right_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: right_mean_of_position_bounds,
                    velocity_vector: right_mean_of_velocity_bounds,
                },
            },
        ]
        .into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();
    let evolution_configuration = super::configuration_parsing::EvolutionConfiguration {
        dead_zone_radius: dead_zone_radius.0,
        inverse_squared_coupling: 0.0,
        inverse_fourth_coupling: 100.0,
        milliseconds_per_time_slice: 1000,
        number_of_time_slices: number_of_time_slices,
    };
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());

    // The potential energy is (r/3)*(force per particle) = 200/81 in total.
    let inverse_fourth_potential_of_pair = InverseSquaredAndFourthPotential {
        inverse_squared_coupling_constant: 0.0,
        inverse_fourth_coupling_constant: evolution_configuration.inverse_fourth_coupling,
        dead_zone_radius: *dead_zone_radius,
    };

    let initial_energy =
        inverse_fourth_potential_of_pair.total_for_both(&left_particle, &right_particle)?;

    // The initial potential should be 200/81 in whatever units it works out as (as explained
    // above), and there is zero initial kinetic energy.
    let expected_initial_energy = 200.0 / 81.0;

    if !data_structure::comparison::within_relative_tolerance(
        expected_initial_energy,
        initial_energy,
        TEST_DEFAULT_TOLERANCE,
    ) {
        return Err(String::from(format!(
            "Expected inital energy = {}, actual inital energy = {}",
            expected_initial_energy, initial_energy
        )));
    }
    compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        Some(
            |particle_list: &std::vec::Vec<data_structure::IndividualParticle>| {
                check_energy_given_potential(
                    2,
                    expected_initial_energy,
                    TEST_DEFAULT_TOLERANCE,
                    particle_list,
                    inverse_fourth_potential_of_pair,
                )
            },
        ),
    )
}

/// This test checks against a special case where there is an analytical solution for the motion of
/// two equal masses under an attractive inverse-square force which have just enough kinetic energy
/// to come to rest infinitely far apart from each other. (So it is the same as
/// test_equal_masses_attracting_inverse_fourth_critical_escape above but for an inverse-square
/// force instead of inverse-fourth.)
pub fn test_equal_masses_attracting_inverse_square_critical_escape<T, U>(
    tested_implementation: &mut T,
    dead_zone_radius: &data_structure::SeparationUnit,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let test_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(1.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(0.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };

    // The details of the calculation are as above in
    // test_equal_masses_attracting_inverse_fourth_critical_escape
    // The solution in this case though is x = t^(2/3).
    // Then v = dx/dt = (2/3) t^(-1/3) and a = dv/dt = (-2/9) t^(-4/3) = (-2/9) x^(-2).
    // In terms of the separation r = 2x and mass m, the force is ma = (-8m/9) r^(-2).
    // The test starts at t = 1, and it doesn't actually matter what m is as long as it is not 0.
    // Hence x = 1.0, so the particles are at +1.0 and at -1.0, and the velocities are +2/3 and
    // -12/3 respectively.
    let left_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(-1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(-2.0 / 3.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let right_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(2.0 / 3.0),
                vertical_component: data_structure::VerticalVelocityUnit(0.0),
            },
        },
    };
    let initial_conditions = vec![left_particle.clone(), right_particle.clone()];

    let second_right_position = 2.0_f64.powf(2.0 / 3.0);
    let second_right_speed = (2.0 / 3.0) * 2.0_f64.powf(-1.0 / 3.0);
    let third_right_position = (3.0_f64).powf(2.0 / 3.0);
    let third_right_speed = (2.0 / 3.0) * (3.0_f64).powf(-1.0 / 3.0);
    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            -second_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            -second_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            second_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            second_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
        ]
        .into_iter(),
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            -third_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            -third_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: test_intrinsics,
                variable_values: data_structure::ParticleVariables {
                    position_vector: data_structure::PositionVector {
                        horizontal_component: data_structure::HorizontalPositionUnit(
                            third_right_position,
                        ),
                        vertical_component: data_structure::VerticalPositionUnit(0.0),
                    },
                    velocity_vector: data_structure::VelocityVector {
                        horizontal_component: data_structure::HorizontalVelocityUnit(
                            third_right_speed,
                        ),
                        vertical_component: data_structure::VerticalVelocityUnit(0.0),
                    },
                },
            },
        ]
        .into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();

    // As calculated above, the force each particle experiences is (-8m/9) r^(-2) and each has
    // mass 1.0 in this test.
    let evolution_configuration = super::configuration_parsing::EvolutionConfiguration {
        dead_zone_radius: dead_zone_radius.0,
        inverse_squared_coupling: -8.0 / 9.0,
        inverse_fourth_coupling: 0.0,
        milliseconds_per_time_slice: 1000,
        number_of_time_slices: number_of_time_slices,
    };
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());

    let inverse_squared_potential_of_pair = InverseSquaredAndFourthPotential {
        inverse_squared_coupling_constant: evolution_configuration.inverse_squared_coupling,
        inverse_fourth_coupling_constant: 0.0,
        dead_zone_radius: *dead_zone_radius,
    };

    let test_tolerances = create_test_tolerances();
    // The total energy should be 0.0 constantly.
    compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        Some(
            |particle_list: &std::vec::Vec<data_structure::IndividualParticle>| {
                check_energy_given_potential(
                    2,
                    0.0,
                    TEST_DEFAULT_TOLERANCE,
                    particle_list,
                    inverse_squared_potential_of_pair,
                )
            },
        ),
    )
}

pub fn test_equal_masses_attracting_inverse_square_circular_orbit<T, U>(
    tested_implementation: &mut T,
    dead_zone_radius: &data_structure::SeparationUnit,
) -> Result<(), String>
where
    T: super::ParticlesInTimeEvolver<U>,
    U: std::iter::ExactSizeIterator<
        Item = <T as super::ParticlesInTimeEvolver<U>>::EmittedIterator,
    >,
{
    let test_intrinsics = data_structure::ParticleIntrinsics {
        inertial_mass: data_structure::InertialMassUnit(1.0),
        inverse_squared_charge: data_structure::InverseSquaredChargeUnit(1.0),
        inverse_fourth_charge: data_structure::InverseFourthChargeUnit(0.0),
        color_brightness: data_structure::new_color_triplet(
            data_structure::RedColorUnit(4.0),
            data_structure::GreenColorUnit(5.0),
            data_structure::BlueColorUnit(6.0),
        ),
    };

    // The force needs to be m r w^2 where w is the angular speed.
    // Since m = r = 1, we pick F = w = 1, so both should have charge 1 and the overall coupling
    // should be 4 to account for the separation being 2, so inverse squared giving 1/4.
    let left_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(-1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(-1.0),
            },
        },
    };
    let right_particle = data_structure::IndividualParticle {
        intrinsic_values: test_intrinsics,
        variable_values: data_structure::ParticleVariables {
            position_vector: data_structure::PositionVector {
                horizontal_component: data_structure::HorizontalPositionUnit(1.0),
                vertical_component: data_structure::VerticalPositionUnit(0.0),
            },
            velocity_vector: data_structure::VelocityVector {
                horizontal_component: data_structure::HorizontalVelocityUnit(0.0),
                vertical_component: data_structure::VerticalVelocityUnit(1.0),
            },
        },
    };
    let initial_conditions = vec![left_particle.clone(), right_particle.clone()];

    // We will choose 200ms per time slice below, so the time sequence has increments of 0.2s.
    // (The fact that we go up to only 1.2 is cheating a little as the inaccuracy adds up over time slices,
    // and by 1.6 the actuals deviate by over 1%.)
    let following_expecteds = [0.2_f64, 0.4_f64, 0.6_f64, 0.8_f64, 1.0_f64, 1.2_f64]
        .iter()
        .map(|time_value| {
            let cosine_value = time_value.cos();
            let sine_value = time_value.sin();
            vec![
                data_structure::IndividualParticle {
                    intrinsic_values: test_intrinsics,
                    variable_values: data_structure::ParticleVariables {
                        position_vector: data_structure::PositionVector {
                            horizontal_component: data_structure::HorizontalPositionUnit(
                                -cosine_value,
                            ),
                            vertical_component: data_structure::VerticalPositionUnit(-sine_value),
                        },
                        velocity_vector: data_structure::VelocityVector {
                            horizontal_component: data_structure::HorizontalVelocityUnit(
                                sine_value,
                            ),
                            vertical_component: data_structure::VerticalVelocityUnit(-cosine_value),
                        },
                    },
                },
                data_structure::IndividualParticle {
                    intrinsic_values: test_intrinsics,
                    variable_values: data_structure::ParticleVariables {
                        position_vector: data_structure::PositionVector {
                            horizontal_component: data_structure::HorizontalPositionUnit(
                                cosine_value,
                            ),
                            vertical_component: data_structure::VerticalPositionUnit(sine_value),
                        },
                        velocity_vector: data_structure::VelocityVector {
                            horizontal_component: data_structure::HorizontalVelocityUnit(
                                -sine_value,
                            ),
                            vertical_component: data_structure::VerticalVelocityUnit(cosine_value),
                        },
                    },
                },
            ]
        })
        .flatten()
        .collect::<std::vec::Vec<data_structure::IndividualParticle>>();

    let expected_sequence = vec![
        initial_conditions
            .iter()
            .cloned()
            .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
            .into_iter(),
        following_expecteds.into_iter(),
    ];

    let number_of_time_slices = expected_sequence.len();

    // As mentioned above, for an inverse-squared force of magnitude 1 with a separation of 2,
    // the coupling must be 4.
    let evolution_configuration = super::configuration_parsing::EvolutionConfiguration {
        dead_zone_radius: dead_zone_radius.0,
        inverse_squared_coupling: -4.0,
        inverse_fourth_coupling: 0.0,
        milliseconds_per_time_slice: 200,
        number_of_time_slices: number_of_time_slices,
    };
    let evolution_result = tested_implementation
        .create_time_sequence(&evolution_configuration, initial_conditions.into_iter());

    let inverse_squared_potential_of_pair = InverseSquaredAndFourthPotential {
        inverse_squared_coupling_constant: evolution_configuration.inverse_squared_coupling,
        inverse_fourth_coupling_constant: 0.0,
        dead_zone_radius: *dead_zone_radius,
    };

    let test_tolerances = create_test_tolerances();
    // The total energy is potential plus kinetic.
    // The potential is -coupling/r => -4/2 = -2.
    // The kinetic is 2 * (0.5 m v^2) => 1.
    // Hence the total is -1.0 in whatever units it works out as.
    compare_time_slices_to_expected(
        evolution_result,
        expected_sequence.into_iter(),
        &test_tolerances,
        Some(
            |particle_list: &std::vec::Vec<data_structure::IndividualParticle>| {
                check_energy_given_potential(
                    2,
                    -1.0,
                    TEST_DEFAULT_TOLERANCE,
                    particle_list,
                    inverse_squared_potential_of_pair,
                )
            },
        ),
    )
}
