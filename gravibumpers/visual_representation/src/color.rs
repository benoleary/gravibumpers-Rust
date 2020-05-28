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

pub fn brightness_from(
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

#[derive(Clone, Copy, Debug)]
pub struct FractionTriplet {
    red_fraction: f64,
    green_fraction: f64,
    blue_fraction: f64,
}

pub fn fraction_from(
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
        brightness_from(
            data_structure::RedColorUnit(self.red_fraction * brightness_triplet.red_value.0),
            data_structure::GreenColorUnit(self.green_fraction * brightness_triplet.green_value.0),
            data_structure::BlueColorUnit(self.blue_fraction * brightness_triplet.blue_value.0),
        )
    }
}
