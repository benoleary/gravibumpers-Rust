use super::particles_to_pixels::ParticleToPixelMapper;
use super::ColorFraction;
use super::HorizontalPixelAmount;
use super::OutOfBoundsError;
use super::RedGreenBlueFraction;
use super::RedGreenBlueIntensity;
use super::VerticalPixelAmount;

const FRAME_HEIGHT: VerticalPixelAmount = VerticalPixelAmount(50);
const FRAME_WIDTH: HorizontalPixelAmount = HorizontalPixelAmount(100);
const HORIZONTAL_PERIOD: i32 = 2 * FRAME_WIDTH.0;
const RED_HALF_WIDTH: HorizontalPixelAmount = HorizontalPixelAmount(5);
const COLOR_NORMALIZATION: f64 = 0.2;

fn new_pixel_matrix(time_index: i32) -> DemonstrationPixelMatrix {
    let time_through_period = time_index % HORIZONTAL_PERIOD;
    let time_until_period = HORIZONTAL_PERIOD - time_through_period;
    let relevant_time = if time_through_period >= FRAME_WIDTH.0 {
        time_until_period
    } else {
        time_through_period
    };
    let red_peak_line = HorizontalPixelAmount(relevant_time);
    let green_left_edge = FRAME_WIDTH - HorizontalPixelAmount((relevant_time / 2) + 10);
    let blue_lower_left_edge = FRAME_HEIGHT - VerticalPixelAmount(relevant_time / 2);
    let blue_lower_right_edge = FRAME_HEIGHT - VerticalPixelAmount(relevant_time / 4);
    let nonred_fraction = ColorFraction((relevant_time as f64) / (FRAME_WIDTH.0 as f64));

    // Since color_fractions_at will already reject any co-ordinate outside the frame,
    // we allow the edges to be techinally outside of the frame.
    DemonstrationPixelMatrix {
        red_upper_edge: VerticalPixelAmount(40),
        red_lower_edge: VerticalPixelAmount(10),
        red_left_edge: (red_peak_line - RED_HALF_WIDTH),
        red_peak_line: red_peak_line,
        red_right_edge: (red_peak_line + RED_HALF_WIDTH),
        green_left_edge: green_left_edge,
        green_fraction: nonred_fraction,
        blue_divider_line: HorizontalPixelAmount(FRAME_WIDTH.0 / 2),
        blue_lower_left_edge: blue_lower_left_edge,
        blue_lower_right_edge: blue_lower_right_edge,
        blue_left_fraction: ColorFraction(1.0),
        blue_right_fraction: nonred_fraction,
    }
}

pub struct DemonstrationPixelMatrix {
    red_upper_edge: VerticalPixelAmount,
    red_lower_edge: VerticalPixelAmount,
    red_left_edge: HorizontalPixelAmount,
    red_peak_line: HorizontalPixelAmount,
    red_right_edge: HorizontalPixelAmount,
    green_left_edge: HorizontalPixelAmount,
    green_fraction: ColorFraction,
    blue_divider_line: HorizontalPixelAmount,
    blue_lower_left_edge: VerticalPixelAmount,
    blue_lower_right_edge: VerticalPixelAmount,
    blue_left_fraction: ColorFraction,
    blue_right_fraction: ColorFraction,
}

impl DemonstrationPixelMatrix {
    fn red_at(
        &self,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> ColorFraction {
        if (vertical_pixels_from_bottom_left < &self.red_lower_edge)
            || (vertical_pixels_from_bottom_left >= &self.red_upper_edge)
            || (horizontal_pixels_from_bottom_left < &self.red_left_edge)
            || (horizontal_pixels_from_bottom_left >= &self.red_right_edge)
        {
            ColorFraction(0.0)
        } else {
            if horizontal_pixels_from_bottom_left < &self.red_peak_line {
                ColorFraction(
                    ((*horizontal_pixels_from_bottom_left - self.red_left_edge).0 as f64)
                        * COLOR_NORMALIZATION,
                )
            } else {
                ColorFraction(
                    ((self.red_right_edge - *horizontal_pixels_from_bottom_left).0 as f64)
                        * COLOR_NORMALIZATION,
                )
            }
        }
    }

    fn green_at(
        &self,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
    ) -> ColorFraction {
        if horizontal_pixels_from_bottom_left < &self.green_left_edge {
            ColorFraction(0.0)
        } else {
            self.green_fraction
        }
    }

    fn blue_at(
        &self,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> ColorFraction {
        let (lower_edge, color_fraction) =
            if horizontal_pixels_from_bottom_left < &self.blue_divider_line {
                (&self.blue_lower_left_edge, self.blue_left_fraction)
            } else {
                (&self.blue_lower_right_edge, self.blue_right_fraction)
            };
        if vertical_pixels_from_bottom_left < lower_edge {
            ColorFraction(0.0)
        } else {
            color_fraction
        }
    }
}

impl super::ColoredPixelMatrix for DemonstrationPixelMatrix {
    fn color_fractions_at(
        &self,
        _reference_intensity: &RedGreenBlueIntensity,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<RedGreenBlueFraction, Box<dyn std::error::Error>> {
        if (horizontal_pixels_from_bottom_left >= self.width_in_pixels())
            || (vertical_pixels_from_bottom_left >= self.height_in_pixels())
        {
            return Err(Box::new(OutOfBoundsError::new(&format!(
                "horizontal_pixels_from_bottom_left {}, vertical_pixels_from_bottom_left {}",
                horizontal_pixels_from_bottom_left.0, vertical_pixels_from_bottom_left.0
            ))));
        }

        Ok(RedGreenBlueFraction {
            red_fraction: self.red_at(
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
            ),
            green_fraction: self.green_at(horizontal_pixels_from_bottom_left),
            blue_fraction: self.blue_at(
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
            ),
        })
    }
    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &FRAME_WIDTH
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &FRAME_HEIGHT
    }
}

type PixelMatrixBox = Box<dyn super::ColoredPixelMatrix>;
type PixelMatrixSequence = super::particles_to_pixels::ColoredPixelMatrixSequence;

#[derive(Copy, Clone, Debug)]
pub struct DummyParticleCollection {}

impl data_structure::ParticleCollection for DummyParticleCollection {}

pub struct DemonstrationMapper {}

impl ParticleToPixelMapper for DemonstrationMapper {
    fn aggregate_particle_colors_to_pixels<T: data_structure::ParticleCollection>(
        &self,
        particle_map_sequence: &mut dyn std::iter::ExactSizeIterator<Item = &T>,
    ) -> Result<PixelMatrixSequence, Box<dyn std::error::Error>> {
        let mut matrix_sequence: Vec<PixelMatrixBox> = Vec::new();
        for (time_index, _) in particle_map_sequence.enumerate() {
            matrix_sequence.push(Box::new(new_pixel_matrix(4 * time_index as i32)))
        }

        Ok(PixelMatrixSequence {
            colored_pixel_matrices: matrix_sequence,
            maximum_color_intensity: RedGreenBlueIntensity {
                red_density: data_structure::ColorUnit(1.0),
                green_density: data_structure::ColorUnit(1.0),
                blue_density: data_structure::ColorUnit(1.0),
            },
        })
    }

    fn width_in_pixels(&self) -> &HorizontalPixelAmount {
        &FRAME_WIDTH
    }
    fn height_in_pixels(&self) -> &VerticalPixelAmount {
        &FRAME_HEIGHT
    }
}
