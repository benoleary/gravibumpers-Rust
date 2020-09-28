/// This module provides an implementation of SequenceAnimator which produces a file in APNG
/// format.
extern crate apng_encoder;
extern crate data_structure;

use super::particles_to_pixels::ParticleToPixelMapper;
use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::SequenceAnimator;
use super::VerticalPixelAmount;
use std::convert::TryInto;

const MILLISECONDS_PER_SECOND: u16 = 1000;

const COLOR_DEPTH: apng_encoder::Color = apng_encoder::Color::RGB(8);

const MAXIMUM_COLOR_BYTE: u8 = 0xFF;

pub fn new<T: ParticleToPixelMapper>(
    particle_to_pixel_mapper: T,
    number_of_plays: u32,
) -> ApngAnimator<T> {
    // I am sticking with the color palette from the apng_encoder example. It should be good enough
    // for my purposes.
    ApngAnimator {
        color_palette: COLOR_DEPTH,
        particle_to_pixel_mapper: particle_to_pixel_mapper,
        number_of_plays: number_of_plays,
    }
}

pub struct ApngAnimator<T: ParticleToPixelMapper> {
    color_palette: apng_encoder::Color,
    particle_to_pixel_mapper: T,
    number_of_plays: u32,
}

impl<T: ParticleToPixelMapper> SequenceAnimator for ApngAnimator<T> {
    fn animate_sequence(
        &self,
        particle_map_sequence: impl std::iter::ExactSizeIterator<
            Item = impl std::iter::ExactSizeIterator<Item = impl data_structure::ParticleRepresentation>,
        >,
        milliseconds_per_frame: u16,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let common_frame_information = apng_encoder::Frame {
            delay: Some(apng_encoder::Delay::new(
                milliseconds_per_frame,
                MILLISECONDS_PER_SECOND,
            )),
            ..Default::default()
        };

        let number_of_frames = particle_map_sequence.len();

        let meta_information = apng_encoder::Meta {
            width: self
                .particle_to_pixel_mapper
                .width_in_pixels()
                .0
                .try_into()?,
            height: self
                .particle_to_pixel_mapper
                .height_in_pixels()
                .0
                .try_into()?,
            color: self.color_palette,
            frames: number_of_frames.try_into()?,
            plays: Some(self.number_of_plays),
        };

        let mut output_file = std::fs::File::create(output_filename).unwrap();
        let mut output_encoder =
            apng_encoder::Encoder::create(&mut output_file, meta_information).unwrap();

        let matrix_sequence = self
            .particle_to_pixel_mapper
            .aggregate_particle_colors_to_pixels(particle_map_sequence)?;

        for pixel_matrix in matrix_sequence.colored_pixel_matrices {
            let flattened_color_bytes =
                &flattened_color_bytes_from(pixel_matrix, &matrix_sequence.maximum_brightness)?;
            output_encoder
                .write_frame(
                    flattened_color_bytes,
                    Some(&common_frame_information),
                    None,
                    None,
                )
                .unwrap();
        }
        output_encoder.finish().unwrap();

        Ok(())
    }
}

fn ceiling_as_byte(color_intensity: f64) -> u8 {
    (color_intensity * (MAXIMUM_COLOR_BYTE as f64)).ceil() as u8
}

// This function creates the byte array specific to APNG representing the rectangle of triplets of
// floating-point numbers representing red-green-blue quantities.
fn flattened_color_bytes_from(
    pixel_matrix: impl ColoredPixelMatrix,
    maximum_color_intensity: &data_structure::AbsoluteColorUnit,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let width_in_pixels = pixel_matrix.width_in_pixels().0;
    let height_in_pixels = pixel_matrix.height_in_pixels().0;
    let flattened_length = 3 * width_in_pixels * height_in_pixels;
    let mut flattened_bytes = vec![0x00; flattened_length.try_into()?];

    for vertical_index in 0..height_in_pixels {
        // I prefer to think of drawing from the bottom-left to the right and up, but APNG lists the
        // bytes from top-left to right and down.
        let pixels_up = VerticalPixelAmount(height_in_pixels - vertical_index - 1);

        for horizontal_index in 0..width_in_pixels {
            // At this point we have already written sets of 3 colors for vertical_index whole
            // *rows* plus horizontal_index pixels in this row.
            let red_index = 3 * ((vertical_index * width_in_pixels) + horizontal_index) as usize;
            let green_index = red_index + 1;
            let blue_index = green_index + 1;

            let color_fractions_at_pixel = pixel_matrix.color_fractions_at(
                maximum_color_intensity,
                &HorizontalPixelAmount(horizontal_index),
                &pixels_up,
            )?;

            let color_triplet = color_fractions_at_pixel * maximum_color_intensity;

            flattened_bytes[red_index] = ceiling_as_byte(color_triplet.get_red().0);
            flattened_bytes[green_index] = ceiling_as_byte(color_triplet.get_green().0);
            flattened_bytes[blue_index] = ceiling_as_byte(color_triplet.get_blue().0);
        }
    }
    Ok(flattened_bytes)
}

