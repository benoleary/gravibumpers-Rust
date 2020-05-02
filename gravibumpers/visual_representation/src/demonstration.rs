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
    let time_within_period = time_index % HORIZONTAL_PERIOD;
    let red_peak_line = if time_within_period >= FRAME_WIDTH.0 {
        HorizontalPixelAmount(HORIZONTAL_PERIOD - time_within_period - 1)
    } else {
        HorizontalPixelAmount(time_within_period)
    };

    // Since color_fractions_at will already reject any co-ordinate outside the frame,
    // we allow the edges to be techinally outside of the frame.
    DemonstrationPixelMatrix {
        red_upper_edge: VerticalPixelAmount(30),
        red_lower_edge: VerticalPixelAmount(0),
        red_left_edge: (red_peak_line - RED_HALF_WIDTH),
        red_peak_line: red_peak_line,
        red_right_edge: (red_peak_line + RED_HALF_WIDTH),
    }
}

pub struct DemonstrationPixelMatrix {
    red_upper_edge: VerticalPixelAmount,
    red_lower_edge: VerticalPixelAmount,
    red_left_edge: HorizontalPixelAmount,
    red_peak_line: HorizontalPixelAmount,
    red_right_edge: HorizontalPixelAmount,
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
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> ColorFraction {
        ColorFraction(0.0)
    }

    fn blue_at(
        &self,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> ColorFraction {
        ColorFraction(0.0)
    }
}

impl super::ColoredPixelMatrix for DemonstrationPixelMatrix {
    fn color_fractions_at(
        &self,
        reference_intensity: &RedGreenBlueIntensity,
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
            green_fraction: self.green_at(
                horizontal_pixels_from_bottom_left,
                vertical_pixels_from_bottom_left,
            ),
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

pub struct DemonstrationMapper {}

type PixelMatrixBox = Box<dyn super::ColoredPixelMatrix>;
type PixelMatrixSequence = super::particles_to_pixels::ColoredPixelMatrixSequence;

#[derive(Copy, Clone, Debug)]
pub struct DummyParticleCollection {}

impl data_structure::ParticleCollection for DummyParticleCollection {}

impl ParticleToPixelMapper for DemonstrationMapper {
    fn aggregate_particle_colors_to_pixels(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
    ) -> Result<PixelMatrixSequence, Box<dyn std::error::Error>> {
        let mut matrix_sequence: Vec<PixelMatrixBox> =
            Vec::with_capacity(particle_map_sequence.len());
        for (time_index, _) in particle_map_sequence.iter().enumerate() {
            matrix_sequence.push(Box::new(new_pixel_matrix(10 * time_index as i32)))
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
