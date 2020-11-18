/// This module provides an implementation of ParticlesInTimeEvolver which uses the Euler method to
/// second order on the positions (assuming a constant force over the timestep) to numerically solve
/// the equations of motion.
use crate::data_structure::particle::CollectionInForceField;
use crate::data_structure::particle::CollectionInForceFieldGenerator;
use crate::data_structure::particle::WritableInForceField;

pub struct SecondOrderEuler<CollectionElement, CollectionGenerator>
where
    CollectionElement: WritableInForceField,
    CollectionGenerator: CollectionInForceFieldGenerator<MutableElement = CollectionElement>,
{
    number_of_internal_slices_per_time_slice: u32,
    collection_generator: CollectionGenerator,

    phantom_particle_type: std::marker::PhantomData<CollectionElement>,
}

impl<CollectionElement, CollectionGenerator>
    SecondOrderEuler<CollectionElement, CollectionGenerator>
where
    CollectionElement: WritableInForceField,
    CollectionGenerator: CollectionInForceFieldGenerator<MutableElement = CollectionElement>,
{
    fn create_particles_in_force_field(
        &self,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::particle::IndividualRepresentation,
        >,
        time_interval_per_internal_slice: &data_structure::time::IntervalUnit,
    ) -> Result<CollectionGenerator::CreatedCollection, Box<dyn std::error::Error>> {
        let mut evolving_particles = self.collection_generator.create_collection();
        let mut initial_condition_errors: std::vec::Vec<(usize, Box<dyn std::error::Error>)> =
            vec![];
        for (initial_particle_index, initial_particle) in initial_conditions.enumerate() {
            match data_structure::time::divide_time_by_mass(
                time_interval_per_internal_slice,
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

        Ok(evolving_particles)
    }

    fn update_forces<ParticleImplementation, ParticleCollection>(
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        particles_with_forces: &mut ParticleCollection,
    ) where
        ParticleImplementation: WritableInForceField,
        ParticleCollection: data_structure::collection::SingleAndPairwiseFinite<
            MutableElement = ParticleImplementation,
        >,
    {
        // First all the forces must be set to zero so that we can aggregate the pairwise forces.
        particles_with_forces.apply_to_every_single(&mut |particle_with_force| {
            let mut force_on_particle = particle_with_force.write_experienced_force();
            force_on_particle.horizontal_component = data_structure::force::HorizontalUnit(0.0);
            force_on_particle.vertical_component = data_structure::force::VerticalUnit(0.0);
        });
        particles_with_forces.apply_to_every_pair(
            &mut |first_particle, second_particle| {
                super::force_on_first_particle_from_second_particle(
                    evolution_configuration,
                    first_particle,
                    second_particle,
                )
            },
            &mut |first_particle, force_on_first| {
                *first_particle.write_experienced_force() += *force_on_first;
            },
            &mut |second_particle, force_on_first| {
                *second_particle.write_experienced_force() -= *force_on_first;
            },
        )
    }

    /// This updates the velocity and position assuming a constant acceleration for the time interval.
    fn update_velocity_and_position<T>(
        time_difference_per_internal_slice: &data_structure::time::IntervalUnit,
        particle_and_force: &mut T,
    ) where
        T: WritableInForceField,
    {
        let velocity_difference = data_structure::velocity_change_from_force(
            particle_and_force.read_experienced_force(),
            particle_and_force.read_timestep_over_inertial_mass(),
        );
        let particle_variables = particle_and_force.write_particle_variables();
        let average_velocity = data_structure::velocity::sum_with_scaled_other(
            &particle_variables.velocity_vector,
            &velocity_difference,
            0.5,
        );
        particle_variables.velocity_vector += velocity_difference;
        data_structure::increment_position_by_velocity_for_time_interval(
            &mut particle_variables.position_vector,
            &average_velocity,
            &time_difference_per_internal_slice,
        );
    }

    fn evolve_particle_configuration<ParticleImplementation, ParticleCollection>(
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        evolving_particles: &mut ParticleCollection,
        number_of_internal_slices_per_time_slice: u32,
        time_interval_per_internal_slice: &data_structure::time::IntervalUnit,
    ) -> std::vec::Vec<std::vec::IntoIter<data_structure::particle::BasicIndividual>>
    where
        ParticleImplementation: WritableInForceField,
        ParticleCollection: data_structure::collection::SingleAndPairwiseFinite<
            MutableElement = ParticleImplementation,
        >,
    {
        let mut evaluations_at_time_slices: std::vec::Vec<
            std::vec::IntoIter<data_structure::particle::BasicIndividual>,
        > = std::vec::Vec::with_capacity(evolution_configuration.number_of_time_slices);

        let mut initial_time_slice_without_force =
            std::vec::Vec::<data_structure::particle::BasicIndividual>::with_capacity(
                evolving_particles.get_count(),
            );
        evolving_particles.apply_to_every_single(&mut |particle_with_force| {
            initial_time_slice_without_force.push(particle_with_force.into_individual_particle());
        });
        evaluations_at_time_slices.push(initial_time_slice_without_force.into_iter());

        for _ in 1..evolution_configuration.number_of_time_slices {
            for _ in 0..number_of_internal_slices_per_time_slice {
                Self::update_forces(evolution_configuration, evolving_particles);

                evolving_particles.apply_to_every_single(&mut |particle_with_force| {
                    Self::update_velocity_and_position(
                        time_interval_per_internal_slice,
                        particle_with_force,
                    )
                });
            }

            let mut current_time_slice_without_force =
                std::vec::Vec::<data_structure::particle::BasicIndividual>::with_capacity(
                    evolving_particles.get_count(),
                );
            evolving_particles.apply_to_every_single(&mut |particle_with_force| {
                current_time_slice_without_force
                    .push(particle_with_force.into_individual_particle());
            });
            evaluations_at_time_slices.push(current_time_slice_without_force.into_iter());
        }
        evaluations_at_time_slices
    }
}

impl<CollectionElement, CollectionGenerator> super::ParticlesInTimeEvolver
    for SecondOrderEuler<CollectionElement, CollectionGenerator>
where
    CollectionElement: WritableInForceField,
    CollectionGenerator: CollectionInForceFieldGenerator<MutableElement = CollectionElement>,
{
    type EmittedParticle = data_structure::particle::BasicIndividual;
    type ParticleIterator = std::vec::IntoIter<Self::EmittedParticle>;
    type IteratorIterator = std::vec::IntoIter<Self::ParticleIterator>;

    fn create_time_sequence(
        &mut self,
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::particle::IndividualRepresentation,
        >,
    ) -> Result<
        super::ParticleSetEvolution<
            Self::EmittedParticle,
            Self::ParticleIterator,
            Self::IteratorIterator,
        >,
        Box<dyn std::error::Error>,
    > {
        if evolution_configuration.dead_zone_radius <= 0.0 {
            return Err(Box::new(super::ParameterError::new(
                "Dead zone radius must be > 0.",
            )));
        }

        if evolution_configuration.number_of_time_slices < 1 {
            return Ok(super::ParticleSetEvolution {
                particle_configurations: vec![].into_iter(),
                milliseconds_between_configurations: evolution_configuration
                    .milliseconds_per_time_slice,
            });
        }

        let seconds_between_configurations = (evolution_configuration.milliseconds_per_time_slice
            as f64)
            * configuration_parsing::SECONDS_PER_MILLISECOND;

        // The calculation uses a smaller time interval than the output time difference between the
        // configurations.
        let time_interval_per_internal_slice = data_structure::time::IntervalUnit(
            seconds_between_configurations / (self.number_of_internal_slices_per_time_slice as f64),
        );
        let mut evolving_particles = self.create_particles_in_force_field(
            initial_conditions,
            &time_interval_per_internal_slice,
        )?;
        let time_slices_without_forces = Self::evolve_particle_configuration(
            evolution_configuration,
            evolving_particles.access_mutable_elements(),
            self.number_of_internal_slices_per_time_slice,
            &time_interval_per_internal_slice,
        );

        Ok(super::ParticleSetEvolution {
            particle_configurations: time_slices_without_forces.into_iter(),
            milliseconds_between_configurations: evolution_configuration
                .milliseconds_per_time_slice,
        })
    }
}

pub fn new_given_memory_strategy<CollectionElement, CollectionGenerator>(
    number_of_internal_slices_per_time_slice: u32,
    collection_generator: CollectionGenerator,
) -> Result<SecondOrderEuler<CollectionElement, CollectionGenerator>, Box<dyn std::error::Error>>
where
    CollectionElement: WritableInForceField,
    CollectionGenerator: CollectionInForceFieldGenerator<MutableElement = CollectionElement>,
{
    if number_of_internal_slices_per_time_slice == 0 {
        Err(Box::new(super::ParameterError::new(
            "Number of internal slices between displayed slices must be > 0.",
        )))
    } else {
        Ok(SecondOrderEuler {
            number_of_internal_slices_per_time_slice: number_of_internal_slices_per_time_slice,
            collection_generator: collection_generator,
            phantom_particle_type: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_functions as evolver_tests;
    use super::*;
    use data_structure::particle::contiguous_struct as contiguous_particle_struct;
    use data_structure::particle::struct_of_boxes as particle_struct_of_boxes;

    const TEST_DEAD_ZONE_RADIUS: data_structure::position::SeparationUnit =
        data_structure::position::SeparationUnit(1.0);

    fn new_maximally_contiguous_for_test() -> Result<
        SecondOrderEuler<
            contiguous_particle_struct::MassNormalizedWithForceField,
            contiguous_particle_struct::VectorOfMassNormalizedWithForceFieldGenerator,
        >,
        String,
    > {
        new_given_memory_strategy(
            100,
            contiguous_particle_struct::VectorOfMassNormalizedWithForceFieldGenerator {},
        )
        .or_else(|construction_error| {
            Err(String::from(format!(
                "Constructor error in new_maximally_contiguous_for_test: {:?}",
                construction_error
            )))
        })
    }

    fn new_contiguous_pointers_for_test() -> Result<
        SecondOrderEuler<
            std::boxed::Box<dyn data_structure::particle::WritableInForceField>,
            contiguous_particle_struct::VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator,
        >,
        String,
    > {
        new_given_memory_strategy(
            100,
            contiguous_particle_struct::VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator {},
        )
        .or_else(|construction_error| {
            Err(String::from(format!(
                "Constructor error in new_contiguous_pointers_for_test: {:?}",
                construction_error
            )))
        })
    }

    fn new_double_boxed_for_test() -> Result<
        SecondOrderEuler<
            std::boxed::Box<dyn data_structure::particle::WritableInForceField>,
            particle_struct_of_boxes::VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator,
        >,
        String,
>{
        new_given_memory_strategy(
            100,
            particle_struct_of_boxes::VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator {},
        )
        .or_else(|construction_error| {
            Err(String::from(format!(
                "Constructor error in new_contiguous_pointers_for_test: {:?}",
                construction_error
            )))
        })
    }

    #[test]
    fn test_single_particle_at_rest_stays_at_rest_with_maximally_contiguous() -> Result<(), String>
    {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_single_particle_at_rest_stays_at_rest(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_rest_stays_at_rest_with_contiguous_pointers() -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_single_particle_at_rest_stays_at_rest(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_rest_stays_at_rest_with_double_boxed() -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_single_particle_at_rest_stays_at_rest(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_constant_speed_with_maximally_contiguous() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_single_particle_at_constant_speed(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_constant_speed_with_contiguous_pointers() -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_single_particle_at_constant_speed(&mut evolver_implementation)
    }

    #[test]
    fn test_single_particle_at_constant_speed_with_double_boxed() -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_single_particle_at_constant_speed(&mut evolver_implementation)
    }

    #[test]
    fn test_uncharged_particles_do_not_accelerate_with_maximally_contiguous() -> Result<(), String>
    {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_uncharged_particles_do_not_accelerate(&mut evolver_implementation)
    }

    #[test]
    fn test_uncharged_particles_do_not_accelerate_with_contiguous_pointers() -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_uncharged_particles_do_not_accelerate(&mut evolver_implementation)
    }

    #[test]
    fn test_uncharged_particles_do_not_accelerate_with_double_boxed() -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_uncharged_particles_do_not_accelerate(&mut evolver_implementation)
    }

    #[test]
    fn test_immobile_repelling_particles_within_dead_zone_stay_at_rest_with_maximally_contiguous(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_immobile_repelling_particles_within_dead_zone_stay_at_rest(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_immobile_repelling_particles_within_dead_zone_stay_at_rest_with_contiguous_pointers(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_immobile_repelling_particles_within_dead_zone_stay_at_rest(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_immobile_repelling_particles_within_dead_zone_stay_at_rest_with_double_boxed(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_immobile_repelling_particles_within_dead_zone_stay_at_rest(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_fourth_critical_escape_with_maximally_contiguous(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_fourth_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_fourth_critical_escape_with_contiguous_pointers(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_fourth_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_fourth_critical_escape_with_double_boxed(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_fourth_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_repelling_inverse_fourth_accelerate_away_equally_with_maximally_contiguous(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_equal_masses_repelling_inverse_fourth_accelerate_away_equally(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_repelling_inverse_fourth_accelerate_away_equally_with_contiguous_pointers(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_equal_masses_repelling_inverse_fourth_accelerate_away_equally(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_repelling_inverse_fourth_accelerate_away_equally_with_double_boxed(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_equal_masses_repelling_inverse_fourth_accelerate_away_equally(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_critical_escape_with_maximally_contiguous(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_critical_escape_with_contiguous_pointers(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_critical_escape_with_double_boxed(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_critical_escape(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_circular_orbit_with_maximally_contiguous(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_circular_orbit(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_circular_orbit_with_contiguous_pointers(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_circular_orbit(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_equal_masses_attracting_inverse_square_circular_orbit_with_double_boxed(
    ) -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_equal_masses_attracting_inverse_square_circular_orbit(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_triangle_at_cancelling_forces_is_stable_with_maximally_contiguous() -> Result<(), String>
    {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_triangle_at_cancelling_forces_is_stable(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_triangle_at_cancelling_forces_is_stable_with_contiguous_pointers() -> Result<(), String>
    {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_triangle_at_cancelling_forces_is_stable(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_triangle_at_cancelling_forces_is_stable_with_double_boxed() -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_triangle_at_cancelling_forces_is_stable(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_approximate_harmonic_oscillator_with_maximally_contiguous() -> Result<(), String> {
        let mut evolver_implementation = new_maximally_contiguous_for_test()?;
        evolver_tests::test_approximate_harmonic_oscillator(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_approximate_harmonic_oscillator_with_contiguous_pointers() -> Result<(), String> {
        let mut evolver_implementation = new_contiguous_pointers_for_test()?;
        evolver_tests::test_approximate_harmonic_oscillator(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }

    #[test]
    fn test_approximate_harmonic_oscillator_with_double_boxed() -> Result<(), String> {
        let mut evolver_implementation = new_double_boxed_for_test()?;
        evolver_tests::test_approximate_harmonic_oscillator(
            &mut evolver_implementation,
            &TEST_DEAD_ZONE_RADIUS,
        )
    }
}
