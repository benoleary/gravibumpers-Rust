use super::color::BrightnessTriplet;
use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::VerticalPixelAmount;

pub struct ColoredPixelMatrixSequence {
    pub colored_pixel_matrices: Vec<Box<dyn ColoredPixelMatrix>>,
    pub maximum_brightness_per_color: BrightnessTriplet,
}

pub trait ParticleToPixelMapper {
    fn aggregate_particle_colors_to_pixels<
        T: data_structure::ParticleIteratorProvider,
        U: std::iter::ExactSizeIterator<Item = T>,
    >(
        &self,
        particle_map_sequence: U,
    ) -> Result<ColoredPixelMatrixSequence, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
