/// There is has no #[cfg(test)] in the main part of the library because it just introduces traits
/// and structs.
extern crate data_structure;
pub mod apng;
pub mod color;
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
    fn animate_sequence<T: data_structure::ParticleIteratorProvider>(
        &self,
        particle_map_sequence: &mut dyn std::iter::ExactSizeIterator<Item = &T>,
        milliseconds_per_frame: u32,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
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

impl std::ops::Sub<VerticalPixelAmount> for VerticalPixelAmount {
    type Output = VerticalPixelAmount;

    fn sub(self, other_amount: VerticalPixelAmount) -> VerticalPixelAmount {
        VerticalPixelAmount(self.0 - other_amount.0)
    }
}

pub trait ColoredPixelMatrix {
    fn color_fractions_at(
        &self,
        reference_brightness: &color::BrightnessTriplet,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<color::FractionTriplet, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
