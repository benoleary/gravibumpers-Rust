/// This crate defines the traits and structs for representing the particles from initial set-up
/// through time evolution to visualization.
///
/// As such this main lib file does not implement any logic (except some trivial implementations
/// of Add and a default unpacking for a trait) and thus has no #[cfg(test)].
///
/// There is a public module (comparison) but this exists to provide utility functions for tests,
/// so itself also has no #[cfg(test)].
pub mod comparison;
use std::ops::Add;

/// First we have some structs for dimensional parameters.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct InertialMassUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AttractiveChargeUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RepulsiveChargeUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RedColorUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GreenColorUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BlueColorUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalPositionUnit(pub f64);

impl Add for HorizontalPositionUnit {
    type Output = HorizontalPositionUnit;

    fn add(self, horizontal_position: HorizontalPositionUnit) -> HorizontalPositionUnit {
        HorizontalPositionUnit(self.0 + horizontal_position.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalPositionUnit(pub f64);

impl Add for VerticalPositionUnit {
    type Output = VerticalPositionUnit;

    fn add(self, vertical_position: VerticalPositionUnit) -> VerticalPositionUnit {
        VerticalPositionUnit(self.0 + vertical_position.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalVelocityUnit(pub f64);

impl Add for HorizontalVelocityUnit {
    type Output = HorizontalVelocityUnit;

    fn add(self, horizontal_velocity: HorizontalVelocityUnit) -> HorizontalVelocityUnit {
        HorizontalVelocityUnit(self.0 + horizontal_velocity.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalVelocityUnit(pub f64);

impl Add for VerticalVelocityUnit {
    type Output = VerticalVelocityUnit;

    fn add(self, vertical_velocity: VerticalVelocityUnit) -> VerticalVelocityUnit {
        VerticalVelocityUnit(self.0 + vertical_velocity.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PositionVector {
    pub horizontal_position: HorizontalPositionUnit,
    pub vertical_position: VerticalPositionUnit,
}

#[derive(Clone, Copy, Debug)]
pub struct VelocityVector {
    pub horizontal_velocity: HorizontalVelocityUnit,
    pub vertical_velocity: VerticalVelocityUnit,
}

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct ParticleIntrinsics {
    pub inertial_mass: InertialMassUnit,
    pub attractive_charge: AttractiveChargeUnit,
    pub repulsive_charge: RepulsiveChargeUnit,
    pub red_brightness: RedColorUnit,
    pub green_brightness: GreenColorUnit,
    pub blue_brightness: BlueColorUnit,
}

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct ParticleVariables {
    pub horizontal_position: HorizontalPositionUnit,
    pub vertical_position: VerticalPositionUnit,
    pub horizontal_velocity: HorizontalVelocityUnit,
    pub vertical_velocity: VerticalVelocityUnit,
}

#[derive(Clone, Copy, Debug)]
pub struct IndividualParticle {
    pub intrinsic_values: ParticleIntrinsics,
    pub variable_values: ParticleVariables,
}

pub trait ParticleIteratorProvider {
    fn get<'a>(&'a mut self)
        -> &'a mut dyn std::iter::ExactSizeIterator<Item = IndividualParticle>;
}

pub struct ParticleVector {
    particle_iterator: Box<dyn std::iter::ExactSizeIterator<Item = IndividualParticle>>,
}

impl ParticleIteratorProvider for ParticleVector {
    fn get<'a>(
        &'a mut self,
    ) -> &'a mut dyn std::iter::ExactSizeIterator<Item = IndividualParticle> {
        &mut self.particle_iterator
    }
}

pub fn wrap_particle_vector(particle_vector: std::vec::Vec<IndividualParticle>) -> ParticleVector {
    ParticleVector {
        particle_iterator: Box::new(particle_vector.into_iter()),
    }
}
