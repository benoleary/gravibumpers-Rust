/// This module provides an implementation of ParticlesInTimeEvolver which uses a contiguous array
/// of structs of structs, so the maximally contiguous case.

/// In order to use Euler's method to second order, we keep the instantaneous force experienced by
/// the particle so that we can evaluate the force field at all the points with particles and only
/// then update the positions for a time step, assuming constant forces for the time step. We also
/// prepare a factor which is the common timestep of the evolution divided by the inertial mass,
/// which is used for multiplication with the force, for better efficiency.
struct ParticleInForceField {
    particle_description: data_structure::IndividualParticle,
    experienced_force: data_structure::ForceVector,
    timestep_over_inertial_mass: data_structure::TimeOverMassUnit,
}

pub struct MaximallyContiguousEuler {
    number_of_internal_slices_per_time_slice: u32,
}

impl MaximallyContiguousEuler {
    /// This updates the velocities and positions assuming a constant acceleration for the time interval.
    fn update_velocities_and_positions(
        &self,
        time_difference_per_internal_slice: &data_structure::TimeDifferenceUnit,
        particles_and_forces: &mut std::vec::Vec<ParticleInForceField>,
    ) {
        for particle_and_force in particles_and_forces.iter_mut() {
            let particle_variables = &mut particle_and_force.particle_description.variable_values;
            let velocity_difference = data_structure::velocity_change_from_force(
                &particle_and_force.experienced_force,
                &particle_and_force.timestep_over_inertial_mass,
            );
            let average_velocity = data_structure::sum_velocity_with_scaled_velocity(
                &particle_variables.velocity_vector,
                &velocity_difference,
                0.5,
            );
            particle_variables.velocity_vector += velocity_difference;
            particle_variables
                .position_vector
                .increment_by_velocity_for_time_difference(
                    &average_velocity,
                    &time_difference_per_internal_slice,
                );
        }
    }
}

fn create_time_slice_copy_without_force<'a>(
    particles_with_forces: impl std::iter::ExactSizeIterator<Item = &'a ParticleInForceField>,
) -> std::vec::IntoIter<data_structure::IndividualParticle> {
    particles_with_forces
        .map(|particle_with_force| {
            data_structure::create_individual_from_representation(
                &particle_with_force.particle_description,
            )
        })
        .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
        .into_iter()
}

fn update_forces(
    evolution_configuration: &configuration_parsing::EvolutionConfiguration,
    particles_and_forces: &mut std::vec::Vec<ParticleInForceField>,
) {
    // First all the forces must be set to zero so that we can aggregate the pairwise forces.
    for mut particle_and_force in particles_and_forces.iter_mut() {
        particle_and_force.experienced_force.horizontal_component =
            data_structure::HorizontalForceUnit(0.0);
        particle_and_force.experienced_force.vertical_component =
            data_structure::VerticalForceUnit(0.0);
    }
    let number_of_particles = particles_and_forces.len();
    for first_particle_index in 0..(number_of_particles - 1) {
        // work out force on p1 = particles_and_forces[first_particle_index] from all
        // p2 = particles_and_forces[second_particle_index], increment force on p1 by each
        // force and increment force on p2 by equal opposite.
        for second_particle_index in (first_particle_index + 1)..number_of_particles {
            let pairwise_force = super::force_on_first_particle_from_second_particle(
                evolution_configuration,
                &particles_and_forces[first_particle_index].particle_description,
                &particles_and_forces[second_particle_index].particle_description,
            );
            particles_and_forces[first_particle_index].experienced_force += pairwise_force;
            particles_and_forces[second_particle_index].experienced_force -= pairwise_force;
        }
    }
}

