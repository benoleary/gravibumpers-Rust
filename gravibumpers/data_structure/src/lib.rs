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

impl Add for RedColorUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GreenColorUnit(pub f64);

impl Add for GreenColorUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BlueColorUnit(pub f64);

impl Add for BlueColorUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorTriplet {
    red_brightness: RedColorUnit,
    green_brightness: GreenColorUnit,
    blue_brightness: BlueColorUnit,
}

impl std::ops::AddAssign for ColorTriplet {
    fn add_assign(&mut self, other_amount: Self) {
        self.red_brightness = self.red_brightness + other_amount.red_brightness;
        self.green_brightness = self.green_brightness + other_amount.green_brightness;
        self.blue_brightness = self.blue_brightness + other_amount.blue_brightness;
    }
}

impl ColorTriplet {
    pub fn get_red(&self) -> RedColorUnit {
        self.red_brightness
    }

    pub fn get_green(&self) -> GreenColorUnit {
        self.green_brightness
    }

    pub fn get_blue(&self) -> BlueColorUnit {
        self.blue_brightness
    }

    pub fn overwrite_each_color_if_brighter(&mut self, possible_brighter_values: &ColorTriplet) {
        if possible_brighter_values.red_brightness > self.red_brightness {
            self.red_brightness = possible_brighter_values.red_brightness;
        }
        if possible_brighter_values.green_brightness > self.green_brightness {
            self.green_brightness = possible_brighter_values.green_brightness;
        }
        if possible_brighter_values.blue_brightness > self.blue_brightness {
            self.blue_brightness = possible_brighter_values.blue_brightness;
        }
    }
}

pub fn new_color_triplet(
    red_brightness: RedColorUnit,
    green_brightness: GreenColorUnit,
    blue_brightness: BlueColorUnit,
) -> ColorTriplet {
    ColorTriplet {
        red_brightness: red_brightness,
        green_brightness: green_brightness,
        blue_brightness: blue_brightness,
    }
}

pub fn color_triplets_match(
    expected_triplet: &ColorTriplet,
    actual_triplet: &ColorTriplet,
    absolute_tolerance: f64,
) -> bool {
    ((expected_triplet.red_brightness.0 - actual_triplet.red_brightness.0).abs()
        <= absolute_tolerance)
        && ((expected_triplet.green_brightness.0 - actual_triplet.green_brightness.0).abs()
            <= absolute_tolerance)
        && ((expected_triplet.blue_brightness.0 - actual_triplet.blue_brightness.0).abs()
            <= absolute_tolerance)
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalPositionUnit(pub f64);

impl Add for HorizontalPositionUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalPositionUnit(pub f64);

impl Add for VerticalPositionUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
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
    pub color_brightness: ColorTriplet,
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

pub trait ParticleIteratorProvider<'a> {
    fn get<'b>(&mut self) -> &mut dyn std::iter::ExactSizeIterator<Item = &'a IndividualParticle>
    where
        'a: 'b;
}

pub struct BorrowedParticleIterator<'a> {
    borrowed_iterator: &'a mut dyn std::iter::ExactSizeIterator<Item = &'a IndividualParticle>,
}

impl<'a> ParticleIteratorProvider<'a> for BorrowedParticleIterator<'a> {
    fn get<'b>(&mut self) -> &mut dyn std::iter::ExactSizeIterator<Item = &'a IndividualParticle>
    where
        'a: 'b,
    {
        &mut self.borrowed_iterator
    }
}

pub struct ParticleVector {
    particle_vector: std::vec::Vec<IndividualParticle>,
}

impl<'a> ParticleIteratorProvider<'a> for ParticleVector<'a> {
    fn get<'b>(&mut self) -> &mut dyn std::iter::ExactSizeIterator<Item = &'a IndividualParticle>
    where
        'a: 'b,
    {
        // This is not working because I am thinking about it all wrong. I need to consider what
        // is being moved and what has ownership of what. It probably does have to be a
        // std::iter::ExactSizeIterator<Item = impl std::iter::ExactSizeIterator<Item = &'a IndividualParticle>>
        // in the end.
        &mut self.borrowed_iterator
    }
}

pub fn wrap_particle_vector<'a>(
    particle_vector: &'a std::vec::Vec<IndividualParticle>,
) -> BorrowedParticleIterator<'a> {
    BorrowedParticleIterator {
        borrowed_iterator: &mut (&particle_vector).into_iter(),
    }
}
