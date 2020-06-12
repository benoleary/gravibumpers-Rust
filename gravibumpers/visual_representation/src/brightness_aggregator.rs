use super::color::BrightnessTriplet as ColorBrightness;
use super::color::FractionTriplet as ColorFraction;
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
                "horizontal_pixels_from_bottom_left {}, vertical_pixels_from_bottom_left {} \
                - width {}, height {}",
                horizontal_pixels_from_bottom_left.0,
                vertical_pixels_from_bottom_left.0,
                self.width_in_pixels_including_border.0,
                self.height_in_pixels_including_border.0
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
