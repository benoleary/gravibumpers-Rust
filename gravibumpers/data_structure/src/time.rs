/// This module provides structs for representing time and time-divided-by-mass (a convenient
/// calculation aid) as dimensionful quantities.

// This corresponds to seconds so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct IntervalUnit(pub f64);

// This particular combination has no physical meaning but allows for an efficient preparation of a
// constant multiplicative factor.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct OverMassUnit(pub f64);

pub fn divide_time_by_mass(
    time_quantity: &IntervalUnit,
    mass_quantity: &super::charge::InertialMassUnit,
) -> Result<OverMassUnit, Box<dyn std::error::Error>> {
    if mass_quantity.0 == 0.0 {
        Err(Box::new(super::DimensionError::new(&format!(
            "Cannot divide time {:?} by zero mass.",
            time_quantity
        ))))
    } else {
        Ok(OverMassUnit(time_quantity.0 / mass_quantity.0))
    }
}
