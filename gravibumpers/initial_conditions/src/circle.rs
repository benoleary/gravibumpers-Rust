use super::ConfigurationParseError;

const RADIUS_LABEL: &str = "radius";
const POPULATION_LABEL: &str = "population";

pub fn from_json(
    given_configuration: &serde_json::Value,
) -> Result<Box<dyn data_structure::ParticleCollection>, Box<dyn std::error::Error>> {
    let circle_radius = match given_configuration[RADIUS_LABEL].as_f64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                RADIUS_LABEL, given_configuration
            ))))
        }
    };
    let circle_population = match given_configuration[POPULATION_LABEL].as_i64() {
        Some(parsed_number) => parsed_number,
        _ => {
            return Err(Box::new(ConfigurationParseError::new(&format!(
                "Could not parse \"{}\" from {}",
                POPULATION_LABEL, given_configuration
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
        let configuration_without_radius = serde_json::json!({
            POPULATION_LABEL: 9001
        });
        let parsing_result = from_json(&configuration_without_radius);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_radius() -> Result<(), String> {
        let configuration_with_string_radius = serde_json::json!({
            RADIUS_LABEL: "over nine thousand",
            POPULATION_LABEL: 9001
        });
        let parsing_result = from_json(&configuration_with_string_radius);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_no_population() -> Result<(), String> {
        let configuration_without_population = serde_json::json!({
            RADIUS_LABEL: 9001.0
        });
        let parsing_result = from_json(&configuration_without_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_malformed_population() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: [9001.0, 9002.0]
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_zero_population() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: 0,
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_reject_when_population_is_one() -> Result<(), String> {
        let configuration_with_array_population = serde_json::json!({
            RADIUS_LABEL: 9001.0,
            POPULATION_LABEL: 1,
        });
        let parsing_result = from_json(&configuration_with_array_population);
        if parsing_result.is_err() {
            Ok(())
        } else {
            Err(String::from("Did not get an error"))
        }
    }

    #[test]
    fn check_parse_two_points() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_parse_three_points() -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    #[test]
    fn check_parse_four_points() -> Result<(), String> {
        Err(String::from("not implemented"))
    }
}
