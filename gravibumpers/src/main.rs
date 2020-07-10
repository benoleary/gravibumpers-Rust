extern crate data_structure;
extern crate serde_json;
extern crate visual_representation;

use std::io::Write;
use std::iter::FromIterator;
use time_evolution::ParticlesInTimeEvolver;
use visual_representation::SequenceAnimator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_line_arguments: Vec<String> = std::env::args().collect();

    if command_line_arguments.len() < 2 {
        return print_help();
    }

    return match command_line_arguments[1].as_str() {
        "rgb_demo" => create_rgb_demonstration(&command_line_arguments),
        "read_file" => run_from_configuration_file(&command_line_arguments),
        _ => print_help(),
    };
}

fn print_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("GraviBumpers!");
    println!("The first argument should be the mode. Currently implemented: rgb_demo, read_file");
    println!("rgb_demo expects 1 further argument: the filename for the output APNG.");
    println!(
        "read_file expects 2 further argument: the filename of the configuration, then the \
        filename for the output."
    );
    Ok(())
}

fn create_rgb_demonstration(
    command_line_arguments: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if command_line_arguments.len() != 3 {
        return print_help();
    }

    let output_filename = &command_line_arguments[2];
    let demonstration_animator = visual_representation::apng::new(
        visual_representation::demonstration::DemonstrationMapper {},
        0,
    );
    let ignored_particle = data_structure::IndividualParticle {
        intrinsic_values: data_structure::ParticleIntrinsics {
            inertial_mass: data_structure::InertialMassUnit(1.9),
            attractive_charge: data_structure::AttractiveChargeUnit(2.8),
            repulsive_charge: data_structure::RepulsiveChargeUnit(3.7),
            color_brightness: data_structure::new_color_triplet(
                data_structure::RedColorUnit(4.6),
                data_structure::GreenColorUnit(5.5),
                data_structure::BlueColorUnit(6.4),
            ),
        },
        variable_values: data_structure::ParticleVariables {
            horizontal_position: data_structure::HorizontalPositionUnit(1.0),
            vertical_position: data_structure::VerticalPositionUnit(-1.0),
            horizontal_velocity: data_structure::HorizontalVelocityUnit(0.1),
            vertical_velocity: data_structure::VerticalVelocityUnit(-0.1),
        },
    };
    let mut dummy_sequence: std::vec::Vec<
        visual_representation::demonstration::SingleParticleCopyIterator,
    > = std::vec::Vec::new();
    for _ in 0..100 {
        dummy_sequence.push(visual_representation::demonstration::new_copy_iterator(
            &ignored_particle,
        ));
    }
    demonstration_animator.animate_sequence(dummy_sequence.into_iter(), 100, output_filename)
}

fn run_from_configuration_file(
    command_line_arguments: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("This will become GraviBumpers!");
    if command_line_arguments.len() != 4 {
        return print_help();
    }

    let input_filename = &command_line_arguments[2];
    let output_filename = &command_line_arguments[3];
    println!(
        "reading configuration from {}, will write to {}",
        input_filename, output_filename
    );

    let configuration_content = std::fs::read_to_string(input_filename)?;
    let deserialized_configuration: serde_json::Value =
        serde_json::from_str(&configuration_content)?;
    let parsed_configuration =
        initial_conditions::parse_deserialized_configuration(&deserialized_configuration)?;
    let initial_particle_map = match parsed_configuration.generator_name {
        "circle" => {
            initial_conditions::circle::from_json(parsed_configuration.generator_configuration)
        }
        _ => {
            return Err(Box::new(initial_conditions::ConfigurationParseError::new(
                &format!(
                    "Generator name \"{}\" is unknown",
                    parsed_configuration.generator_name
                ),
            )))
        }
    }?;

    let mut particles_in_time_evolver = time_evolution::DummyEvolver {
        number_of_copies: 23,
    };
    let particle_map_sequence =
        particles_in_time_evolver.create_time_sequence(initial_particle_map.iter())?;
    println!(
        "particle_map_sequence.len() = {}",
        particle_map_sequence.len()
    );
    let mut raw_output_file = std::fs::File::create(format!("raw_{}", output_filename))?;

    // Here we copy into a vector so that we can print it as a placeholder until we implement
    // creating a pixel map out of a particle list.
    let particle_vector: std::vec::Vec<data_structure::IndividualParticle> =
        std::vec::Vec::from_iter(initial_particle_map.iter().map(|particle_representation| {
            data_structure::create_individual_from_representation(particle_representation)
        }));
    write!(raw_output_file, "{:?}", particle_vector)?;

    let pixel_brightness_aggregator = visual_representation::brightness_aggregator::new(
        visual_representation::HorizontalPixelAmount(-10),
        visual_representation::HorizontalPixelAmount(10),
        visual_representation::VerticalPixelAmount(-10),
        visual_representation::VerticalPixelAmount(10),
        false,
    )
    .expect("Fixed borders should not be wrong");
    let particle_animator = visual_representation::apng::new(pixel_brightness_aggregator, 1);
    particle_animator.animate_sequence(particle_map_sequence, 100, output_filename)
}
