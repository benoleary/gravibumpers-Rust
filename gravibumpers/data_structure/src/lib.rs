/// This crate defines the traits and structs for representing the particles from initial set-up
/// through time evolution to visualization.
///
/// As such this main lib file does not implement any logic (except some trivial implementations
/// of Add and a default unpacking for a trait) and thus has no #[cfg(test)].
///
/// There are public modules (comparison, color) but these exist to provide traits, structs, and
/// simple utility functions, or utility functions for tests, so also have no #[cfg(test)].
pub mod charge;
pub mod color;
pub mod comparison;
pub mod force;
pub mod particle;
pub mod position;
pub mod time;
pub mod velocity;
use std::error::Error;

#[derive(Debug)]
pub struct DimensionError {
    error_message: String,
}

impl DimensionError {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}

impl Error for DimensionError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl std::fmt::Display for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error setting dimension: {}", self.error_message)
    }
}

pub fn velocity_change_from_force(
    applied_force: &force::DimensionfulVector,
    timestep_over_inertial_mass: &time::OverMassUnit,
) -> velocity::DimensionfulVector {
    velocity::DimensionfulVector {
        horizontal_component: velocity::HorizontalUnit(
            applied_force.horizontal_component.0 * timestep_over_inertial_mass.0,
        ),
        vertical_component: velocity::VerticalUnit(
            applied_force.vertical_component.0 * timestep_over_inertial_mass.0,
        ),
    }
}

pub fn increment_position_by_velocity_for_time_interval(
    position_vector: &mut position::DimensionfulVector,
    velocity_vector: &velocity::DimensionfulVector,
    time_interval: &time::IntervalUnit,
) {
    position_vector.increment_by_components(
        &position::HorizontalUnit(velocity_vector.horizontal_component.0 * time_interval.0),
        &position::VerticalUnit(velocity_vector.vertical_component.0 * time_interval.0),
    );
}
