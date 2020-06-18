/// This module provides implementations of ColoredPixelMatrix and
/// particles_to_pixels::ParticleToPixelMapper which perform the basic functionality of
/// rounding particle co-ordinates to pixel co-ordinates, aggregating the color brightnesses
/// which land in each pixel.
use super::color::BrightnessTriplet as ColorBrightness;
use super::color::FractionTriplet as ColorFraction;
use super::particles_to_pixels::ColoredPixelMatrixSequence as PixelMatrixSequence;
use super::HorizontalPixelAmount;
use super::OutOfBoundsError;
use super::VerticalPixelAmount;

pub struct AggregatedBrightnessMatrix {
    brightness_matrix: std::vec::Vec<std::vec::Vec<ColorBrightness>>,
    width_in_pixels_including_border: HorizontalPixelAmount,
    height_in_pixels_including_border: VerticalPixelAmount,
}

impl super::ColoredPixelMatrix for AggregatedBrightnessMatrix {
    fn color_fractions_at(
        &self,
        reference_brightness: &ColorBrightness,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<ColorFraction, Box<dyn std::error::Error>> {
        let height_index = vertical_pixels_from_bottom_left.0;
        let width_index = vertical_pixels_from_bottom_left.0;
        if (horizontal_pixels_from_bottom_left >= self.width_in_pixels())
            || (vertical_pixels_from_bottom_left >= self.height_in_pixels())
            || (height_index < 0)
            || (width_index < 0)
        {
            return Err(Box::new(OutOfBoundsError::new(&format!(
                "horizontal_pixels_from_bottom_left {:?}, vertical_pixels_from_bottom_left {:?} \
                - width {:?}, height {:?}",
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
                self.width_in_pixels_including_border,
                self.height_in_pixels_including_border
            ))));
        }

        // We have checked that the height and width indices are not negative already, so the cast
        // to a larger-sized but unsigned type will work.
        super::color::fraction_from_triplets(
            &self.brightness_matrix[height_index as usize][width_index as usize],
            reference_brightness,
        )
    }

    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &self.width_in_pixels_including_border
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &self.height_in_pixels_including_border
    }
}

pub struct PixelBrightnessAggregator {
    horizontal_offset_of_origin_from_picture_bottom_left: HorizontalPixelAmount,
    vertical_offset_of_origin_from_picture_bottom_left: VerticalPixelAmount,
    width_in_pixels_including_border: HorizontalPixelAmount,
    height_in_pixels_including_border: VerticalPixelAmount,
}

pub fn new(
    left_border: HorizontalPixelAmount,
    right_border: HorizontalPixelAmount,
    upper_border: VerticalPixelAmount,
    lower_border: VerticalPixelAmount,
) -> PixelBrightnessAggregator {
    PixelBrightnessAggregator {
        horizontal_offset_of_origin_from_picture_bottom_left: HorizontalPixelAmount(
            (right_border + left_border).0 / 2,
        ),
        vertical_offset_of_origin_from_picture_bottom_left: VerticalPixelAmount(
            (upper_border + lower_border).0 / 2,
        ),
        width_in_pixels_including_border: HorizontalPixelAmount((right_border - left_border).0),
        height_in_pixels_including_border: VerticalPixelAmount((upper_border - lower_border).0),
    }
}

impl PixelBrightnessAggregator {
    fn aggregate_over_particle_iterator(
        &self,
        particles_to_draw: impl std::iter::ExactSizeIterator<Item = data_structure::IndividualParticle>,
    ) -> AggregatedBrightnessMatrix {
        let mut aggregated_brightnesses = AggregatedBrightnessMatrix {
            brightness_matrix: vec![
                vec![
                    super::color::zero_brightness();
                    self.width_in_pixels_including_border.abs_as_usize()
                ];
                self.height_in_pixels_including_border.abs_as_usize()
            ],
            width_in_pixels_including_border: self.width_in_pixels_including_border,
            height_in_pixels_including_border: self.height_in_pixels_including_border,
        };
        panic!("Implement something!")
        //aggregated_brightnesses
    }
}

