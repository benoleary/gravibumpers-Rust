/// This crate provides structs, traits, and functions for turning sequences of particle
/// collections into an animated visual representation.
///
/// There is has no #[cfg(test)] in the main file of the library because it just introduces traits
/// and structs, along with a few trivial implementations of some standard traits.
extern crate data_structure;
pub mod apng;
pub mod brightness_aggregator;
pub mod color;
pub mod demonstration;
pub mod particles_to_pixels;
use std::error::Error;

#[derive(Debug)]
pub struct OutOfBoundsError {
    error_message: String,
}

impl OutOfBoundsError {
    pub fn new(error_message: &str) -> Self {
        Self {
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
        particle_map_sequence: impl std::iter::ExactSizeIterator<
            Item = impl std::iter::ExactSizeIterator<Item = impl data_structure::ParticleRepresentation>,
        >,
        milliseconds_per_frame: u16,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// The pixel co-ordinates are taken as from the bottom-left of the picture because that is how
/// I find it easiest to visualize.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct HorizontalPixelAmount(pub i32);

pub fn new_horizontal_pixel_unit_rounding_to_negative_infinity(
    horizontal_coordinate: data_structure::HorizontalPositionUnit,
) -> HorizontalPixelAmount {
    if horizontal_coordinate.0 < 0.0 {
        HorizontalPixelAmount(horizontal_coordinate.0 as i32 - 1)
    } else {
        HorizontalPixelAmount(horizontal_coordinate.0 as i32)
    }
}

impl HorizontalPixelAmount {
    pub fn abs_as_usize(&self) -> usize {
        // The abs takes care of the unsigned, and u32 must fit inside usize.
        self.0.abs() as usize
    }

    pub fn as_position_unit(&self) -> data_structure::HorizontalPositionUnit {
        data_structure::HorizontalPositionUnit(self.0 as f64)
    }
}

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

pub fn new_vertical_pixel_unit_rounding_to_negative_infinity(
    vertical_coordinate: data_structure::VerticalPositionUnit,
) -> VerticalPixelAmount {
    if vertical_coordinate.0 < 0.0 {
        VerticalPixelAmount(vertical_coordinate.0 as i32 - 1)
    } else {
        VerticalPixelAmount(vertical_coordinate.0 as i32)
    }
}

impl VerticalPixelAmount {
    pub fn abs_as_usize(&self) -> usize {
        // The abs takes care of the unsigned, and u32 must fit inside usize.
        self.0.abs() as usize
    }

    pub fn as_position_unit(&self) -> data_structure::VerticalPositionUnit {
        data_structure::VerticalPositionUnit(self.0 as f64)
    }
}

impl std::ops::Add<VerticalPixelAmount> for VerticalPixelAmount {
    type Output = VerticalPixelAmount;

    fn add(self, other_amount: VerticalPixelAmount) -> VerticalPixelAmount {
        VerticalPixelAmount(self.0 + other_amount.0)
    }
}

impl std::ops::Sub<VerticalPixelAmount> for VerticalPixelAmount {
    type Output = VerticalPixelAmount;

    fn sub(self, other_amount: VerticalPixelAmount) -> VerticalPixelAmount {
        VerticalPixelAmount(self.0 - other_amount.0)
    }
}

pub trait ColoredPixelMatrix {
    fn color_fractions_at(
        &self,
        reference_brightness: &data_structure::ColorTriplet,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<color::FractionTriplet, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
