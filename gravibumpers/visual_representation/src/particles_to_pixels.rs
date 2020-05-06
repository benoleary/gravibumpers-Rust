use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::RedGreenBlueIntensity;
use super::VerticalPixelAmount;

pub struct ColoredPixelMatrixSequence {
    pub colored_pixel_matrices: Vec<Box<dyn ColoredPixelMatrix>>,
    pub maximum_color_intensity: RedGreenBlueIntensity,
}

pub trait ParticleToPixelMapper {
    fn aggregate_particle_colors_to_pixels<T: data_structure::ParticleCollection>(
        &self,
        particle_map_sequence: &mut dyn std::iter::Iterator<Item = &T>,
    ) -> Result<ColoredPixelMatrixSequence, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
