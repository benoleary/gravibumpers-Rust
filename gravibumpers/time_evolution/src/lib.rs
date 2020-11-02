/// This crate provides structs, traits, and functions for evolving initial conditions into
/// sequences of collections of particles.
extern crate configuration_parsing;
extern crate data_structure;
pub mod second_order_euler;
pub mod test_functions;
use data_structure::force::DimensionfulVector as ForceVector;
use data_structure::particle::IndividualRepresentation as ParticleRepresentation;
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
    T: ParticleRepresentation,
    U: std::iter::ExactSizeIterator<Item = T>,
    V: std::iter::ExactSizeIterator<Item = U>,
{
    pub particle_configurations: V,
    pub milliseconds_between_configurations: u16,
}

pub trait ParticlesInTimeEvolver {
    type EmittedParticle: ParticleRepresentation;
    type ParticleIterator: std::iter::ExactSizeIterator<Item = Self::EmittedParticle>;
    type IteratorIterator: std::iter::ExactSizeIterator<Item = Self::ParticleIterator>;

    fn create_time_sequence(
        &mut self,
        evolution_configuration: &configuration_parsing::EvolutionConfiguration,
        initial_conditions: impl std::iter::ExactSizeIterator<Item = impl ParticleRepresentation>,
    ) -> Result<
        ParticleSetEvolution<Self::EmittedParticle, Self::ParticleIterator, Self::IteratorIterator>,
        Box<dyn std::error::Error>,
    >;
}

fn force_on_first_particle_from_second_particle(
    evolution_configuration: &configuration_parsing::EvolutionConfiguration,
    first_particle: &impl ParticleRepresentation,
    second_particle: &impl ParticleRepresentation,
) -> ForceVector {
    let separation_vector = first_particle.read_variables().position_vector
        - second_particle.read_variables().position_vector;
    if data_structure::position::SeparationUnit(evolution_configuration.dead_zone_radius)
        .is_greater_than_square(&separation_vector)
    {
        return ForceVector {
            horizontal_component: data_structure::force::HorizontalUnit(0.0),
            vertical_component: data_structure::force::VerticalUnit(0.0),
        };
    }

    let inverse_separation = data_structure::position::square_separation_vector(&separation_vector)
        .to_inverse_square_root();

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
    ForceVector {
        horizontal_component: data_structure::force::HorizontalUnit(
            separation_vector.horizontal_component.0 * force_magnitude_over_separation,
        ),
        vertical_component: data_structure::force::VerticalUnit(
            separation_vector.vertical_component.0 * force_magnitude_over_separation,
        ),
    }
}
