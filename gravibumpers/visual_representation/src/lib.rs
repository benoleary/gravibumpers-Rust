/// There is has no #[cfg(test)] in the main part of the library because it just introduces traits
/// and structs.
extern crate data_structure;
pub mod apng;
pub mod particles_to_pixels;

pub trait SequenceAnimator {
    fn animate_sequence(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
        milliseconds_per_frame: u32,
        output_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct RedGreenBlueIntensity {
    pub red_density: data_structure::ColorUnit,
    pub green_density: data_structure::ColorUnit,
    pub blue_density: data_structure::ColorUnit,
}

pub struct ColorFraction(f64);

pub struct RedGreenBlueFraction {
    pub red_fraction: ColorFraction,
    pub green_fraction: ColorFraction,
    pub blue_fraction: ColorFraction,
}

/// The pixel co-ordinates are taken as from the bottom-left of the picture because that is how
/// I find it easiest to visualize.
pub struct HorizontalPixelAmount(u32);
pub struct VerticalPixelAmount(u32);

pub trait ColoredPixelMatrix {
    fn color_fractions_at(
        &self,
        reference_intensity: &RedGreenBlueIntensity,
        horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
        vertical_pixels_from_bottom_left: &VerticalPixelAmount,
    ) -> Result<RedGreenBlueFraction, Box<dyn std::error::Error>>;

    fn width_in_pixels(&self) -> HorizontalPixelAmount;
    fn height_in_pixels(&self) -> VerticalPixelAmount;
}