impl super::particles_to_pixels::ParticleToPixelMapper for PixelBrightnessAggregator {
    type Output = AggregatedBrightnessMatrix;
    fn aggregate_particle_colors_to_pixels(
        &self,
        particle_map_sequence: impl std::iter::ExactSizeIterator<
            Item = impl data_structure::ParticleIteratorProvider,
        >,
    ) -> Result<PixelMatrixSequence<Self::Output>, Box<dyn std::error::Error>> {
        let mut aggregated_brightnesses: PixelMatrixSequence<AggregatedBrightnessMatrix> =
            PixelMatrixSequence {
                colored_pixel_matrices: vec![],
                maximum_brightness_per_color: super::color::zero_brightness(),
            };

        for mut particle_map in particle_map_sequence {
            aggregated_brightnesses
                .colored_pixel_matrices
                .push(self.aggregate_over_particle_iterator(particle_map.get()));
        }

        Ok(aggregated_brightnesses)
    }

    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &self.width_in_pixels_including_border
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &self.height_in_pixels_including_border
    }
}

#[cfg(test)]
mod tests {
    use super::super::ColoredPixelMatrix;
    use super::*;

    fn new_reference_brightness() -> ColorBrightness {
        super::super::color::brightness_from_values(
            data_structure::RedColorUnit(10.0),
            data_structure::GreenColorUnit(10.0),
            data_structure::BlueColorUnit(10.0),
        )
    }

    fn new_test_fraction(color_brightness: &ColorBrightness) -> Result<ColorFraction, String> {
        let reference_color = new_reference_brightness();
        match super::super::color::fraction_from_triplets(color_brightness, &reference_color) {
            Ok(color_fraction) => Ok(color_fraction),
            Err(unexpected_error) => Err(String::from(format!(
                "Could not produce valid fraction ({:?}/{:?}) for test: {:?}",
                color_brightness,
                reference_color,
                unexpected_error.to_string()
            ))),
        }
    }

    fn new_lower_left_color() -> ColorBrightness {
        super::super::color::brightness_from_values(
            data_structure::RedColorUnit(1.0),
            data_structure::GreenColorUnit(0.0),
            data_structure::BlueColorUnit(0.0),
        )
    }

    fn new_lower_right_color() -> ColorBrightness {
        super::super::color::brightness_from_values(
            data_structure::RedColorUnit(0.0),
            data_structure::GreenColorUnit(1.0),
            data_structure::BlueColorUnit(0.0),
        )
    }

    fn new_upper_left_color() -> ColorBrightness {
        super::super::color::brightness_from_values(
            data_structure::RedColorUnit(0.0),
            data_structure::GreenColorUnit(0.0),
            data_structure::BlueColorUnit(1.0),
        )
    }

    fn new_upper_right_color() -> ColorBrightness {
        super::super::color::brightness_from_values(
            data_structure::RedColorUnit(0.5),
            data_structure::GreenColorUnit(0.5),
            data_structure::BlueColorUnit(0.5),
        )
    }

    fn new_test_matrix() -> AggregatedBrightnessMatrix {
        AggregatedBrightnessMatrix {
            brightness_matrix: vec![
                vec![new_lower_left_color(), new_lower_right_color()],
                vec![new_upper_left_color(), new_upper_right_color()],
            ],
            width_in_pixels_including_border: HorizontalPixelAmount(2),
            height_in_pixels_including_border: VerticalPixelAmount(2),
        }
    }

