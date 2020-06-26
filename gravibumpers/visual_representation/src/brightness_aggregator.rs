/// This module provides implementations of ColoredPixelMatrix and
/// particles_to_pixels::ParticleToPixelMapper which perform the basic functionality of
/// rounding particle co-ordinates to pixel co-ordinates, aggregating the color brightnesses
/// which land in each pixel.
use super::color::FractionTriplet as ColorFraction;
use super::particles_to_pixels::ColoredPixelMatrixSequence as PixelMatrixSequence;
use super::HorizontalPixelAmount;
use super::OutOfBoundsError;
use super::VerticalPixelAmount;

pub struct AggregatedBrightnessMatrix {
    brightness_matrix: std::vec::Vec<std::vec::Vec<data_structure::ColorTriplet>>,
    width_in_pixels_including_border: HorizontalPixelAmount,
    height_in_pixels_including_border: VerticalPixelAmount,
}

impl AggregatedBrightnessMatrix {
    fn add_brightness_without_bounds_check_returning_current_triplet(
        &mut self,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
        brightness_to_add: &data_structure::ColorTriplet,
    ) -> &data_structure::ColorTriplet {
        let height_index = vertical_pixels_from_bottom_left.0;
        let width_index = horizontal_pixels_from_bottom_left.0;
        let pixel_to_update =
            &mut self.brightness_matrix[height_index as usize][width_index as usize];
        *pixel_to_update += *brightness_to_add;
        pixel_to_update
    }
}

