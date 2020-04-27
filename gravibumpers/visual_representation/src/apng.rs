extern crate apng_encoder;
extern crate data_structure;

use super::particles_to_pixels::ParticleToPixelMapper;
use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::RedGreenBlueIntensity;
use super::SequenceAnimator;
use super::VerticalPixelAmount;
use std::convert::TryInto;

pub fn new(particle_to_pixel_mapper: Box<dyn ParticleToPixelMapper>) -> Box<dyn SequenceAnimator> {
    // I am sticking with the color palette from the apng_encoder example. It should be good enough
    // for my purposes.
    Box::new(ApngAnimator {
        color_palette: apng_encoder::Color::RGB(8),
        particle_to_pixel_mapper: particle_to_pixel_mapper,
    })
}

const MILLISECONDS_PER_SECOND: u16 = 1000;

struct ApngAnimator {
    color_palette: apng_encoder::Color,
    particle_to_pixel_mapper: Box<dyn ParticleToPixelMapper>,
}

impl SequenceAnimator for ApngAnimator {
    fn animate_sequence(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
        milliseconds_per_frame: u32,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let common_frame_information = apng_encoder::Frame {
            delay: Some(apng_encoder::Delay::new(
                milliseconds_per_frame.try_into()?,
                MILLISECONDS_PER_SECOND,
            )),
            ..Default::default()
        };

        let number_of_frames = particle_map_sequence.len();

        let meta_information = apng_encoder::Meta {
            width: self.particle_to_pixel_mapper.width_in_pixels().0,
            height: self.particle_to_pixel_mapper.height_in_pixels().0,
            color: self.color_palette,
            frames: number_of_frames.try_into()?,
            plays: Some(1),
        };

        let mut output_file = std::fs::File::create(output_filename).unwrap();

        let mut output_encoder =
            apng_encoder::Encoder::create(&mut output_file, meta_information).unwrap();

        let matrix_sequence = self
            .particle_to_pixel_mapper
            .aggregate_particle_colors_to_pixels(particle_map_sequence)?;

        for pixel_matrix in matrix_sequence.colored_pixel_matrices {
            let flattened_color_bytes = &flattened_color_bytes_from(
                &*pixel_matrix,
                &matrix_sequence.maximum_color_intensity,
            )?;
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

// This function creates the byte array specific to APNG representing the rectangle of triplets of
// floating-point numbers representing red-green-blue quantities.
fn flattened_color_bytes_from(
    pixel_matrix: &dyn ColoredPixelMatrix,
    maximum_color_intensity: &RedGreenBlueIntensity,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let width_in_pixels = pixel_matrix.width_in_pixels().0;
    let height_in_pixels = pixel_matrix.height_in_pixels().0;
    let flattened_length = 3 * width_in_pixels * height_in_pixels;
    let flattened_bytes = vec![0x00; flattened_length.try_into()?];

    for vertical_index in 0..height_in_pixels {
        // I prefer to think of drawing from the bottom-left to the right and up, but APNG lists the
        // bytes from top-left to right and down.
        let pixels_up = VerticalPixelAmount(height_in_pixels - vertical_index - 1);

        for horizontal_index in 0..width_in_pixels {
            // At this point we have already written sets of 3 colors for vertical_index whole
            // *rows* plus horizontal_index pixels in this row.
            let red_index = 3 * ((vertical_index * width_in_pixels) + horizontal_index);
            let green_index = red_index + 1;
            let blue_index = green_index + 1;

            let color_fractions_at_pixel = pixel_matrix.color_fractions_at(
                maximum_color_intensity,
                &HorizontalPixelAmount(horizontal_index),
                &pixels_up,
            );
        }
    }

    /*
        // RED   GREEN
        // BLACK BLUE
        test_encoder
            .write_frame(
                &[
                    // (x=0,y=0)            (x=1,y=0)
                    0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00,
                    // (x=0,y=1)            (x=1,y=1)
                    0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
                ],
                Some(&test_frame),
                None,
                None,
            )
            .unwrap();
    */
    // Still to do: actually fill the bytes, obviously. The above should guide me.
    Ok(flattened_bytes)
}

#[cfg(test)]
mod tests {
    use super::super::ColorFraction;
    use super::super::RedGreenBlueFraction;
    use super::super::RedGreenBlueIntensity;
    use super::*;
    use data_structure::ColorUnit;

    struct MockColoredPixelMatrix {}
    impl ColoredPixelMatrix for MockColoredPixelMatrix {
        fn color_fractions_at(
            &self,
            reference_intensity: &RedGreenBlueIntensity,
            horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
            vertical_pixels_from_bottom_left: &VerticalPixelAmount,
        ) -> Result<RedGreenBlueFraction, Box<dyn std::error::Error>> {
            match (
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
            ) {
                (HorizontalPixelAmount(0), VerticalPixelAmount(y)) => Ok(RedGreenBlueFraction {
                    red_fraction: ColorFraction(0.5 * (*y as f64)),
                    green_fraction: ColorFraction(0.5 * (*y as f64)),
                    blue_fraction: ColorFraction(0.5 * (*y as f64)),
                }),
                (HorizontalPixelAmount(1), VerticalPixelAmount(y)) => Ok(RedGreenBlueFraction {
                    red_fraction: ColorFraction(0.5 * (*y as f64)),
                    green_fraction: ColorFraction(0.0),
                    blue_fraction: ColorFraction(0.5 * (*y as f64)),
                }),
                (HorizontalPixelAmount(2), VerticalPixelAmount(y)) => Ok(RedGreenBlueFraction {
                    red_fraction: ColorFraction(0.5 * (*y as f64)),
                    green_fraction: ColorFraction(0.5 * (*y as f64)),
                    blue_fraction: ColorFraction(0.0),
                }),
                _ => Ok(RedGreenBlueFraction {
                    red_fraction: ColorFraction(0.0),
                    green_fraction: ColorFraction(0.0),
                    blue_fraction: ColorFraction(0.0),
                }),
            }
        }
        fn width_in_pixels(&self) -> HorizontalPixelAmount {
            HorizontalPixelAmount(4)
        }
        fn height_in_pixels(&self) -> VerticalPixelAmount {
            VerticalPixelAmount(3)
        }
    }

    #[test]
    fn test_flattened_color_bytes_from() {
        let mock_matrix = MockColoredPixelMatrix {};

        let full_intensity = RedGreenBlueIntensity {
            red_density: ColorUnit(1.0),
            green_density: ColorUnit(1.0),
            blue_density: ColorUnit(1.0),
        };

        #[rustfmt::skip]
        let expected_bytes: Vec<u8> = vec![
            // 0r    0g    0b    1r    1g    1b    2r    2g    2b    3r    3g    3b
            0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
            // 0r    0g    0b    1r    1g    1b    2r    2g    2b    3r    3g    3b
            0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00,
            // 0r    0g    0b    1r    1g    1b    2r    2g    2b    3r    3g    3b
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let flattened_color_bytes = flattened_color_bytes_from(&mock_matrix, &full_intensity)
            .expect("Mock should always return Ok(...)");

        assert_eq!(
            expected_bytes, flattened_color_bytes,
            "APNG bytes for a test frame, left is expected, right is actual"
        );
    }
}
