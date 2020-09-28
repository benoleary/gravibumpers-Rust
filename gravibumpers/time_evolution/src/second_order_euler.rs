/// This module provides an implementation of ParticlesInTimeEvolver which uses a contiguous array
/// of structs of structs, so the maximally contiguous case.

/// In order to use Euler's method to second order, we keep the instantaneous force experienced by
/// the particle so that we can evaluate the force field at all the points with particles and only
/// then update the positions for a time step, assuming constant forces for the time step. We also
/// prepare a factor which is the common timestep of the evolution divided by the inertial mass,
/// which is used for multiplication with the force, for better efficiency.
pub trait ParticleInForceField: data_structure::ParticleRepresentation + Sized {
    fn into_individual_particle(&self) -> data_structure::IndividualParticle;
    fn read_experienced_force<'a>(&'a self) -> &'a data_structure::ForceVector;
    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a data_structure::TimeOverMassUnit;
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut data_structure::ParticleVariables;
    fn write_experienced_force<'a>(&'a mut self) -> &'a mut data_structure::ForceVector;
}

/// The trait should have a consistent order of iteration.
pub trait IndexedParticleCollectionInForceField<'a>:
    std::ops::Index<usize> + std::ops::IndexMut<usize> + Sized
where
    <Self as std::ops::Index<usize>>::Output: ParticleInForceField + 'a,
    Self: 'a,
{
    type ImmutableIterator: std::iter::ExactSizeIterator<
        Item = &'a <Self as std::ops::Index<usize>>::Output,
    >;
    type MutableIterator: std::iter::ExactSizeIterator<
        Item = &'a mut <Self as std::ops::Index<usize>>::Output,
    >;
    fn add_particle(
        &mut self,
        particle_to_add: &impl data_structure::ParticleRepresentation,
        timestep_over_inertial_mass: &data_structure::TimeOverMassUnit,
    );
    fn get_length(&self) -> usize;
    fn get_immutable_iterator(&'a mut self) -> Self::ImmutableIterator;
    fn get_mutable_iterator(&'a mut self) -> Self::MutableIterator;
    fn reset_forces(&mut self);
    fn apply_pairwise_force(
        &mut self,
        index_for_adding_given_force: usize,
        index_for_subtracting_given_force: usize,
        force_vector: &data_structure::ForceVector,
    );
}

struct MassNormalizedParticleWithForceField {
    particle_description: data_structure::IndividualParticle,
    experienced_force: data_structure::ForceVector,
    timestep_over_inertial_mass: data_structure::TimeOverMassUnit,
}

impl data_structure::ParticleRepresentation for MassNormalizedParticleWithForceField {
    fn read_intrinsics(&self) -> &data_structure::ParticleIntrinsics {
        self.particle_description.read_intrinsics()
    }

    fn read_variables(&self) -> &data_structure::ParticleVariables {
        self.particle_description.read_variables()
    }
}

impl ParticleInForceField for MassNormalizedParticleWithForceField {
    fn into_individual_particle(&self) -> data_structure::IndividualParticle {
        self.particle_description
    }

    fn read_experienced_force<'a>(&'a self) -> &'a data_structure::ForceVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a data_structure::TimeOverMassUnit {
        &self.timestep_over_inertial_mass
    }

    fn write_particle_variables<'a>(&'a mut self) -> &'a mut data_structure::ParticleVariables {
        &mut self.particle_description.variable_values
    }

    fn write_experienced_force<'a>(&'a mut self) -> &'a mut data_structure::ForceVector {
        &mut self.experienced_force
    }
}

impl<'a> IndexedParticleCollectionInForceField<'a>
    for std::vec::Vec<MassNormalizedParticleWithForceField>
{
    type ImmutableIterator = std::slice::Iter<'a, MassNormalizedParticleWithForceField>;
    type MutableIterator = std::slice::IterMut<'a, MassNormalizedParticleWithForceField>;
    fn add_particle(
        &mut self,
        particle_to_add: &impl data_structure::ParticleRepresentation,
        timestep_over_inertial_mass: &data_structure::TimeOverMassUnit,
    ) {
        self.push(MassNormalizedParticleWithForceField {
            particle_description: data_structure::create_individual_from_representation(
                particle_to_add,
            ),
            experienced_force: data_structure::ForceVector {
                horizontal_component: data_structure::HorizontalForceUnit(0.0),
                vertical_component: data_structure::VerticalForceUnit(0.0),
            },
            timestep_over_inertial_mass: *timestep_over_inertial_mass,
        })
    }

    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_immutable_iterator(&'a mut self) -> Self::ImmutableIterator {
        self.iter()
    }

    fn get_mutable_iterator(&'a mut self) -> Self::MutableIterator {
        self.iter_mut()
    }

    fn reset_forces(&mut self) {
        for particle_with_force in self.iter_mut() {
            let mut force_on_particle = particle_with_force.write_experienced_force();
            force_on_particle.horizontal_component = data_structure::HorizontalForceUnit(0.0);
            force_on_particle.vertical_component = data_structure::VerticalForceUnit(0.0);
        }
    }

    fn apply_pairwise_force(
        &mut self,
        index_for_adding_given_force: usize,
        index_for_subtracting_given_force: usize,
        force_vector: &data_structure::ForceVector,
    ) {
        *self[index_for_adding_given_force].write_experienced_force() += *force_vector;
        *self[index_for_subtracting_given_force].write_experienced_force() -= *force_vector;
    }
}

