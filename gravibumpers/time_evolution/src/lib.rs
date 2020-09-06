/// This crate provides structs, traits, and functions for evolving initial conditions into
/// sequences of collections of particles.
extern crate configuration_parsing;
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

pub struct ParticleSetEvolution<T, U, V>
where
    T: data_structure::ParticleRepresentation,
    U: std::iter::ExactSizeIterator<Item = T>,
    V: std::iter::ExactSizeIterator<Item = U>,
{
    pub particle_configurations: V,
    pub milliseconds_between_configurations: u16,
}

pub trait ParticlesInTimeEvolver<T: std::iter::ExactSizeIterator<Item = Self::EmittedIterator>> {
    type EmittedParticle: data_structure::ParticleRepresentation;
    type EmittedIterator: std::iter::ExactSizeIterator<Item = Self::EmittedParticle>;

    fn create_time_sequence(
        &mut self,
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        initial_conditions: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::ParticleRepresentation,
        >,
    ) -> Result<
        ParticleSetEvolution<Self::EmittedParticle, Self::EmittedIterator, T>,
        Box<dyn std::error::Error>,
    >
    where
        <Self as ParticlesInTimeEvolver<T>>::EmittedIterator: std::iter::ExactSizeIterator;
}

fn force_on_first_particle_from_second_particle(
    evolution_configuration: &configuration_parsing::EvolutionConfiguration,
    first_particle: &impl data_structure::ParticleRepresentation,
    second_particle: &impl data_structure::ParticleRepresentation,
) -> data_structure::ForceVector {
    let separation_vector = first_particle.read_variables().position_vector
        - second_particle.read_variables().position_vector;
    if data_structure::SeparationUnit(evolution_configuration.dead_zone_radius)
        .is_greater_than_square(&separation_vector)
    {
        return data_structure::ForceVector {
            horizontal_component: data_structure::HorizontalForceUnit(0.0),
            vertical_component: data_structure::VerticalForceUnit(0.0),
        };
    }

    let inverse_separation =
        data_structure::square_separation_vector(&separation_vector).to_inverse_square_root();

    let inverse_squared_separation =
        inverse_separation.get_value() * inverse_separation.get_value();
    let inverse_squared_force = evolution_configuration.inverse_squared_coupling
        * first_particle.read_intrinsics().inverse_squared_charge.0
        * second_particle.read_intrinsics().inverse_squared_charge.0
        * inverse_squared_separation;
    let inverse_fourth_force = evolution_configuration.inverse_fourth_coupling
        * first_particle.read_intrinsics().inverse_fourth_charge.0
        * second_particle.read_intrinsics().inverse_fourth_charge.0
        * inverse_squared_separation
        * inverse_squared_separation;

    // We combine the sum of the two kinds of force with an additional 1/r so that we can multiply
    // the separation vector directly.
    let force_magnitude_over_separation =
        (inverse_squared_force + inverse_fourth_force) * inverse_separation.get_value();
    data_structure::ForceVector {
        horizontal_component: data_structure::HorizontalForceUnit(
            separation_vector.horizontal_component.0 * force_magnitude_over_separation,
        ),
        vertical_component: data_structure::VerticalForceUnit(
            separation_vector.vertical_component.0 * force_magnitude_over_separation,
        ),
    }
}
