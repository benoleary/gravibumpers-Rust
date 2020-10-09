/// This module provides structs for representing couplings as dimensionful quantities.

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
pub struct InertialMassUnit(pub f64);
