extern crate apng_encoder;

use super::SequenceAnimator;
use std::convert::TryInto;
pub fn new() -> Box<dyn SequenceAnimator> {
    Box::new(ApngAnimator {
        color_palette: apng_encoder::Color::RGB(8),
    })
}

const MILLISECONDS_PER_SECOND: u16 = 1000;

struct ApngAnimator {
    color_palette: apng_encoder::Color,
}

impl SequenceAnimator for ApngAnimator {
    fn animate_sequence(
        &self,
        particle_map_sequence: &[Box<dyn data_structure::ParticleCollection>],
        milliseconds_per_frame: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let milliseconds_per_frame_as_short: u16 = milliseconds_per_frame.try_into()?;
        // This is copy-pasted from the example displayed on the crates.io page for the apng-encoder
        // crate, with some re-naming of variables.
        // It is serving as a basis for making GIFs of the mass maps.
        // Generate 2x2 Animated PNG (4 frames)
        let test_meta = apng_encoder::Meta {
            width: 2,
            height: 2,
            color: self.color_palette,
            frames: 4,
            plays: None, // Infinite loop
        };

        // Delay = 1/2 (0.5) seconds
        let test_frame = apng_encoder::Frame {
            delay: Some(apng_encoder::Delay::new(
                milliseconds_per_frame_as_short,
                MILLISECONDS_PER_SECOND,
            )),
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

        Ok(())
    }
}
