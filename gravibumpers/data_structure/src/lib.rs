/// This crate defines the traits and structs for representing the particles from initial set-up
/// through time evolution to visualization.
///
/// As such this main lib file does not implement any logic (except some trivial implementations
/// of Add and a default unpacking for a trait) and thus has no #[cfg(test)].
///
/// There is a public module (comparison) but this exists to provide utility functions for tests,
/// so itself also has no #[cfg(test)].
pub mod comparison;
use std::error::Error;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

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

/// This is the unit for the charges on the particles for the inverse-displacement-squared force
/// but the factor for getting an acceleration in pixels per squared second per inertial mass unit
/// is a dimensionful configuration parameter.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct InverseSquaredChargeUnit(pub f64);

/// This is the unit for the charges on the particles for the inverse-displacement-to-the-fourth
/// force but the factor for getting an acceleration in pixels per squared second per inertial mass
/// unit is a dimensionful configuration parameter.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct InverseFourthChargeUnit(pub f64);

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AbsoluteColorUnit(pub f64);

impl AbsoluteColorUnit {
    pub fn update_to_other_if_brighter(&mut self, other_amount: &Self) {
        self.0 = self.0.max(other_amount.0)
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

    pub fn get_total(&self) -> AbsoluteColorUnit {
        AbsoluteColorUnit(self.red_brightness.0 + self.green_brightness.0 + self.blue_brightness.0)
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
    relative_tolerance: f64,
) -> bool {
    comparison::within_relative_tolerance(
        expected_triplet.red_brightness.0,
        actual_triplet.red_brightness.0,
        relative_tolerance,
    ) && comparison::within_relative_tolerance(
        expected_triplet.green_brightness.0,
        actual_triplet.green_brightness.0,
        relative_tolerance,
    ) && comparison::within_relative_tolerance(
        expected_triplet.blue_brightness.0,
        actual_triplet.blue_brightness.0,
        relative_tolerance,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct InertialMassUnit(pub f64);

// This corresponds to seconds so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TimeDifferenceUnit(pub f64);

// This particular combination has no physical meaning but allows for an efficient preparation of a
// constant multiplicative factor.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TimeOverMassUnit(pub f64);

pub fn divide_time_by_mass(
    time_quantity: &TimeDifferenceUnit,
    mass_quantity: &InertialMassUnit,
) -> Result<TimeOverMassUnit, Box<dyn std::error::Error>> {
    if mass_quantity.0 == 0.0 {
        Err(Box::new(DimensionError::new(&format!(
            "Cannot divide time {:?} by zero mass.",
            time_quantity
        ))))
    } else {
        Ok(TimeOverMassUnit(time_quantity.0 / mass_quantity.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalForceUnit(pub f64);

impl Add for HorizontalForceUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for HorizontalForceUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalForceUnit(pub f64);

impl Add for VerticalForceUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for VerticalForceUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ForceVector {
    pub horizontal_component: HorizontalForceUnit,
    pub vertical_component: VerticalForceUnit,
}

impl AddAssign for ForceVector {
    fn add_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component + other_amount.horizontal_component;
        self.vertical_component = self.vertical_component + other_amount.vertical_component;
    }
}

impl SubAssign for ForceVector {
    fn sub_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component - other_amount.horizontal_component;
        self.vertical_component = self.vertical_component - other_amount.vertical_component;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalVelocityUnit(pub f64);

impl Add for HorizontalVelocityUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for HorizontalVelocityUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalVelocityUnit(pub f64);

impl Add for VerticalVelocityUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for VerticalVelocityUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VelocityVector {
    pub horizontal_component: HorizontalVelocityUnit,
    pub vertical_component: VerticalVelocityUnit,
}

impl VelocityVector {}

impl AddAssign for VelocityVector {
    fn add_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component + other_amount.horizontal_component;
        self.vertical_component = self.vertical_component + other_amount.vertical_component;
    }
}

impl SubAssign for VelocityVector {
    fn sub_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component - other_amount.horizontal_component;
        self.vertical_component = self.vertical_component - other_amount.vertical_component;
    }
}

pub fn velocity_change_from_force(
    applied_force: &ForceVector,
    timestep_over_inertial_mass: &TimeOverMassUnit,
) -> VelocityVector {
    VelocityVector {
        horizontal_component: HorizontalVelocityUnit(
            applied_force.horizontal_component.0 * timestep_over_inertial_mass.0,
        ),
        vertical_component: VerticalVelocityUnit(
            applied_force.vertical_component.0 * timestep_over_inertial_mass.0,
        ),
    }
}

pub fn sum_velocity_with_scaled_velocity(
    base_velocity: &VelocityVector,
    velocity_to_scale: &VelocityVector,
    scaling_factor: f64,
) -> VelocityVector {
    VelocityVector {
        horizontal_component: HorizontalVelocityUnit(
            base_velocity.horizontal_component.0
                + (scaling_factor * velocity_to_scale.horizontal_component.0),
        ),
        vertical_component: VerticalVelocityUnit(
            base_velocity.vertical_component.0
                + (scaling_factor * velocity_to_scale.vertical_component.0),
        ),
    }
}

// This corresponds to pixels so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalPositionUnit(pub f64);

impl Add for HorizontalPositionUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for HorizontalPositionUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

impl AddAssign for HorizontalPositionUnit {
    fn add_assign(&mut self, other_amount: Self) {
        self.0 = self.0 + other_amount.0;
    }
}

// This corresponds to pixels so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalPositionUnit(pub f64);

impl Add for VerticalPositionUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for VerticalPositionUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

impl AddAssign for VerticalPositionUnit {
    fn add_assign(&mut self, other_amount: Self) {
        self.0 = self.0 + other_amount.0;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PositionVector {
    pub horizontal_component: HorizontalPositionUnit,
    pub vertical_component: VerticalPositionUnit,
}

impl AddAssign for PositionVector {
    fn add_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component + other_amount.horizontal_component;
        self.vertical_component = self.vertical_component + other_amount.vertical_component;
    }
}

impl SubAssign for PositionVector {
    fn sub_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component - other_amount.horizontal_component;
        self.vertical_component = self.vertical_component - other_amount.vertical_component;
    }
}

impl Add for PositionVector {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self {
            horizontal_component: self.horizontal_component + other_amount.horizontal_component,
            vertical_component: self.vertical_component + other_amount.vertical_component,
        }
    }
}

impl Sub for PositionVector {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self {
            horizontal_component: self.horizontal_component - other_amount.horizontal_component,
            vertical_component: self.vertical_component - other_amount.vertical_component,
        }
    }
}

impl PositionVector {
    pub fn increment_by_velocity_for_time_difference(
        &mut self,
        velocity_vector: &VelocityVector,
        time_difference: &TimeDifferenceUnit,
    ) {
        self.horizontal_component +=
            HorizontalPositionUnit(velocity_vector.horizontal_component.0 * time_difference.0);
        self.vertical_component +=
            VerticalPositionUnit(velocity_vector.vertical_component.0 * time_difference.0);
    }
}

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct ParticleIntrinsics {
    pub inertial_mass: InertialMassUnit,
    pub inverse_squared_charge: InverseSquaredChargeUnit,
    pub inverse_fourth_charge: InverseFourthChargeUnit,
    pub color_brightness: ColorTriplet,
}

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct ParticleVariables {
    pub position_vector: PositionVector,
    pub velocity_vector: VelocityVector,
}

pub trait ParticleRepresentation {
    fn read_intrinsics(&self) -> &ParticleIntrinsics;
    fn read_variables(&self) -> &ParticleVariables;
}

#[derive(Clone, Copy, Debug)]
pub struct IndividualParticle {
    pub intrinsic_values: ParticleIntrinsics,
    pub variable_values: ParticleVariables,
}

impl ParticleRepresentation for IndividualParticle {
    fn read_intrinsics(&self) -> &ParticleIntrinsics {
        &self.intrinsic_values
    }
    fn read_variables(&self) -> &ParticleVariables {
        &self.variable_values
    }
}

impl ParticleRepresentation for &IndividualParticle {
    fn read_intrinsics(&self) -> &ParticleIntrinsics {
        &self.intrinsic_values
    }
    fn read_variables(&self) -> &ParticleVariables {
        &self.variable_values
    }
}

pub fn create_individual_from_representation(
    particle_representation: &impl ParticleRepresentation,
) -> IndividualParticle {
    IndividualParticle {
        intrinsic_values: *particle_representation.read_intrinsics(),
        variable_values: *particle_representation.read_variables(),
    }
}

/// This holds the inverse of a distance, so 1/r.
#[derive(Clone, Copy, Debug)]
pub struct InverseSeparationUnit(f64);

impl InverseSeparationUnit {
    pub fn get_value(&self) -> f64 {
        self.0
    }
}

/// This represents the square of the Euclidean distance between two co-ordinates in pixels.
#[derive(Clone, Copy, Debug)]
pub struct SquaredSeparationUnit(pub f64);

pub fn square_separation_vector(separation_vector: &PositionVector) -> SquaredSeparationUnit {
    let horizontal_squared =
        separation_vector.horizontal_component.0 * separation_vector.horizontal_component.0;
    let vertical_squared =
        separation_vector.vertical_component.0 * separation_vector.vertical_component.0;
    SquaredSeparationUnit(horizontal_squared + vertical_squared)
}

impl SquaredSeparationUnit {
    pub fn to_inverse_square_root(&self) -> InverseSeparationUnit {
        InverseSeparationUnit(1.0 / self.0.sqrt())
    }

    pub fn is_greater_than_square(&self, separation_vector: &PositionVector) -> bool {
        self.0 > square_separation_vector(separation_vector).0
    }
}

/// This represents the Euclidean distance between two co-ordinates in pixels.
#[derive(Clone, Copy, Debug)]
pub struct SeparationUnit(pub f64);

impl SeparationUnit {
    fn to_square(&self) -> SquaredSeparationUnit {
        SquaredSeparationUnit(self.0 * self.0)
    }

    pub fn is_greater_than_square(&self, separation_vector: &PositionVector) -> bool {
        self.to_square().is_greater_than_square(separation_vector)
    }

    pub fn is_greater_than_difference(
        &self,
        first_position: &PositionVector,
        second_position: &PositionVector,
    ) -> (bool, SquaredSeparationUnit) {
        let difference_vector = *first_position - *second_position;
        let squared_separation = square_separation_vector(&difference_vector);
        (
            self.is_greater_than_square(&difference_vector),
            squared_separation,
        )
    }
}

/// This returns 1 divided by either the separation of the two given points or by the given minimum,
/// whichever is greater (so giving an upper bound on the returned inverse separation). It does not
/// guard against division by zero!
pub fn get_capped_inverse_separation(
    first_position: &PositionVector,
    second_position: &PositionVector,
    minimum_separation: &SeparationUnit,
) -> InverseSeparationUnit {
    let (is_capped, squared_separation) =
        minimum_separation.is_greater_than_difference(first_position, second_position);

    if is_capped {
        InverseSeparationUnit(1.0 / minimum_separation.0)
    } else {
        squared_separation.to_inverse_square_root()
    }
}
