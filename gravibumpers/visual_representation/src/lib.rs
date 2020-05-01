/// There is has no #[cfg(test)] in the main part of the library because it just introduces traits
/// and structs.
extern crate data_structure;
pub mod apng;
pub mod demonstration;
pub mod particles_to_pixels;
use std::error::Error;

#[derive(Debug)]
pub struct OutOfBoundsError {
    error_message: String,
}

impl OutOfBoundsError {
    pub fn new(error_message: &str) -> OutOfBoundsError {
        OutOfBoundsError {
            error_message: error_message.to_string(),
        }
    }
}

impl Error for OutOfBoundsError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl std::fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Out of bounds: {}", self.error_message)
    }
}

pub trait SequenceAnimator {
    fn animate_sequence(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
        milliseconds_per_frame: u32,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct RedGreenBlueIntensity {
    pub red_density: data_structure::ColorUnit,
    pub green_density: data_structure::ColorUnit,
    pub blue_density: data_structure::ColorUnit,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ColorFraction(pub f64);

impl std::ops::Mul<&data_structure::ColorUnit> for ColorFraction {
    type Output = data_structure::ColorUnit;

    fn mul(self, color_with_unit: &data_structure::ColorUnit) -> data_structure::ColorUnit {
        data_structure::ColorUnit(self.0 * color_with_unit.0)
    }
}

pub struct RedGreenBlueFraction {
    pub red_fraction: ColorFraction,
    pub green_fraction: ColorFraction,
    pub blue_fraction: ColorFraction,
}

impl std::ops::Mul<&RedGreenBlueIntensity> for RedGreenBlueFraction {
    type Output = RedGreenBlueIntensity;

    fn mul(self, intensity_triplet: &RedGreenBlueIntensity) -> RedGreenBlueIntensity {
        RedGreenBlueIntensity {
            red_density: (self.red_fraction * &intensity_triplet.red_density),
            green_density: (self.green_fraction * &intensity_triplet.green_density),
            blue_density: (self.blue_fraction * &intensity_triplet.blue_density),
        }
    }
}

/// The pixel co-ordinates are taken as from the bottom-left of the picture because that is how
/// I find it easiest to visualize.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct HorizontalPixelAmount(pub i32);

impl std::ops::Add<HorizontalPixelAmount> for HorizontalPixelAmount {
    type Output = HorizontalPixelAmount;

    fn add(self, other_amount: HorizontalPixelAmount) -> HorizontalPixelAmount {
        HorizontalPixelAmount(self.0 + other_amount.0)
    }
}

impl std::ops::Sub<HorizontalPixelAmount> for HorizontalPixelAmount {
    type Output = HorizontalPixelAmount;

    fn sub(self, other_amount: HorizontalPixelAmount) -> HorizontalPixelAmount {
        HorizontalPixelAmount(self.0 - other_amount.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VerticalPixelAmount(pub i32);

pub trait ColoredPixelMatrix {
    fn color_fractions_at(
        &self,
        reference_intensity: &RedGreenBlueIntensity,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<RedGreenBlueFraction, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