pub struct SecondOrderEuler {
    number_of_internal_slices_per_time_slice: u32,
}

impl SecondOrderEuler {
    /// This updates the velocities and positions assuming a constant acceleration for the time interval.
    fn update_velocities_and_positions<'a, T, U>(
        &'a self,
        time_difference_per_internal_slice: &data_structure::TimeDifferenceUnit,
        particles_and_forces: U,
    ) where
        T: ParticleInForceField + 'a,
        U: std::iter::ExactSizeIterator<Item = &'a mut T>,
    {
        for particle_and_force in particles_and_forces {
            let velocity_difference = data_structure::velocity_change_from_force(
                particle_and_force.read_experienced_force(),
                particle_and_force.read_timestep_over_inertial_mass(),
            );
            let particle_variables = particle_and_force.write_particle_variables();
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

fn create_time_slice_copy_without_force<'a, T, U>(
    particles_with_forces: U,
) -> std::vec::IntoIter<data_structure::IndividualParticle>
where
    T: ParticleInForceField + 'a,
    U: std::iter::ExactSizeIterator<Item = &'a T>,
{
    particles_with_forces
        .map(|particle_with_force| particle_with_force.into_individual_particle())
        .collect::<std::vec::Vec<data_structure::IndividualParticle>>()
        .into_iter()
}

fn aggregate_pairwise_forces<'a, T, U>(
    evolution_configuration: &configuration_parsing::EvolutionConfiguration,
    particles_with_forces: &'a mut U,
) where
    T: ParticleInForceField + 'a,
    U: IndexedParticleCollectionInForceField<'a, Output = T> + 'a,
{
    let number_of_particles = particles_with_forces.get_length();
    for first_particle_index in 0..(number_of_particles - 1) {
        for second_particle_index in (first_particle_index + 1)..number_of_particles {
            let pairwise_force = super::force_on_first_particle_from_second_particle(
                evolution_configuration,
                &particles_with_forces[first_particle_index],
                &particles_with_forces[second_particle_index],
            );
            particles_with_forces.apply_pairwise_force(
                first_particle_index,
                second_particle_index,
                &pairwise_force,
            );
        }
    }
}

fn update_forces<'a, 'b, T, U>(
    evolution_configuration: &configuration_parsing::EvolutionConfiguration,
    particles_with_forces: &'a mut U,
) where
    T: ParticleInForceField + 'b,
    U: IndexedParticleCollectionInForceField<'b, Output = T> + 'a,
    'a: 'b,
{
    // First all the forces must be set to zero so that we can aggregate the pairwise forces.
    particles_with_forces.reset_forces();

    aggregate_pairwise_forces(evolution_configuration, particles_with_forces);
}

impl
    super::ParticlesInTimeEvolver<
        std::vec::IntoIter<std::vec::IntoIter<data_structure::IndividualParticle>>,
    > for SecondOrderEuler
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
        let mut evolving_particles: std::vec::Vec<MassNormalizedParticleWithForceField> =
            std::vec::Vec::with_capacity(initial_conditions.len());
        let mut initial_condition_errors: std::vec::Vec<(usize, Box<dyn std::error::Error>)> =
            vec![];
        for (initial_particle_index, initial_particle) in initial_conditions.enumerate() {
            match data_structure::divide_time_by_mass(
                &time_interval_per_internal_slice,
                &initial_particle.read_intrinsics().inertial_mass,
            ) {
                Ok(time_over_mass) => {
                    evolving_particles.add_particle(&initial_particle, &time_over_mass)
                }
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
            evolving_particles.get_immutable_iterator(),
        ));
        for _ in 1..evolution_configuration.number_of_time_slices {
            for _ in 0..self.number_of_internal_slices_per_time_slice {
                update_forces(evolution_configuration, &mut evolving_particles);
                self.update_velocities_and_positions(
                    &time_interval_per_internal_slice,
                    evolving_particles.get_mutable_iterator(),
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

pub fn new_second_order_euler(
    number_of_internal_slices_per_time_slice: u32,
) -> Result<SecondOrderEuler, Box<dyn std::error::Error>> {
    if number_of_internal_slices_per_time_slice == 0 {
        Err(Box::new(super::ParameterError::new(
            "Number of internal slices between displayed slices must be > 0.",
        )))
    } else {
        Ok(SecondOrderEuler {
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

    fn new_maximally_contiguous_euler_for_test() -> Result<SecondOrderEuler, String> {
        new_second_order_euler(100).or_else(|construction_error| {
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

    #[test]
    fn test_equal_masses_attracting_inverse_square_critical_escape() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_circular_orbit() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_circular_orbit(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_triangle_at_cancelling_forces_is_stable() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_triangle_at_cancelling_forces_is_stable(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_approximate_harmonic_oscillator() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_euler_for_test()?;
        evolver_tests::test_approximate_harmonic_oscillator(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }
}
