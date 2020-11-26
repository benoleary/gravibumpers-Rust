/// This crate provides structs, traits, and functions for turning sequences of particle
/// collections into an animated visual representation.
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
            Item = impl std::iter::ExactSizeIterator<
                Item = impl data_structure::particle::IndividualRepresentation,
            >,
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
    horizontal_coordinate: data_structure::position::HorizontalUnit,
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

    pub fn as_position_unit(&self) -> data_structure::position::HorizontalUnit {
        data_structure::position::HorizontalUnit(self.0 as f64)
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
    vertical_coordinate: data_structure::position::VerticalUnit,
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

    pub fn as_position_unit(&self) -> data_structure::position::VerticalUnit {
        data_structure::position::VerticalUnit(self.0 as f64)
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
        reference_brightness: &data_structure::color::AbsoluteUnit,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<color::FractionTriplet, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pixel_rounding() -> Result<(), String> {
        let mut failure_messages: std::vec::Vec<String> = vec![];
        for (input_float, expected_int) in vec![
            (-101.1, -102),
            (-10.0, -10),
            (-1.5, -2),
            (-0.9, -1),
            (0.0, 0),
            (0.11, 0),
            (1.11, 1),
            (2.99, 2),
            (9000.001, 9000),
        ] {
            let actual_horizontal = super::new_horizontal_pixel_unit_rounding_to_negative_infinity(
                data_structure::position::HorizontalUnit(input_float),
            );
            if actual_horizontal.0 != expected_int {
                failure_messages.push(String::from(format!(
                    "input f64 = {}, actual_horizontal = {:?}, expected_int = {}",
                    input_float, actual_horizontal, expected_int
                )));
            }
            let actual_vertical = super::new_vertical_pixel_unit_rounding_to_negative_infinity(
                data_structure::position::VerticalUnit(input_float),
            );
            if actual_vertical.0 != expected_int {
                failure_messages.push(String::from(format!(
                    "input f64 = {}, actual_vertical = {:?}, expected_int = {}",
                    input_float, actual_vertical, expected_int
                )));
            }
        }

        if failure_messages.is_empty() {
            Ok(())
        } else {
            Err(failure_messages.join("\n"))
        }
    }
}
