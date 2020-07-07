/// This crate provides structs, traits, and functions for evolving initial conditions into
/// sequences of collections of particles.
extern crate data_structure;

pub trait ParticlesInTimeEvolver<T: std::iter::ExactSizeIterator<Item = Self::EmittedIterator>> {
    type EmittedParticle: data_structure::ParticleRepresentation;
    type EmittedIterator: std::iter::ExactSizeIterator<Item = Self::EmittedParticle>;

    fn create_time_sequence(
        &mut self,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::ParticleRepresentation,
        >,
    ) -> Result<T, Box<dyn std::error::Error>>;
}

pub struct DummyEvolver {
    pub number_of_copies: usize,
}

impl ParticlesInTimeEvolver<std::vec::IntoIter<std::vec::IntoIter<data_structure::ParticleVector>>>
    for DummyEvolver
{
    type Output = data_structure::ParticleVector;

    fn create_time_sequence<'a>(
        &mut self,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = &'a data_structure::IndividualParticle,
        >,
    ) -> Result<std::vec::IntoIter<data_structure::ParticleVector>, Box<dyn std::error::Error>>
    {
        let number_of_particles = initial_conditions.len();
        self.vector_of_copies: std::vec::Vec<std::vec::Vec<data_structure::IndividualParticle>> =
            std::vec::Vec::with_capacity(self.number_of_copies);
        for _ in 0..self.number_of_copies {
            let particle_vector: std::vec::Vec<data_structure::IndividualParticle> =
                std::vec::Vec::with_capacity(number_of_particles);
            vector_of_copies.push(particle_vector);
        }

        for initial_particle in initial_conditions {
            for copy_vector in &mut vector_of_copies {
                copy_vector.push(*initial_particle);
            }
        }

        let mut vector_of_iterators: std::vec::Vec<data_structure::ParticleVector> =
            std::vec::Vec::with_capacity(self.number_of_copies);
        for copy_vector in vector_of_copies {
            vector_of_iterators.push(data_structure::wrap_particle_vector(copy_vector));
        }

        Ok(vector_of_iterators.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dummy_produces_correct_length() -> Result<(), String> {
        let expected_length = 23;
        let particles_in_time_evolver = super::DummyEvolver {
            number_of_copies: expected_length,
        };
        let empty_initial_conditions: std::vec::Vec<data_structure::IndividualParticle> = vec![];
        let evolution_result =
            particles_in_time_evolver.create_time_sequence(empty_initial_conditions.into_iter());

        match evolution_result {
            Ok(particle_map_sequence) => {
                if particle_map_sequence.len() == expected_length {
                    return Ok(());
                } else {
                    return Err(String::from(format!(
                        "Expected length = {}, actual length = {}",
                        expected_length,
                        particle_map_sequence.len()
                    )));
                }
            }
            Err(evolution_error) => Err(String::from(format!(
                "Time evolution encountered error {:?}",
                evolution_error
            ))),
        }
    }
}
