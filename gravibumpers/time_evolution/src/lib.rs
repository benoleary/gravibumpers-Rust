/// This crate provides structs, traits, and functions for evolving initial conditions into
/// sequences of collections of particles.
extern crate data_structure;
pub mod test_functions;
pub mod vec_of_pure_struct;
use std::error::Error;

#[derive(Debug)]
pub struct EvolutionError {
    error_message: String,
}

impl EvolutionError {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}

impl Error for EvolutionError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl std::fmt::Display for EvolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error in time evolution: {}", self.error_message)
    }
}

#[derive(Debug)]
pub struct ParameterError {
    error_message: String,
}

impl ParameterError {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}

impl Error for ParameterError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl std::fmt::Display for ParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error in time evolution: {}", self.error_message)
    }
}

pub trait ParticlesInTimeEvolver<T: std::iter::ExactSizeIterator<Item = Self::EmittedIterator>> {
    type EmittedParticle: data_structure::ParticleRepresentation;
    type EmittedIterator: std::iter::ExactSizeIterator<Item = Self::EmittedParticle>;

    fn create_time_sequence(
        &mut self,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::ParticleRepresentation,
        >,
        number_of_time_slices: usize,
    ) -> Result<T, Box<dyn std::error::Error>>;
}

fn force_on_first_particle_from_second_particle(
    first_particle: impl data_structure::ParticleRepresentation,
    second_particle: impl data_structure::ParticleRepresentation,
) -> data_structure::ForceVector {
    data_structure::ForceVector {
        horizontal_component: data_structure::HorizontalForceUnit(0.0),
        vertical_component: data_structure::VerticalForceUnit(0.0),
    }
}
