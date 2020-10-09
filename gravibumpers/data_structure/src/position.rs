/// This module provides structs for representing positions and separations as dimensionful
/// quantities.
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

// This corresponds to pixels so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HorizontalUnit(pub f64);

impl Add for HorizontalUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for HorizontalUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

impl AddAssign for HorizontalUnit {
    fn add_assign(&mut self, other_amount: Self) {
        self.0 = self.0 + other_amount.0;
    }
}

// This corresponds to pixels so as to keep things reasonable to estimate.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VerticalUnit(pub f64);

impl Add for VerticalUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

impl Sub for VerticalUnit {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self(self.0 - other_amount.0)
    }
}

impl AddAssign for VerticalUnit {
    fn add_assign(&mut self, other_amount: Self) {
        self.0 = self.0 + other_amount.0;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DimensionfulVector {
    pub horizontal_component: HorizontalUnit,
    pub vertical_component: VerticalUnit,
}

impl AddAssign for DimensionfulVector {
    fn add_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component + other_amount.horizontal_component;
        self.vertical_component = self.vertical_component + other_amount.vertical_component;
    }
}

impl SubAssign for DimensionfulVector {
    fn sub_assign(&mut self, other_amount: Self) {
        self.horizontal_component = self.horizontal_component - other_amount.horizontal_component;
        self.vertical_component = self.vertical_component - other_amount.vertical_component;
    }
}

impl Add for DimensionfulVector {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self {
            horizontal_component: self.horizontal_component + other_amount.horizontal_component,
            vertical_component: self.vertical_component + other_amount.vertical_component,
        }
    }
}

impl Sub for DimensionfulVector {
    type Output = Self;

    fn sub(self, other_amount: Self) -> Self {
        Self {
            horizontal_component: self.horizontal_component - other_amount.horizontal_component,
            vertical_component: self.vertical_component - other_amount.vertical_component,
        }
    }
}

impl DimensionfulVector {
    pub fn increment_by_components(
        &mut self,
        horizontal_increment: &HorizontalUnit,
        vertical_increment: &VerticalUnit,
    ) {
        self.horizontal_component += *horizontal_increment;
        self.vertical_component += *vertical_increment;
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

pub fn square_separation_vector(separation_vector: &DimensionfulVector) -> SquaredSeparationUnit {
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

    pub fn is_greater_than_square(&self, separation_vector: &DimensionfulVector) -> bool {
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

    pub fn is_greater_than_square(&self, separation_vector: &DimensionfulVector) -> bool {
        self.to_square().is_greater_than_square(separation_vector)
    }

    pub fn is_greater_than_difference(
        &self,
        first_position: &DimensionfulVector,
        second_position: &DimensionfulVector,
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
    first_position: &DimensionfulVector,
    second_position: &DimensionfulVector,
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
