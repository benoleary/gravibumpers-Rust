extern crate data_structure;

pub trait ParticlesInTimeEvolver<T: data_structure::ParticleIteratorProvider> {
    fn create_time_sequence(
        &self,
        initial_conditions: &mut dyn data_structure::ParticleIteratorProvider,
    ) -> Box<dyn std::iter::ExactSizeIterator<Item = T>>;
}

pub struct DummyEvolver {
    pub number_of_copies: usize,
}

impl ParticlesInTimeEvolver<data_structure::ParticleVector> for DummyEvolver {
    fn create_time_sequence(
        &self,
        initial_conditions: &mut dyn data_structure::ParticleIteratorProvider,
    ) -> Box<dyn std::iter::ExactSizeIterator<Item = data_structure::ParticleVector>> {
        let particle_iterator = initial_conditions.get();
        let number_of_particles = particle_iterator.len();
        let mut vector_of_copies: std::vec::Vec<std::vec::Vec<data_structure::IndividualParticle>> =
            std::vec::Vec::with_capacity(self.number_of_copies);
        for _ in 0..self.number_of_copies {
            let particle_vector: std::vec::Vec<data_structure::IndividualParticle> =
                std::vec::Vec::with_capacity(number_of_particles);
            vector_of_copies.push(particle_vector);
        }

        for initial_particle in particle_iterator {
            for copy_vector in &mut vector_of_copies {
                copy_vector.push(initial_particle);
            }
        }

        let mut vector_of_iterators: std::vec::Vec<data_structure::ParticleVector> =
            std::vec::Vec::with_capacity(self.number_of_copies);
        for copy_vector in vector_of_copies {
            vector_of_iterators.push(data_structure::wrap_particle_vector(copy_vector));
        }

        Box::new(vector_of_iterators.into_iter())
    }
}

pub fn hold_place(input_int: i32) -> i32 {
    println!(
        "time_evolution::hold_place(input_int = {input_int})",
        input_int = input_int
    );
    234
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_placeholder() {
        let placeholder_value = hold_place(0);
        assert_eq!(
            234, placeholder_value,
            "placeholder test, left is expected, right is actual"
        );
    }
}
