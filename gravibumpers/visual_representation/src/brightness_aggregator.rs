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
        /*
        Err(Box::new(OutOfBoundsError::new(&format!(
            "horizontal_offset_of_origin_from_picture_bottom_left {:?}, \
             vertical_offset_of_origin_from_picture_bottom_left {:?}, \
             width_in_pixels_including_border {:?}, \
             height_in_pixels_including_border {:?}",
            self.horizontal_offset_of_origin_from_picture_bottom_left,
            self.vertical_offset_of_origin_from_picture_bottom_left,
            self.width_in_pixels_including_border,
            self.height_in_pixels_including_border
        ))))
        */
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
    use super::*;

    fn new_test_matrix() -> AggregatedBrightnessMatrix {
        AggregatedBrightnessMatrix {
            brightness_matrix: vec![
                vec![
                    super::super::color::brightness_from_values(
                        data_structure::RedColorUnit(1.0),
                        data_structure::GreenColorUnit(0.0),
                        data_structure::BlueColorUnit(0.0),
                    ),
                    super::super::color::brightness_from_values(
                        data_structure::RedColorUnit(0.0),
                        data_structure::GreenColorUnit(1.0),
                        data_structure::BlueColorUnit(0.0),
                    ),
                ],
                vec![
                    super::super::color::brightness_from_values(
                        data_structure::RedColorUnit(0.0),
                        data_structure::GreenColorUnit(0.0),
                        data_structure::BlueColorUnit(1.0),
                    ),
                    super::super::color::brightness_from_values(
                        data_structure::RedColorUnit(0.5),
                        data_structure::GreenColorUnit(0.5),
                        data_structure::BlueColorUnit(0.5),
                    ),
                ],
            ],
            width_in_pixels_including_border: HorizontalPixelAmount(2),
            height_in_pixels_including_border: VerticalPixelAmount(2),
        }
    }
    #[test]
    fn check_out_of_bounds_produces_error() -> Result<(), String> {
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_internal_pixels_are_correct() -> Result<(), String> {
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_three_particles_in_three_separate_pixels() -> Result<(), String> {
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_six_particles_in_only_three_pixels() -> Result<(), String> {
        // 3 in 1, 2 in 1, 1 in 1.
        Err(String::from("Implement something"))
    }

    #[test]
    fn check_offscreen_particle_not_drawn_when_appropriate() -> Result<(), String> {
        Err(String::from("use macro rules for E/NE/N/NW/W/SW/S/SE!"))
    }

    #[test]
    fn check_offscreen_particle_drawn_on_border_when_appropriate() -> Result<(), String> {
        Err(String::from("use macro rules for E/NE/N/NW/W/SW/S/SE!"))
    }
}
