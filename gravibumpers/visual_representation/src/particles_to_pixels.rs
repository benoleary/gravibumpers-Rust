use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::RedGreenBlueIntensity;
use super::VerticalPixelAmount;

pub struct ColoredPixelMatrixSequence {
    pub colored_pixel_matrices: Vec<Box<dyn ColoredPixelMatrix>>,
    pub maximum_color_intensity: RedGreenBlueIntensity,
}

pub trait ParticleToPixelMapper {
    fn aggregate_particle_colors_to_pixels(
        &self,
        particle_map_sequence: data_structure::ParticleSetIterator,
    ) -> Result<ColoredPixelMatrixSequence, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
