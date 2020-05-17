use super::ConfigurationParseError;

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    let circle_radius = match given_configuration["radius"].as_f64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"radius\" from {}",
                given_configuration
            ))))
        }
    };
    let circle_population = match given_configuration["population"].as_i64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"population\" from {}",
                given_configuration
            ))))
        }
    };
    from_numbers(circle_radius, circle_population)
}

fn from_numbers(
    circle_radius: f64,
    circle_population: i64,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    Err(Box::new(ConfigurationParseError::new(&format!(
        "Not yet implemented"
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reject_when_no_radius() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_malformed_radius() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_no_population() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_reject_when_malformed_population() -> Result<(), String> {
        Err(String::from("not implemented"))
    }
}
