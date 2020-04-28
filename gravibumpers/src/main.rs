extern crate data_structure;
extern crate visual_representation;

fn main() {
    println!("This will become GraviBumpers!");

    let initial_conditions_placeholder = initial_conditions::hold_place(12);
    println!(
        "initial_conditions_placeholder = {}",
        initial_conditions_placeholder
    );

    let time_evolution_placeholder = time_evolution::hold_place(23);
    println!(
        "time_evolution_placeholder = {}",
        time_evolution_placeholder
    );

    let test_animator = visual_representation::apng::new();
    test_animator.animate_sequence(&vec![], 250).unwrap()
}

mod animation_experiment {
    use visual_representation::ColorFraction;
    use visual_representation::HorizontalPixelAmount;
    use visual_representation::OutOfBoundsError;
    use visual_representation::RedGreenBlueFraction;
    use visual_representation::RedGreenBlueIntensity;
    use visual_representation::VerticalPixelAmount;

    const HEIGHT_IN_PIXELS: u32 = 50;
    const WIDTH_IN_PIXELS: u32 = 100;

    struct ExperimentalColoredPixelMatrix {
        time_index: u8,
    }

    impl visual_representation::ColoredPixelMatrix for ExperimentalColoredPixelMatrix {
        fn color_fractions_at(
            &self,
            reference_intensity: &RedGreenBlueIntensity,
            horizontal_pixels_from_bottom_left: &HorizontalPixelAmount,
            vertical_pixels_from_bottom_left: &VerticalPixelAmount,
        ) -> Result<RedGreenBlueFraction, Box<dyn std::error::Error>> {
            let horizontal_coordinate = horizontal_pixels_from_bottom_left.0;
            let vertical_coordinate = vertical_pixels_from_bottom_left.0;

            if (horizontal_coordinate >= WIDTH_IN_PIXELS)
                || (vertical_coordinate >= HEIGHT_IN_PIXELS)
            {
                return Err(Box::new(OutOfBoundsError::new(&format!(
                    "horizontal_pixels_from_bottom_left {}, vertical_pixels_from_bottom_left {}",
                    horizontal_pixels_from_bottom_left.0, vertical_pixels_from_bottom_left.0
                ))));
            }

            let mut color_fraction_triplet = RedGreenBlueFraction {
                red_fraction: ColorFraction(0.5 * (self.time_index as f64)),
                green_fraction: ColorFraction(0.5 * (self.time_index as f64)),
                blue_fraction: ColorFraction(0.5 * (self.time_index as f64)),
            };

            Ok(color_fraction_triplet)
        }

        fn width_in_pixels(&self) -> HorizontalPixelAmount {
            HorizontalPixelAmount(WIDTH_IN_PIXELS)
        }
        fn height_in_pixels(&self) -> VerticalPixelAmount {
            VerticalPixelAmount(HEIGHT_IN_PIXELS)
        }
    }
}
