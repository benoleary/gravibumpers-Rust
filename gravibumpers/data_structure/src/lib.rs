/// This crate defines the traits and structs for representing the particles from
/// initial set-up through time evolution to visualization.
///
/// As such it does not implement any logic and thus has no #[cfg(test)].

/// The particles have 3 intrinsic qualities which do not change, unlike their
/// positions and velocities.
pub struct ParticleIntrinsics {
    pub inertial_mass: f64,
    pub attractive_charge: f64,
    pub repulsive_charge: f64,
}

/// All the particles are encapsulated behind a ParticleCollection.
/// I could split the functionality of describing the particles for the time evolution
/// and of describing pixel-level aggregations for visualization across different traits
/// but I want to keep these concepts explicitly coupled.
pub trait ParticleCollection {
    fn aggregate_quantities_for_pixel(
        &self,
        horizontal_coordinate: i32,
        vertical_coordinate: i32,
    ) -> ParticleIntrinsics;
}
