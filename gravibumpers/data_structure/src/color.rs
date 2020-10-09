/// This module provides structs for representing colors as dimensionful quantities and a utility
/// function for creation.
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RedUnit(pub f64);

impl Add for RedUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GreenUnit(pub f64);

impl Add for GreenUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BlueUnit(pub f64);

impl Add for BlueUnit {
    type Output = Self;

    fn add(self, other_amount: Self) -> Self {
        Self(self.0 + other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AbsoluteUnit(pub f64);

impl AbsoluteUnit {
    pub fn update_to_other_if_brighter(&mut self, other_amount: &Self) {
        self.0 = self.0.max(other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedGreenBlueTriplet {
    red_brightness: RedUnit,
    green_brightness: GreenUnit,
    blue_brightness: BlueUnit,
}

impl std::ops::AddAssign for RedGreenBlueTriplet {
    fn add_assign(&mut self, other_amount: Self) {
        self.red_brightness = self.red_brightness + other_amount.red_brightness;
        self.green_brightness = self.green_brightness + other_amount.green_brightness;
        self.blue_brightness = self.blue_brightness + other_amount.blue_brightness;
    }
}

impl RedGreenBlueTriplet {
    pub fn get_red(&self) -> RedUnit {
        self.red_brightness
    }

    pub fn get_green(&self) -> GreenUnit {
        self.green_brightness
    }

    pub fn get_blue(&self) -> BlueUnit {
        self.blue_brightness
    }

    pub fn get_total(&self) -> AbsoluteUnit {
        AbsoluteUnit(self.red_brightness.0 + self.green_brightness.0 + self.blue_brightness.0)
    }
}

pub fn new_triplet(
    red_brightness: RedUnit,
    green_brightness: GreenUnit,
    blue_brightness: BlueUnit,
) -> RedGreenBlueTriplet {
    RedGreenBlueTriplet {
        red_brightness: red_brightness,
        green_brightness: green_brightness,
        blue_brightness: blue_brightness,
    }
}