#[cfg(test)]
mod tests {
    use super::super::color::FractionTriplet as ColorFraction;
    use super::super::OutOfBoundsError;
    use super::*;

    const MAX_BYTE: u8 = 0xFF;
    const HALF_BYTE: u8 = 0x80;
    const ZERO_BYTE: u8 = 0x00;

    struct MockColoredPixelMatrix {}
    impl ColoredPixelMatrix for MockColoredPixelMatrix {
        fn color_fractions_at(
            &self,
            _reference_intensity: &data_structure::AbsoluteColorUnit,
            horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
            vertical_pixels_from_bottom_left: &VerticalPixelAmount,
        ) -> Result<ColorFraction, Box<dyn std::error::Error>> {
            match (
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
            ) {
                (HorizontalPixelAmount(0), VerticalPixelAmount(y)) => {
                    Ok(super::super::color::fraction_from_values(
                        0.5 * (*y as f64),
                        0.5 * (*y as f64),
                        0.5 * (*y as f64),
                    ))
                }
                (HorizontalPixelAmount(1), VerticalPixelAmount(y)) => {
                    Ok(super::super::color::fraction_from_values(
                        0.5 * (*y as f64),
                        0.0,
                        0.5 * (*y as f64),
                    ))
                }
                (HorizontalPixelAmount(2), VerticalPixelAmount(y)) => {
                    Ok(super::super::color::fraction_from_values(
                        0.5 * (*y as f64),
                        0.5 * (*y as f64),
                        0.0,
                    ))
                }
                (HorizontalPixelAmount(3), VerticalPixelAmount(_)) => {
                    Ok(super::super::color::fraction_from_values(0.0, 0.0, 0.0))
                }
                _ => Err(Box::new(OutOfBoundsError::new(&format!(
                    "horizontal_pixels_from_bottom_left {}, vertical_pixels_from_bottom_left {}",
                    horizontal_pixels_from_bottom_left.0, vertical_pixels_from_bottom_left.0
                )))),
            }
        }
        fn width_in_pixels(&self) -> &HorizontalPixelAmount {
            &HorizontalPixelAmount(4)
        }
        fn height_in_pixels(&self) -> &VerticalPixelAmount {
            &VerticalPixelAmount(3)
        }
    }

    #[test]
    fn test_flattened_color_bytes_from() {
        let mock_matrix = MockColoredPixelMatrix {};

        let full_intensity = data_structure::AbsoluteColorUnit(1.0);

        #[rustfmt::skip]
        let expected_bytes: Vec<u8> = vec![
            //    0r        0g        0b        1r         1g        1b
            //        2r        2g         2b         3r         3g         3b
            MAX_BYTE, MAX_BYTE, MAX_BYTE, MAX_BYTE, ZERO_BYTE, MAX_BYTE,
                MAX_BYTE, MAX_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE,
            //     0r         0g         0b         1r         1g         1b
            //         2r         2g         2b         3r         3g         3b
            HALF_BYTE, HALF_BYTE, HALF_BYTE, HALF_BYTE, ZERO_BYTE, HALF_BYTE,
                HALF_BYTE, HALF_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE,
            //     0r         0g         0b         1r         1g         1b
            //         2r         2g         2b         3r         3g         3b
            ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE,
                ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE, ZERO_BYTE,
        ];

        let flattened_color_bytes = flattened_color_bytes_from(mock_matrix, &full_intensity)
            .expect("Mock should always return Ok(...)");

        assert_eq!(
            expected_bytes, flattened_color_bytes,
            "APNG bytes for a test frame, left is expected, right is actual"
        );
    }
}
