/// This module provides structs for representing forces as dimensionful quantities.
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

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
