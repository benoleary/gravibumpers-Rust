/// This crate defines the traits and structs for representing the particles from
/// initial set-up through time evolution to visualization.
///
/// As such it does not implement any logic and thus has no #[cfg(test)].

/// First we have some structs for dimensional parameters.
pub struct InertialMassUnit(pub f64);
pub struct AttractiveChargeUnit(pub f64);
pub struct RepulsiveChargeUnit(pub f64);
pub struct ColorUnit(pub f64);

/// The particles have 3 intrinsic qualities which do not change, unlike their
/// positions and velocities.
pub struct ParticleIntrinsics {
    pub inertial_mass: InertialMassUnit,
    pub attractive_charge: AttractiveChargeUnit,
    pub repulsive_charge: RepulsiveChargeUnit,
}

/// All the particles are encapsulated behind a ParticleCollection.
/// This definition is going to change a lot.
pub trait ParticleCollection {}
