extern crate apng_encoder;

pub struct ColorDensityPixel {
    pub red_density: f64,
    pub green_density: f64,
    pub blue_density: f64,
}

pub trait ColorDensityMap {
    fn color_density_for_pixel_at(
        &self,
        horizontal_coordinate: i32,
        vertical_coordinate: i32,
    ) -> ColorDensityPixel;
}

pub fn animate_sequence(
    color_density_map_sequence: &[Box<dyn ColorDensityMap>],
    milliseconds_per_frame: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn hold_place(input_int: i32) -> i32 {
    println!(
        "visual_representation::hold_place(input_int = {input_int})",
        input_int = input_int
    );

    // This is copy-pasted from the example displayed on the crates.io page for the apng-encoder
    // crate, with some re-naming of variables.
    // It is serving as a basis for making GIFs of the mass maps.
    // Generate 2x2 Animated PNG (4 frames)
    let test_meta = apng_encoder::Meta {
        width: 2,
        height: 2,
        color: apng_encoder::Color::RGB(8),
        frames: 4,
        plays: None, // Infinite loop
    };

    // Delay = 1/2 (0.5) seconds
    let test_frame = apng_encoder::Frame {
        delay: Some(apng_encoder::Delay::new(1, 2)),
        ..Default::default()
    };

    let mut test_file = std::fs::File::create("test_2x2.png").unwrap();

    let mut test_encoder = apng_encoder::Encoder::create(&mut test_file, test_meta).unwrap();

    // RED   GREEN
    // BLACK BLUE
    test_encoder
        .write_frame(
            &[
                // (x=0,y=0)            (x=1,y=0)
                0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, // (x=0,y=1)            (x=1,y=1)
                0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
            ],
            Some(&test_frame),
            None,
            None,
        )
        .unwrap();

    // BLACK RED
    // BLUE  GREEN
    test_encoder
        .write_frame(
            &[
                0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00,
            ],
            Some(&test_frame),
            None,
            None,
        )
        .unwrap();

    // BLUE  BLACK
    // GREEN RED
    test_encoder
        .write_frame(
            &[
                0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00,
            ],
            Some(&test_frame),
            None,
            None,
        )
        .unwrap();
    // GREEN BLUE
    // RED   BLACK
    test_encoder
        .write_frame(
            &[
                0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            Some(&test_frame),
            None,
            None,
        )
        .unwrap();

    // !!IMPORTANT DO NOT FORGET!!
    test_encoder.finish().unwrap();

    345
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_placeholder() {
        let placeholder_value = hold_place(0);
        assert_eq!(
            345, placeholder_value,
            "placeholder test, left is expected, right is actual"
        );
    }
}