    fn new_test_particle_intrinsics(
        color_brightness: &ColorBrightness,
    ) -> data_structure::ParticleIntrinsics {
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.2),
            attractive_charge: data_structure::AttractiveChargeUnit(-3.4),
            repulsive_charge: data_structure::RepulsiveChargeUnit(5.6),
            red_brightness: color_brightness.get_red(),
            green_brightness: color_brightness.get_green(),
            blue_brightness: color_brightness.get_blue(),
        }
    }

    #[test]
    fn check_out_of_bounds_produces_error() -> Result<(), String> {
        let test_matrix = new_test_matrix();
        let mut failing_points: std::vec::Vec<(
            HorizontalPixelAmount,
            VerticalPixelAmount,
            ColorFraction,
        )> = std::vec::Vec::new();
        for (horizontal_pixel, vertical_pixel) in &[
            (HorizontalPixelAmount(2), VerticalPixelAmount(0)),
            (HorizontalPixelAmount(2), VerticalPixelAmount(2)),
            (HorizontalPixelAmount(0), VerticalPixelAmount(20)),
            (HorizontalPixelAmount(-1), VerticalPixelAmount(3)),
            (HorizontalPixelAmount(-4), VerticalPixelAmount(1)),
            (HorizontalPixelAmount(-1), VerticalPixelAmount(-2)),
            (HorizontalPixelAmount(1), VerticalPixelAmount(-1)),
            (HorizontalPixelAmount(2), VerticalPixelAmount(-12)),
        ] {
            let function_result = test_matrix.color_fractions_at(
                &new_reference_brightness(),
                horizontal_pixel,
                vertical_pixel,
            );
            if let Ok(unexpected_brightness) = function_result {
                failing_points.push((*horizontal_pixel, *vertical_pixel, unexpected_brightness));
            }
        }

        if failing_points.is_empty() {
            Ok(())
        } else {
            Err(String::from(format!(
                "Following points had color fractions (as (x, y, unexpected result)): {:?}",
                failing_points
            )))
        }
    }

    #[test]
    fn check_internal_pixels_are_correct() -> Result<(), String> {
        let test_matrix = new_test_matrix();
        let mut points_in_error: std::vec::Vec<(
            HorizontalPixelAmount,
            VerticalPixelAmount,
            String,
        )> = std::vec::Vec::new();
        let mut points_with_incorrect_color: std::vec::Vec<(
            HorizontalPixelAmount,
            VerticalPixelAmount,
            ColorFraction,
        )> = std::vec::Vec::new();
        for (horizontal_pixel, vertical_pixel, expected_color_fraction) in &[
            (
                HorizontalPixelAmount(0),
                VerticalPixelAmount(0),
                new_test_fraction(&new_lower_left_color())?,
            ),
            (
                HorizontalPixelAmount(1),
                VerticalPixelAmount(0),
                new_test_fraction(&new_lower_right_color())?,
            ),
            (
                HorizontalPixelAmount(1),
                VerticalPixelAmount(1),
                new_test_fraction(&new_upper_right_color())?,
            ),
            (
                HorizontalPixelAmount(0),
                VerticalPixelAmount(1),
                new_test_fraction(&new_upper_left_color())?,
            ),
        ] {
            let function_result = test_matrix.color_fractions_at(
                &new_reference_brightness(),
                horizontal_pixel,
                vertical_pixel,
            );

            match function_result {
                Ok(resulting_brightness) => {
                    if &resulting_brightness != expected_color_fraction {
                        points_with_incorrect_color.push((
                            *horizontal_pixel,
                            *vertical_pixel,
                            resulting_brightness,
                        ));
                    }
                }
                Err(unexpected_error) => points_in_error.push((
                    *horizontal_pixel,
                    *vertical_pixel,
                    unexpected_error.to_string(),
                )),
            }
        }

        if points_in_error.is_empty() && points_with_incorrect_color.is_empty() {
            Ok(())
        } else {
            Err(String::from(format!(
                "Following points had incorrect color fractions (as (x, y, color)): {:?} \n \
                Following points had unexpected errors (as (x, y, error)): {:?}",
                points_with_incorrect_color, points_in_error
            )))
        }
    }

    #[test]
    fn check_three_particles_in_three_separate_pixels() -> Result<(), String> {
        let expected_colored_pixels = vec![
            (
                HorizontalPixelAmount(0),
                VerticalPixelAmount(0),
                super::super::color::brightness_from_values(
                    data_structure::RedColorUnit(0.75),
                    data_structure::GreenColorUnit(0.25),
                    data_structure::BlueColorUnit(0.0),
                ),
            ),
            (
                HorizontalPixelAmount(1),
                VerticalPixelAmount(1),
                super::super::color::brightness_from_values(
                    data_structure::RedColorUnit(0.0),
                    data_structure::GreenColorUnit(0.25),
                    data_structure::BlueColorUnit(2.0),
                ),
            ),
            (
                HorizontalPixelAmount(9),
                VerticalPixelAmount(-1),
                super::super::color::brightness_from_values(
                    data_structure::RedColorUnit(0.05),
                    data_structure::GreenColorUnit(0.9),
                    data_structure::BlueColorUnit(0.05),
                ),
            ),
        ];
        let test_particles = vec![
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[0].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(0.0),
                    vertical_position: data_structure::VerticalPositionUnit(0.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(-10.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(9.9),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[1].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(0.1),
                    vertical_position: data_structure::VerticalPositionUnit(1.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.001),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.99),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[0].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(9.999),
                    vertical_position: data_structure::VerticalPositionUnit(-0.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
        ];
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_six_particles_in_only_three_pixels() -> Result<(), String> {
        // 3 in 1, 2 in 1, 1 in 1.
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_offscreen_particle_not_drawn_when_appropriate() -> Result<(), String> {
        Err(String::from(
            "Implement something like check_internal_pixels_are_correct",
        ))
    }

    #[test]
    fn check_offscreen_particle_drawn_on_border_when_appropriate() -> Result<(), String> {
        Err(String::from(
            "Implement something like check_internal_pixels_are_correct",
        ))
    }
}
