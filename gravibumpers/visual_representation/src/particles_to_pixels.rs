use super::color::BrightnessTriplet;
use super::ColoredPixelMatrix;
use super::HorizontalPixelAmount;
use super::VerticalPixelAmount;

pub struct ColoredPixelMatrixSequence<T: ColoredPixelMatrix> {
    pub colored_pixel_matrices: Vec<T>,
    pub maximum_brightness_per_color: BrightnessTriplet,
}

pub trait ParticleToPixelMapper {
    type Output: ColoredPixelMatrix;
    fn aggregate_particle_colors_to_pixels<
        U: data_structure::ParticleIteratorProvider,
        V: std::iter::ExactSizeIterator<Item = U>,
    >(
        &self,
        particle_map_sequence: V,
    ) -> Result<ColoredPixelMatrixSequence<Self::Output>, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> &HorizontalPixelAmount;
    fn height_in_pixels(&self) -> &VerticalPixelAmount;
}
