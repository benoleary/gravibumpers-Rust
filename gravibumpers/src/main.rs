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
        Box::new(visual_representation::demonstration::DemonstrationMapper {}),
        0,
    );
    let mut dummy_sequence: Vec<visual_representation::demonstration::DummyParticleVector> =
        Vec::new();
    for _ in 0..100 {
        dummy_sequence.push(visual_representation::demonstration::DummyParticleVector {});
    }
    (*demonstration_animator).animate_sequence(dummy_sequence.iter().cloned(), 100, output_filename)
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
    let mut initial_particle_map = match parsed_configuration.generator_name {
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

    let particles_in_time_evolver = time_evolution::DummyEvolver {
        number_of_copies: 23,
    };
    let particle_map_sequence =
        particles_in_time_evolver.create_time_sequence(initial_particle_map.get())?;
    println!(
        "particle_map_sequence.len() = {}",
        particle_map_sequence.len()
    );
    let mut output_file = std::fs::File::create(output_filename)?;

    // Here we copy into a vector so that we can print it as a placeholder until we implement
    // creating a pixel map out of a particle list.
    let particle_vector: std::vec::Vec<data_structure::IndividualParticle> =
        std::vec::Vec::from_iter(initial_particle_map.get());
    write!(output_file, "{:?}", particle_vector)?;
    Ok(())
}
