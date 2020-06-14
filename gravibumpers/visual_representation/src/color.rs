use super::OutOfBoundsError;

#[derive(Clone, Copy, Debug)]
pub struct BrightnessTriplet {
    red_value: data_structure::RedColorUnit,
    green_value: data_structure::GreenColorUnit,
    blue_value: data_structure::BlueColorUnit,
}

impl BrightnessTriplet {
    pub fn get_red(&self) -> data_structure::RedColorUnit {
        self.red_value
    }
    pub fn get_green(&self) -> data_structure::GreenColorUnit {
        self.green_value
    }
    pub fn get_blue(&self) -> data_structure::BlueColorUnit {
        self.blue_value
    }
}

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
    numerator_triplet: &BrightnessTriplet,
    denominator_triplet: &BrightnessTriplet,
) -> Result<FractionTriplet, Box<dyn std::error::Error>> {
    Ok(FractionTriplet {
        red_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.red_value.0,
            denominator_triplet.red_value.0,
        )?,
        green_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.green_value.0,
            denominator_triplet.green_value.0,
        )?,
        blue_fraction: divide_or_zero_if_numerator_is_zero(
            numerator_triplet.blue_value.0,
            denominator_triplet.blue_value.0,
        )?,
    })
}

pub fn brightness_from_values(
    red_value: data_structure::RedColorUnit,
    green_value: data_structure::GreenColorUnit,
    blue_value: data_structure::BlueColorUnit,
) -> BrightnessTriplet {
    BrightnessTriplet {
        red_value: red_value,
        green_value: green_value,
        blue_value: blue_value,
    }
}

pub fn zero_brightness() -> BrightnessTriplet {
    brightness_from_values(
        data_structure::RedColorUnit(0.0),
        data_structure::GreenColorUnit(0.0),
        data_structure::BlueColorUnit(0.0),
    )
}

#[derive(Clone, Copy, Debug)]
pub struct FractionTriplet {
    red_fraction: f64,
    green_fraction: f64,
    blue_fraction: f64,
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

impl std::ops::Mul<&BrightnessTriplet> for FractionTriplet {
    type Output = BrightnessTriplet;

    fn mul(self, brightness_triplet: &BrightnessTriplet) -> BrightnessTriplet {
        brightness_from_values(
            data_structure::RedColorUnit(self.red_fraction * brightness_triplet.red_value.0),
            data_structure::GreenColorUnit(self.green_fraction * brightness_triplet.green_value.0),
            data_structure::BlueColorUnit(self.blue_fraction * brightness_triplet.blue_value.0),
        )
    }
}
