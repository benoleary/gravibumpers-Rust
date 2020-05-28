/// This crate defines the traits and structs for representing the particles from
/// initial set-up through time evolution to visualization.
///
/// As such it does not implement any logic and thus has no #[cfg(test)].
pub mod comparison;

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalPositionUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalVelocityUnit(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalVelocityUnit(pub f64);

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
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
pub struct ParticleVariables {
    pub horizontal_position: HorizontalPositionUnit,
    pub vertical_position: VerticalPositionUnit,
    pub horizontal_velocity: HorizontalVelocityUnit,
    pub vertical_velocity: VerticalVelocityUnit,
}

pub struct IndividualParticle {
    pub intrinsic_values: ParticleIntrinsics,
    pub variable_values: ParticleVariables,
}

pub trait ParticleIteratorProvider {
    fn get(&mut self) -> &mut dyn std::iter::ExactSizeIterator<Item = IndividualParticle>;
}

pub type ParticleIterator = dyn std::iter::ExactSizeIterator<Item = IndividualParticle>;
