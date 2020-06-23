/// This module defines some "dimensionful" structs for representing colors. It has only struct
/// definitions and some trivial functions, and thus has no #[cfg(test)].
use super::OutOfBoundsError;

fn divide_or_zero_if_numerator_is_zero(
    numerator_value: f64,
    denominator_value: f64,
) -> Result<f64, Box<dyn std::error::Error>> {
    if numerator_value == 0.0 {
        Ok(0.0)
    } else if denominator_value == 0.0 {
        Err(Box::new(OutOfBoundsError::new(&format!(
            "trying to divide {:?} by {:?}",
            numerator_value, denominator_value
        ))))
    } else {
        Ok(numerator_value / denominator_value)
    }
}

pub fn fraction_from_triplets(
    numerator_triplet: &data_structure::ColorTriplet,
    denominator_triplet: &data_structure::ColorTriplet,
) -> Result<FractionTriplet, Box<dyn std::error::Error>> {
    Ok(FractionTriplet {
        red_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.get_red().0,
            denominator_triplet.get_red().0,
        )?,
        green_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.get_green().0,
            denominator_triplet.get_green().0,
        )?,
        blue_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.get_blue().0,
            denominator_triplet.get_blue().0,
        )?,
    })
}

pub fn zero_brightness() -> data_structure::ColorTriplet {
    data_structure::new_color_triplet(
        data_structure::RedColorUnit(0.0),
        data_structure::GreenColorUnit(0.0),
        data_structure::BlueColorUnit(0.0),
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FractionTriplet {
    red_fraction: f64,
    green_fraction: f64,
    blue_fraction: f64,
}

impl std::ops::Mul<&data_structure::ColorTriplet> for FractionTriplet {
    type Output = data_structure::ColorTriplet;

    fn mul(
        self,
        brightness_triplet: &data_structure::ColorTriplet,
    ) -> data_structure::ColorTriplet {
        data_structure::new_color_triplet(
            data_structure::RedColorUnit(self.red_fraction * brightness_triplet.get_red().0),
            data_structure::GreenColorUnit(self.green_fraction * brightness_triplet.get_green().0),
            data_structure::BlueColorUnit(self.blue_fraction * brightness_triplet.get_blue().0),
        )
    }
}

pub fn fraction_from_values(
    red_fraction: f64,
    green_fraction: f64,
    blue_fraction: f64,
) -> FractionTriplet {
    FractionTriplet {
        red_fraction: red_fraction,
        green_fraction: green_fraction,
        blue_fraction: blue_fraction,
    }
}

pub fn zero_fraction() -> FractionTriplet {
    fraction_from_values(0.0, 0.0, 0.0)
}
