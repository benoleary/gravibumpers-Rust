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
    fn aggregate_over_particle_iterator(&self) -> AggregatedBrightnessMatrix {
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
    }

    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &self.width_in_pixels_including_border
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &self.height_in_pixels_including_border
    }
}
