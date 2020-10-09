/// This module defines some "dimensionful" structs for representing colors. It has only struct
/// definitions and some trivial functions, and thus has no #[cfg(test)].
use super::OutOfBoundsError;

pub fn fraction_from_triplets(
    numerator_triplet: &data_structure::color::RedGreenBlueTriplet,
    denominator_value: &data_structure::color::AbsoluteUnit,
) -> Result<FractionTriplet, Box<dyn std::error::Error>> {
    // If the triplet is zero-brightness, then we take a zero fraction no matter what the reference
    // brightness is.
    if (numerator_triplet.get_red().0 == 0.0)
        && (numerator_triplet.get_green().0 == 0.0)
        && (numerator_triplet.get_blue().0 == 0.0)
    {
        return Ok(FractionTriplet {
            red_fraction: 0.0,
            green_fraction: 0.0,
            blue_fraction: 0.0,
        });
    }

    if denominator_value.0 == 0.0 {
        return Err(Box::new(OutOfBoundsError::new(&format!(
            "trying to divide {:?} by {:?}",
            numerator_triplet, denominator_value
        ))));
    }

    let fraction_factor = 1.0 / denominator_value.0;
    Ok(FractionTriplet {
        red_fraction: numerator_triplet.get_red().0 * fraction_factor,
        green_fraction: numerator_triplet.get_green().0 * fraction_factor,
        blue_fraction: numerator_triplet.get_blue().0 * fraction_factor,
    })
}

pub fn zero_brightness() -> data_structure::color::RedGreenBlueTriplet {
    data_structure::color::new_triplet(
        data_structure::color::RedUnit(0.0),
        data_structure::color::GreenUnit(0.0),
        data_structure::color::BlueUnit(0.0),
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FractionTriplet {
    red_fraction: f64,
    green_fraction: f64,
    blue_fraction: f64,
}

impl std::ops::Mul<&data_structure::color::AbsoluteUnit> for FractionTriplet {
    type Output = data_structure::color::RedGreenBlueTriplet;

    fn mul(
        self,
        reference_brightness: &data_structure::color::AbsoluteUnit,
    ) -> data_structure::color::RedGreenBlueTriplet {
        data_structure::color::new_triplet(
            data_structure::color::RedUnit(self.red_fraction * reference_brightness.0),
            data_structure::color::GreenUnit(self.green_fraction * reference_brightness.0),
            data_structure::color::BlueUnit(self.blue_fraction * reference_brightness.0),
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

pub fn fraction_triplets_match(
    expected_triplet: &FractionTriplet,
    actual_triplet: &FractionTriplet,
    absolute_tolerance: f64,
) -> bool {
    ((expected_triplet.red_fraction - actual_triplet.red_fraction).abs() <= absolute_tolerance)
        && ((expected_triplet.green_fraction - actual_triplet.green_fraction).abs()
            <= absolute_tolerance)
        && ((expected_triplet.blue_fraction - actual_triplet.blue_fraction).abs()
            <= absolute_tolerance)
}