impl
    super::ParticlesInTimeEvolver<
        std::vec::IntoIter<std::vec::IntoIter<data_structure::IndividualParticle>>,
    > for MaximallyContiguousEuler
{
    type EmittedParticle = data_structure::IndividualParticle;
    type EmittedIterator = std::vec::IntoIter<Self::EmittedParticle>;

    fn create_time_sequence(
        &mut self,
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::ParticleRepresentation,
        >,
    ) -> Result<
        super::ParticleSetEvolution<
            Self::EmittedParticle,
            Self::EmittedIterator,
            std::vec::IntoIter<Self::EmittedIterator>,
        >,
        Box<dyn std::error::Error>,
    > {
        if evolution_configuration.dead_zone_radius <= 0.0 {
            return Err(Box::new(super::ParameterError::new(
                "Dead zone radius must be > 0.",
            )));
        }
        let seconds_between_configurations = (evolution_configuration.milliseconds_per_time_slice
            as f64)
            * configuration_parsing::SECONDS_PER_MILLISECOND;

        if evolution_configuration.number_of_time_slices < 1 {
            return Ok(super::ParticleSetEvolution {
                particle_configurations: vec![].into_iter(),
                milliseconds_between_configurations: evolution_configuration
                    .milliseconds_per_time_slice,
            });
        }
        let mut evaluations_at_time_slices: std::vec::Vec<Self::EmittedIterator> =
            std::vec::Vec::with_capacity(evolution_configuration.number_of_time_slices);

        // The calculation uses a smaller time interval than the output time difference between the
        // configurations.
        let time_interval_per_internal_slice = data_structure::TimeDifferenceUnit(
            seconds_between_configurations / (self.number_of_internal_slices_per_time_slice as f64),
        );
        let mut evolving_particles: std::vec::Vec<ParticleInForceField> =
            std::vec::Vec::with_capacity(initial_conditions.len());
        let mut initial_condition_errors: std::vec::Vec<(usize, Box<dyn std::error::Error>)> =
            vec![];
        for (initial_particle_index, initial_particle) in initial_conditions.enumerate() {
            match data_structure::divide_time_by_mass(
                &time_interval_per_internal_slice,
                &initial_particle.read_intrinsics().inertial_mass,
            ) {
                Ok(time_over_mass) => evolving_particles.push(ParticleInForceField {
                    particle_description: data_structure::create_individual_from_representation(
                        &initial_particle,
                    ),
                    experienced_force: data_structure::ForceVector {
                        horizontal_component: data_structure::HorizontalForceUnit(0.0),
                        vertical_component: data_structure::VerticalForceUnit(0.0),
                    },
                    timestep_over_inertial_mass: time_over_mass,
                }),
                Err(initial_condition_error) => {
                    initial_condition_errors.push((initial_particle_index, initial_condition_error))
                }
            };
        }

        if !initial_condition_errors.is_empty() {
            return Err(Box::new(super::EvolutionError::new(&format!(
                "The following initial particles could not be set up for time evolution: {:?}",
                initial_condition_errors
            ))));
        }

        evaluations_at_time_slices.push(create_time_slice_copy_without_force(
            evolving_particles.iter(),
        ));
        for _ in 1..evolution_configuration.number_of_time_slices {
            for _ in 0..self.number_of_internal_slices_per_time_slice {
                update_forces(evolution_configuration, &mut evolving_particles);
                self.update_velocities_and_positions(
                    &time_interval_per_internal_slice,
                    &mut evolving_particles,
                );
            }

            evaluations_at_time_slices.push(create_time_slice_copy_without_force(
                evolving_particles.iter(),
            ));
        }
        Ok(super::ParticleSetEvolution {
            particle_configurations: evaluations_at_time_slices.into_iter(),
            milliseconds_between_configurations: evolution_configuration
                .milliseconds_per_time_slice,
        })
    }
}

pub fn new_maximally_contiguous_euler(
    number_of_internal_slices_per_time_slice: u32,
) -> Result<MaximallyContiguousEuler, Box<dyn std::error::Error>> {
    if number_of_internal_slices_per_time_slice == 0 {
        Err(Box::new(super::ParameterError::new(
            "Number of internal slices between displayed slices must be > 0.",
        )))
    } else {
        Ok(MaximallyContiguousEuler {
            number_of_internal_slices_per_time_slice: number_of_internal_slices_per_time_slice,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_functions as evolver_tests;
    use super::*;

    const TEST_DEAD_ZONE_RADIUS: data_structure::SeparationUnit =
        data_structure::SeparationUnit(1.0);

    fn new_maximally_contiguous_euler_for_test() -> Result<MaximallyContiguousEuler, String> {
        new_maximally_contiguous_euler(100).or_else(|construction_error| {
            Err(String::from(format!(
                "Constructor error: {:?}",
                construction_error
            )))
        })
    }

    #[test]
    fn test_single_particle_at_rest_stays_at_rest() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_single_particle_at_rest_stays_at_rest(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_constant_speed() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_single_particle_at_constant_speed(&mut evolver_implementation)
    }

    #[test]
    fn test_uncharged_particles_do_not_accelerate() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_uncharged_particles_do_not_accelerate(&mut evolver_implementation)
    }

    #[test]
    fn test_immobile_repelling_particles_within_dead_zone_stay_at_rest() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_immobile_repelling_particles_within_dead_zone_stay_at_rest(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_fourth_critical_escape() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_fourth_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_repelling_inverse_fourth_accelerate_away_equally() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_equal_masses_repelling_inverse_fourth_accelerate_away_equally(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }
}
