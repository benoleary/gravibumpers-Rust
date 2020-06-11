extern crate data_structure;

pub trait ParticlesInTimeEvolver<
    T: data_structure::ParticleIteratorProvider,
    U: std::iter::ExactSizeIterator<Item = T>,
>
{
    fn create_time_sequence(
        &self,
        initial_conditions: impl std::iter::ExactSizeIterator<Item = data_structure::IndividualParticle>,
    ) -> Result<U, Box<dyn std::error::Error>>;
}

pub struct DummyEvolver {
    pub number_of_copies: usize,
}

impl
    ParticlesInTimeEvolver<
        data_structure::ParticleVector,
        std::vec::IntoIter<data_structure::ParticleVector>,
    > for DummyEvolver
{
    fn create_time_sequence(
        &self,
        initial_conditions: impl std::iter::ExactSizeIterator<Item = data_structure::IndividualParticle>,
    ) -> Result<std::vec::IntoIter<data_structure::ParticleVector>, Box<dyn std::error::Error>>
    {
        let number_of_particles = initial_conditions.len();
        let mut vector_of_copies: std::vec::Vec<std::vec::Vec<data_structure::IndividualParticle>> =
            std::vec::Vec::with_capacity(self.number_of_copies);
        for _ in 0..self.number_of_copies {
            let particle_vector: std::vec::Vec<data_structure::IndividualParticle> =
                std::vec::Vec::with_capacity(number_of_particles);
            vector_of_copies.push(particle_vector);
        }

        for initial_particle in initial_conditions {
            for copy_vector in &mut vector_of_copies {
                copy_vector.push(initial_particle);
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
