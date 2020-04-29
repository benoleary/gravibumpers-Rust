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

    const HEIGHT_IN_PIXELS: VerticalPixelAmount = VerticalPixelAmount(50);
    const WIDTH_IN_PIXELS: HorizontalPixelAmount = HorizontalPixelAmount(100);

    pub fn new_matrix(time_index: u32) -> ExperimentalColoredPixelMatrix {
        ExperimentalColoredPixelMatrix {
            red_upper_edge: VerticalPixelAmount(time_index),
            red_lower_edge: VerticalPixelAmount(time_index),
            red_left_edge: HorizontalPixelAmount(time_index),
            red_peak_line: HorizontalPixelAmount(time_index),
            red_right_edge: HorizontalPixelAmount(time_index),
        }
    }

    struct ExperimentalColoredPixelMatrix {
        red_upper_edge: VerticalPixelAmount,
        red_lower_edge: VerticalPixelAmount,
        red_left_edge: HorizontalPixelAmount,
        red_peak_line: HorizontalPixelAmount,
        red_right_edge: HorizontalPixelAmount,
    }

    impl visual_representation::ColoredPixelMatrix for ExperimentalColoredPixelMatrix {
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

            let mut color_fraction_triplet = RedGreenBlueFraction {
                red_fraction: ColorFraction(0.0),
                green_fraction: ColorFraction(0.0),
                blue_fraction: ColorFraction(0.0),
            };

            if vertical_pixels_from_bottom_left < &VerticalPixelAmount(30) {
                //if horizontal_pixels_from_bottom_left > HorizontalPixelAmount(t)
            };

            Ok(color_fraction_triplet)
        }

        fn width_in_pixels(&self) -> &HorizontalPixelAmount {
            &WIDTH_IN_PIXELS
        }
        fn height_in_pixels(&self) -> &VerticalPixelAmount {
            &HEIGHT_IN_PIXELS
        }
    }
}