impl super::ColoredPixelMatrix for AggregatedBrightnessMatrix {
    fn color_fractions_at(
        &self,
        reference_brightness: &data_structure::ColorTriplet,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<ColorFraction, Box<dyn std::error::Error>> {
        let height_index = vertical_pixels_from_bottom_left.0;
        let width_index = horizontal_pixels_from_bottom_left.0;
        if (horizontal_pixels_from_bottom_left >= &self.width_in_pixels_including_border)
            || (vertical_pixels_from_bottom_left >= &self.height_in_pixels_including_border)
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

#[derive(Clone, Copy, Debug)]
struct PixelWindow {
    pub left_border: HorizontalPixelAmount,
    pub right_border: HorizontalPixelAmount,
    pub lower_border: VerticalPixelAmount,
    pub upper_border: VerticalPixelAmount,
    pub width_in_pixels_including_border: HorizontalPixelAmount,
    pub height_in_pixels_including_border: VerticalPixelAmount,
}

pub struct PixelBrightnessAggregator {
    pixel_window: PixelWindow,
    add_brightness_from_particle_returning_current_triplet: Box<
        dyn Fn(
            &PixelWindow,
            &mut AggregatedBrightnessMatrix,
            &data_structure::IndividualParticle,
        ) -> Option<data_structure::ColorTriplet>,
    >,
}

impl PixelBrightnessAggregator {
    fn aggregate_over_particle_iterator(
        &self,
        particles_to_draw: impl std::iter::ExactSizeIterator<Item = data_structure::IndividualParticle>,
    ) -> (AggregatedBrightnessMatrix, data_structure::ColorTriplet) {
        let mut aggregated_brightnesses = AggregatedBrightnessMatrix {
            brightness_matrix: vec![
                vec![
                    super::color::zero_brightness();
                    self.pixel_window
                        .width_in_pixels_including_border
                        .abs_as_usize()
                ];
                self.pixel_window
                    .height_in_pixels_including_border
                    .abs_as_usize()
            ],
            width_in_pixels_including_border: self.pixel_window.width_in_pixels_including_border,
            height_in_pixels_including_border: self.pixel_window.height_in_pixels_including_border,
        };

        let mut maximum_brightnesses = super::color::zero_brightness();
        let add_brightness_from = &*self.add_brightness_from_particle_returning_current_triplet;
        for particle_to_draw in particles_to_draw {
            let update_result = add_brightness_from(
                &self.pixel_window,
                &mut aggregated_brightnesses,
                &particle_to_draw,
            );
            if let Some(updated_pixel) = update_result {
                maximum_brightnesses.overwrite_each_color_if_brighter(&updated_pixel);
            }
        }
        (aggregated_brightnesses, maximum_brightnesses)
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
            let (aggregated_brightnesses_in_map, maximum_brightness_in_map) =
                self.aggregate_over_particle_iterator(particle_map.get());
            aggregated_brightnesses
                .colored_pixel_matrices
                .push(aggregated_brightnesses_in_map);
            aggregated_brightnesses
                .maximum_brightness_per_color
                .overwrite_each_color_if_brighter(&maximum_brightness_in_map);
        }

        Ok(aggregated_brightnesses)
    }

    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &self.pixel_window.width_in_pixels_including_border
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &self.pixel_window.height_in_pixels_including_border
    }
}

fn draw_only_onscreen_particles(
    pixel_window: &PixelWindow,
    aggregation_matrix: &mut AggregatedBrightnessMatrix,
    particle_to_draw: &data_structure::IndividualParticle,
) -> Option<data_structure::ColorTriplet> {
    let particle_horizontal_coordinate = particle_to_draw.variable_values.horizontal_position;
    let particle_vertical_coordinate = particle_to_draw.variable_values.vertical_position;
    if (particle_horizontal_coordinate >= pixel_window.left_border.as_position_unit())
        && (particle_horizontal_coordinate <= pixel_window.right_border.as_position_unit())
        && (particle_vertical_coordinate >= pixel_window.lower_border.as_position_unit())
        && (particle_vertical_coordinate <= pixel_window.upper_border.as_position_unit())
    {
        // The f64s have to fit into i32s because each was within a pair of i32 values.
        let horizontal_pixel = super::new_horizontal_pixel_unit_rounding_to_negative_infinity(
            particle_horizontal_coordinate,
        ) - pixel_window.left_border;
        let vertical_pixel = super::new_vertical_pixel_unit_rounding_to_negative_infinity(
            particle_vertical_coordinate,
        ) - pixel_window.lower_border;
        Some(
            *aggregation_matrix.add_brightness_without_bounds_check_returning_current_triplet(
                &horizontal_pixel,
                &vertical_pixel,
                &particle_to_draw.intrinsic_values.color_brightness,
            ),
        )
    } else {
        None
    }
}

fn draw_offscreen_particles_on_border(
    pixel_window: &PixelWindow,
    aggregation_matrix: &mut AggregatedBrightnessMatrix,
    particle_to_draw: &data_structure::IndividualParticle,
) -> Option<data_structure::ColorTriplet> {
    let particle_horizontal_coordinate = particle_to_draw.variable_values.horizontal_position;
    let particle_vertical_coordinate = particle_to_draw.variable_values.vertical_position;
    let horizontal_pixel = if particle_horizontal_coordinate
        < pixel_window.left_border.as_position_unit()
    {
        HorizontalPixelAmount(0)
    } else if particle_horizontal_coordinate > pixel_window.right_border.as_position_unit() {
        pixel_window.right_border - pixel_window.left_border
    } else {
        HorizontalPixelAmount(particle_horizontal_coordinate.0 as i32) - pixel_window.left_border
    };
    let vertical_pixel =
        if particle_vertical_coordinate < pixel_window.lower_border.as_position_unit() {
            VerticalPixelAmount(0)
        } else if particle_vertical_coordinate > pixel_window.upper_border.as_position_unit() {
            pixel_window.upper_border - pixel_window.lower_border
        } else {
            VerticalPixelAmount(particle_vertical_coordinate.0 as i32) - pixel_window.lower_border
        };

    Some(
        *aggregation_matrix.add_brightness_without_bounds_check_returning_current_triplet(
            &horizontal_pixel,
            &vertical_pixel,
            &particle_to_draw.intrinsic_values.color_brightness,
        ),
    )
}

pub fn new(
    left_border: HorizontalPixelAmount,
    right_border: HorizontalPixelAmount,
    lower_border: VerticalPixelAmount,
    upper_border: VerticalPixelAmount,
    draw_offscreen_on_border: bool,
) -> Result<PixelBrightnessAggregator, Box<dyn std::error::Error>> {
    if (right_border < left_border) || (upper_border < lower_border) {
        return Err(Box::new(OutOfBoundsError::new(&format!(
            "right border {:?} must not be less than left border {:?} \
             and upper border {:?} must not be less than lower border {:?}",
            right_border, left_border, upper_border, lower_border
        ))));
    }
    let add_particle_brightness: Box<
        dyn Fn(
            &PixelWindow,
            &mut AggregatedBrightnessMatrix,
            &data_structure::IndividualParticle,
        ) -> Option<data_structure::ColorTriplet>,
    > = if draw_offscreen_on_border {
        Box::new(draw_offscreen_particles_on_border)
    } else {
        Box::new(draw_only_onscreen_particles)
    };

    // The borders are included in the width, so if the left border is at -10 and the right at +20,
    // the width is 31. The height is the difference plus one for the analogous reason.
    let pixel_window = PixelWindow {
        left_border: left_border,
        right_border: right_border,
        lower_border: lower_border,
        upper_border: upper_border,
        width_in_pixels_including_border: right_border - left_border + HorizontalPixelAmount(1),
        height_in_pixels_including_border: upper_border - lower_border + VerticalPixelAmount(1),
    };
    Ok(PixelBrightnessAggregator {
        pixel_window: pixel_window,
        add_brightness_from_particle_returning_current_triplet: add_particle_brightness,
    })
}

#[cfg(test)]
mod tests {
    use super::super::ColoredPixelMatrix;
    use super::*;

