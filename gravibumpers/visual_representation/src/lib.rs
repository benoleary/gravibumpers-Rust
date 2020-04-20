extern crate data_structure;
pub mod apng;

pub trait SequenceAnimator {
    fn animate_sequence(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
        milliseconds_per_frame: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

struct ColorDensityPixel {
    pub red_density: f64,
    pub green_density: f64,
    pub blue_density: f64,
}

trait ColorDensityMap {
    fn color_density_for_pixel_at(
        &self,
        horizontal_coordinate: i32,
        vertical_coordinate: i32,
    ) -> ColorDensityPixel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animate_returns_ok() -> Result<(), String> {
        let test_animator = apng::new();
        test_animator.animate_sequence(vec![], 250)
    }
}