    const COLOR_FRACTION_TOLERANCE: f64 = 0.000001;

    fn new_test_fraction(
        color_brightness: &data_structure::ColorTriplet,
    ) -> Result<ColorFraction, String> {
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

    fn new_lower_left_color() -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
            data_structure::RedColorUnit(1.0),
            data_structure::GreenColorUnit(0.0),
            data_structure::BlueColorUnit(0.0),
        )
    }

    fn new_lower_right_color() -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
            data_structure::RedColorUnit(0.0),
            data_structure::GreenColorUnit(1.0),
            data_structure::BlueColorUnit(0.0),
        )
    }

    fn new_upper_left_color() -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
            data_structure::RedColorUnit(0.0),
            data_structure::GreenColorUnit(0.0),
            data_structure::BlueColorUnit(1.0),
        )
    }

    fn new_upper_right_color() -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
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

    fn new_reference_brightness() -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
            data_structure::RedColorUnit(1.0),
            data_structure::GreenColorUnit(1.0),
            data_structure::BlueColorUnit(1.0),
        )
    }

    fn new_test_particle_intrinsics(
        color_fraction: &ColorFraction,
    ) -> data_structure::ParticleIntrinsics {
        data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.2),
            attractive_charge: data_structure::AttractiveChargeUnit(-3.4),
            repulsive_charge: data_structure::RepulsiveChargeUnit(5.6),
            color_brightness: *color_fraction * &new_reference_brightness(),
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

    fn compare_pixel_with_expected(
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
        expected_brights: &std::vec::Vec<(
            HorizontalPixelAmount,
            VerticalPixelAmount,
            ColorFraction,
        )>,
        actual_pixel: &ColorFraction,
    ) -> Option<String> {
        let mut expected_pixel = super::super::color::zero_fraction();
        for expected_bright in expected_brights {
            if (expected_bright.0 == *horizontal_pixels_from_bottom_left)
                && (expected_bright.1 == *vertical_pixels_from_bottom_left)
            {
                expected_pixel = expected_bright.2;
                break;
            }
        }

        if !super::super::color::fraction_triplets_match(
            actual_pixel,
            &expected_pixel,
            COLOR_FRACTION_TOLERANCE,
        ) {
            Some(String::from(format!(
                "({:?},{:?}): expected {:?}, actual {:?}",
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
                expected_pixel,
                actual_pixel
            )))
        } else {
            None
        }
    }

    // It isn't really necessary to extract this and use a closure, but I wanted to try it out.
    fn loop_over_all_pixels(
        height_in_pixels: &VerticalPixelAmount,
        width_in_pixels: &HorizontalPixelAmount,
        function_per_pixel: &mut impl FnMut(HorizontalPixelAmount, VerticalPixelAmount) -> (),
    ) -> () {
        for vertical_pixel in 0..height_in_pixels.0 {
            let vertical_pixels_from_bottom_left = VerticalPixelAmount(vertical_pixel);
            for horizontal_pixel in 0..width_in_pixels.0 {
                let horizontal_pixels_from_bottom_left = HorizontalPixelAmount(horizontal_pixel);
                function_per_pixel(
                    horizontal_pixels_from_bottom_left,
                    vertical_pixels_from_bottom_left,
                );
            }
        }
    }

    fn assert_pixels_as_expected_with_implicit_black_background(
        resulting_matrix: &AggregatedBrightnessMatrix,
        resulting_maximum_brightnesses: &data_structure::ColorTriplet,
        expected_pixels: &std::vec::Vec<(
            HorizontalPixelAmount,
            VerticalPixelAmount,
            ColorFraction,
        )>,
        expected_maximum_brightnesses: &data_structure::ColorTriplet,
    ) -> Result<(), String> {
        let reference_brightness = new_reference_brightness();
        let mut failure_messages: std::vec::Vec<String> = vec![];
        if !data_structure::color_triplets_match(
            expected_maximum_brightnesses,
            resulting_maximum_brightnesses,
            COLOR_FRACTION_TOLERANCE,
        ) {
            failure_messages.push(String::from(format!(
                "Incorrect maximum brightnesses: expected {:?}, actual {:?}",
                expected_maximum_brightnesses, resulting_maximum_brightnesses,
            )));
        }
        loop_over_all_pixels(
            resulting_matrix.height_in_pixels(),
            resulting_matrix.width_in_pixels(),
            &mut |horizontal_pixels_from_bottom_left, vertical_pixels_from_bottom_left| {
                let actual_result = resulting_matrix.color_fractions_at(
                    &reference_brightness,
                    &horizontal_pixels_from_bottom_left,
                    &vertical_pixels_from_bottom_left,
                );

                if let Ok(actual_pixel) = actual_result {
                    let pixel_comparison = compare_pixel_with_expected(
                        &horizontal_pixels_from_bottom_left,
                        &vertical_pixels_from_bottom_left,
                        expected_pixels,
                        &actual_pixel,
                    );
                    if let Some(failure_message) = pixel_comparison {
                        failure_messages.push(failure_message);
                    }
                } else {
                    failure_messages.push(String::from(format!(
                        "({:?},{:?}) produced error {:?}",
                        horizontal_pixels_from_bottom_left,
                        vertical_pixels_from_bottom_left,
                        actual_result
                    )));
                }
            },
        );

        if failure_messages.is_empty() {
            Ok(())
        } else {
            Err(failure_messages.join("\n"))
        }
    }

    #[test]
    fn check_three_particles_in_three_separate_pixels() -> Result<(), String> {
        // We have a view on co-ordinates with 10 <= x <= 30, -10 <= y <= 10.
        let pixel_brightness_aggregator = new(
            HorizontalPixelAmount(10),
            HorizontalPixelAmount(30),
            VerticalPixelAmount(-10),
            VerticalPixelAmount(10),
            false,
        )
        .expect("Test should not get borders mixed up");
        // Since the view is 10 <= x <= 30, -10 <= y <= 10, the expected horizontal
        // co-ordinate is 10 less than the particle x co-ordinate, and the expected vertical
        // co-ordinate is 10 higher than the particle y co-ordinate.
        let expected_colored_pixels = vec![
            (
                HorizontalPixelAmount(0),
                VerticalPixelAmount(10),
                super::super::color::fraction_from_values(0.75, 0.25, 0.0),
            ),
            (
                HorizontalPixelAmount(1),
                VerticalPixelAmount(11),
                super::super::color::fraction_from_values(0.0, 0.25, 2.0),
            ),
            (
                HorizontalPixelAmount(9),
                VerticalPixelAmount(9),
                super::super::color::fraction_from_values(0.05, 0.9, 0.05),
            ),
        ];
        let test_particles = vec![
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[0].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(10.0),
                    vertical_position: data_structure::VerticalPositionUnit(0.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(-10.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(9.9),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[1].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(11.1),
                    vertical_position: data_structure::VerticalPositionUnit(1.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.001),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.99),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[2].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(19.999),
                    vertical_position: data_structure::VerticalPositionUnit(-0.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
        ];
        let (resulting_matrix, resulting_maximum_brightnesses) = pixel_brightness_aggregator
            .aggregate_over_particle_iterator(test_particles.into_iter());
        assert_pixels_as_expected_with_implicit_black_background(
            &resulting_matrix,
            &resulting_maximum_brightnesses,
            &expected_colored_pixels,
            &data_structure::new_color_triplet(
                data_structure::RedColorUnit(0.75),
                data_structure::GreenColorUnit(0.9),
                data_structure::BlueColorUnit(2.0),
            ),
        )
    }

    fn new_test_ten_by_ten_aggregator(draw_offscreen_on_border: bool) -> PixelBrightnessAggregator {
        new(
            HorizontalPixelAmount(0),
            HorizontalPixelAmount(10),
            VerticalPixelAmount(0),
            VerticalPixelAmount(10),
            draw_offscreen_on_border,
        )
        .expect("Test should not get borders mixed up")
    }

    #[test]
    fn check_six_particles_in_only_three_pixels() -> Result<(), String> {
        let pixel_brightness_aggregator = new_test_ten_by_ten_aggregator(false);
        let expected_colored_pixels = vec![
            (
                HorizontalPixelAmount(3),
                VerticalPixelAmount(3),
                super::super::color::fraction_from_values(0.3, 0.2, 0.1),
            ),
            (
                HorizontalPixelAmount(5),
                VerticalPixelAmount(9),
                super::super::color::fraction_from_values(0.0, 2.0, 2.0),
            ),
            (
                HorizontalPixelAmount(8),
                VerticalPixelAmount(0),
                super::super::color::fraction_from_values(0.0, 0.0, 1.0),
            ),
        ];
        let test_particles = vec![
            // First of 3 in pixel (3, 3).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.1, 0.0, 0.1),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(3.0),
                    vertical_position: data_structure::VerticalPositionUnit(3.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(10.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(10.0),
                },
            },
            // Second of 3 in pixel (3, 3).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.1, 0.1, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(3.0),
                    vertical_position: data_structure::VerticalPositionUnit(3.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(-1.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(1.0),
                },
            },
            // Third of 3 in pixel (3, 3).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.1, 0.1, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(3.5),
                    vertical_position: data_structure::VerticalPositionUnit(3.8),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            // First of 2 in pixel (5, 9).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 2.0, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(5.9),
                    vertical_position: data_structure::VerticalPositionUnit(9.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            // Second of 2 in pixel (5, 9).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 0.0, 2.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(5.0),
                    vertical_position: data_structure::VerticalPositionUnit(9.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            // Only particle in pixel (8, 0).
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(&expected_colored_pixels[2].2),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(8.999),
                    vertical_position: data_structure::VerticalPositionUnit(0.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
        ];
        let (resulting_matrix, resulting_maximum_brightnesses) = pixel_brightness_aggregator
            .aggregate_over_particle_iterator(test_particles.into_iter());
        assert_pixels_as_expected_with_implicit_black_background(
            &resulting_matrix,
            &resulting_maximum_brightnesses,
            &expected_colored_pixels,
            &data_structure::new_color_triplet(
                data_structure::RedColorUnit(0.3),
                data_structure::GreenColorUnit(2.0),
                data_structure::BlueColorUnit(2.0),
            ),
        )
    }

    fn new_test_particles_outside_frame() -> std::vec::Vec<data_structure::IndividualParticle> {
        vec![
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 0.0, 1.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(11.0),
                    vertical_position: data_structure::VerticalPositionUnit(3.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(-10.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 1.0, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(30.0),
                    vertical_position: data_structure::VerticalPositionUnit(30.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(-1.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(1.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 1.0, 1.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(3.5),
                    vertical_position: data_structure::VerticalPositionUnit(13.8),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(1.0, 0.0, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(-0.001),
                    vertical_position: data_structure::VerticalPositionUnit(10.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(1.0, 0.0, 1.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(-500.0),
                    vertical_position: data_structure::VerticalPositionUnit(1.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(1.0, 1.0, 0.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(-1.0),
                    vertical_position: data_structure::VerticalPositionUnit(-1.0),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(1.0, 1.0, 1.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(8.999),
                    vertical_position: data_structure::VerticalPositionUnit(-0.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
            data_structure::IndividualParticle {
                intrinsic_values: new_test_particle_intrinsics(
                    &super::super::color::fraction_from_values(0.0, 0.0, 2.0),
                ),
                variable_values: data_structure::ParticleVariables {
                    horizontal_position: data_structure::HorizontalPositionUnit(88.999),
                    vertical_position: data_structure::VerticalPositionUnit(-100.001),
                    horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                    vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
                },
            },
        ]
    }

    #[test]
    fn check_offscreen_particle_not_drawn_when_appropriate() -> Result<(), String> {
        let pixel_brightness_aggregator = new_test_ten_by_ten_aggregator(false);
        let expected_colored_pixels = vec![];
        let test_particles = new_test_particles_outside_frame();
        let (resulting_matrix, resulting_maximum_brightnesses) = pixel_brightness_aggregator
            .aggregate_over_particle_iterator(test_particles.into_iter());
        assert_pixels_as_expected_with_implicit_black_background(
            &resulting_matrix,
            &resulting_maximum_brightnesses,
            &expected_colored_pixels,
            &data_structure::new_color_triplet(
                data_structure::RedColorUnit(0.0),
                data_structure::GreenColorUnit(0.0),
                data_structure::BlueColorUnit(0.0),
            ),
        )
    }

    fn color_fraction_from_particle_intrinsics(
        particle_intrinsics: data_structure::ParticleIntrinsics,
    ) -> ColorFraction {
        super::super::color::fraction_from_triplets(
            &particle_intrinsics.color_brightness,
            &new_reference_brightness(),
        )
        .expect("How did the test constant end up as zero?")
    }

    #[test]
    fn check_offscreen_particle_drawn_on_border_when_appropriate() -> Result<(), String> {
        let pixel_brightness_aggregator = new_test_ten_by_ten_aggregator(true);
        let mut test_particles = new_test_particles_outside_frame();
        // We add 2 more particles to check for aggregation of brightnesses on the lower edge and
        // lower-right corner.
        test_particles.push(data_structure::IndividualParticle {
            intrinsic_values: new_test_particle_intrinsics(
                &super::super::color::fraction_from_values(1.0, 1.0, 3.0),
            ),
            variable_values: data_structure::ParticleVariables {
                horizontal_position: data_structure::HorizontalPositionUnit(8.1),
                vertical_position: data_structure::VerticalPositionUnit(-2.2),
                horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
            },
        });
        test_particles.push(data_structure::IndividualParticle {
            intrinsic_values: new_test_particle_intrinsics(
                &super::super::color::fraction_from_values(0.0, 3.0, 3.0),
            ),
            variable_values: data_structure::ParticleVariables {
                horizontal_position: data_structure::HorizontalPositionUnit(14.0),
                vertical_position: data_structure::VerticalPositionUnit(-100.001),
                horizontal_velocity: data_structure::HorizontalVelocityUnit(0.0),
                vertical_velocity: data_structure::VerticalVelocityUnit(0.0),
            },
        });

        let left_edge = HorizontalPixelAmount(0);
        let right_edge = HorizontalPixelAmount(10);
        let lower_edge = VerticalPixelAmount(0);
        let upper_edge = VerticalPixelAmount(10);
        let expected_colored_pixels = vec![
            (
                right_edge,
                VerticalPixelAmount(3),
                color_fraction_from_particle_intrinsics(test_particles[0].intrinsic_values),
            ),
            (
                right_edge,
                upper_edge,
                color_fraction_from_particle_intrinsics(test_particles[1].intrinsic_values),
            ),
            (
                HorizontalPixelAmount(3),
                upper_edge,
                color_fraction_from_particle_intrinsics(test_particles[2].intrinsic_values),
            ),
            (
                left_edge,
                upper_edge,
                color_fraction_from_particle_intrinsics(test_particles[3].intrinsic_values),
            ),
            (
                left_edge,
                VerticalPixelAmount(1),
                color_fraction_from_particle_intrinsics(test_particles[4].intrinsic_values),
            ),
            (
                left_edge,
                lower_edge,
                color_fraction_from_particle_intrinsics(test_particles[5].intrinsic_values),
            ),
            (
                HorizontalPixelAmount(8),
                lower_edge,
                super::super::color::fraction_from_values(2.0, 2.0, 4.0),
            ),
            (
                right_edge,
                lower_edge,
                super::super::color::fraction_from_values(0.0, 3.0, 5.0),
            ),
        ];
        let (resulting_matrix, resulting_maximum_brightnesses) = pixel_brightness_aggregator
            .aggregate_over_particle_iterator(test_particles.into_iter());
        assert_pixels_as_expected_with_implicit_black_background(
            &resulting_matrix,
            &resulting_maximum_brightnesses,
            &expected_colored_pixels,
            &data_structure::new_color_triplet(
                data_structure::RedColorUnit(2.0),
                data_structure::GreenColorUnit(3.0),
                data_structure::BlueColorUnit(5.0),
            ),
        )
    }
}
