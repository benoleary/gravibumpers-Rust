extern crate data_structure;
extern crate serde_json;
extern crate visual_representation;

use contiguous_particle_struct::VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator;
use contiguous_particle_struct::VectorOfMassNormalizedWithForceFieldGenerator;
use data_structure::particle::contiguous_struct as contiguous_particle_struct;
use data_structure::particle::struct_of_boxes as particle_struct_of_boxes;
use particle_struct_of_boxes::VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator;
use visual_representation::SequenceAnimator;

fn print_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("GraviBumpers!");
    println!("The first argument should be the mode. Currently implemented: rgb_demo, read_file");
    println!("rgb_demo expects 1 further argument: the filename for the output APNG.");
    println!(
        "read_file expects 3 further arguments: the filename of the configuration, then the \
        filename for the output, then a single word to determine if off-screen particles should \
        be drawn on the border (case-insensitive 'yes' or 'true' to draw them, 'no' or 'false' \
        leave them undrawn)."
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
    let ignored_particle = data_structure::particle::BasicIndividual {
        intrinsic_values: data_structure::particle::IntrinsicPart {
            inertial_mass: data_structure::charge::InertialMassUnit(1.9),
            inverse_squared_charge: data_structure::charge::InverseSquaredChargeUnit(2.8),
            inverse_fourth_charge: data_structure::charge::InverseFourthChargeUnit(3.7),
            color_brightness: data_structure::color::new_triplet(
                data_structure::color::RedUnit(4.6),
                data_structure::color::GreenUnit(5.5),
                data_structure::color::BlueUnit(6.4),
            ),
        },
        variable_values: data_structure::particle::VariablePart {
            position_vector: data_structure::position::DimensionfulVector {
                horizontal_component: data_structure::position::HorizontalUnit(1.0),
                vertical_component: data_structure::position::VerticalUnit(-1.0),
            },
            velocity_vector: data_structure::velocity::DimensionfulVector {
                horizontal_component: data_structure::velocity::HorizontalUnit(0.1),
                vertical_component: data_structure::velocity::VerticalUnit(-0.1),
            },
        },
    };
    let mut dummy_sequence: std::vec::Vec<
        visual_representation::demonstration::SingleParticleBorrowIterator,
    > = std::vec::Vec::new();
    for _ in 0..100 {
        dummy_sequence.push(visual_representation::demonstration::new_borrow_iterator(
            &ignored_particle,
        ));
    }
    demonstration_animator.animate_sequence(dummy_sequence.into_iter(), 100, output_filename)
}

fn evolve_and_animate(
    parsed_configuration: &configuration_parsing::ParsedConfiguration,
    particles_in_time_evolver: &mut impl time_evolution::ParticlesInTimeEvolver,
    initial_particle_configuration: impl std::iter::ExactSizeIterator<
        Item = impl data_structure::particle::IndividualRepresentation,
    >,
    should_draw_offscreen_on_border: bool,
    output_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let instant_before_evolution = std::time::Instant::now();
    let particle_set_evolution = particles_in_time_evolver.create_time_sequence(
        &parsed_configuration.evolution_configuration,
        initial_particle_configuration,
    )?;

    println!(
        "Calculation of time evolution took {}ms",
        instant_before_evolution.elapsed().as_millis()
    );

    let picture_configuration = &parsed_configuration.picture_configuration;
    let pixel_brightness_aggregator = visual_representation::brightness_aggregator::new(
        visual_representation::HorizontalPixelAmount(picture_configuration.right_border_coordinate),
        visual_representation::VerticalPixelAmount(picture_configuration.upper_border_coordinate),
        visual_representation::HorizontalPixelAmount(picture_configuration.left_border_coordinate),
        visual_representation::VerticalPixelAmount(picture_configuration.lower_border_coordinate),
        should_draw_offscreen_on_border,
    )?;
    let particle_animator = visual_representation::apng::new(pixel_brightness_aggregator, 1);

    let instant_before_animation = std::time::Instant::now();
    particle_animator.animate_sequence(
        particle_set_evolution.particle_configurations,
        particle_set_evolution.milliseconds_between_configurations,
        output_filename,
    )?;

    println!(
        "Animation took {}ms",
        instant_before_animation.elapsed().as_millis()
    );

    Ok(())
}

fn run_from_configuration_file(
    command_line_arguments: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("GraviBumpers!");
    if command_line_arguments.len() != 5 {
        return print_help();
    }

    let input_filename = &command_line_arguments[2];
    let output_filename = &command_line_arguments[3];
    let input_for_drawing_offscreen = &command_line_arguments[4];
    let should_draw_offscreen_on_border = String::from("yes")
        .eq_ignore_ascii_case(input_for_drawing_offscreen)
        || String::from("true").eq_ignore_ascii_case(input_for_drawing_offscreen);
    if !should_draw_offscreen_on_border
        && !(String::from("no").eq_ignore_ascii_case(input_for_drawing_offscreen)
            || String::from("false").eq_ignore_ascii_case(input_for_drawing_offscreen))
    {
        return print_help();
    }

    println!(
        "Reading configuration from {}, will write to {}",
        input_filename, output_filename
    );

    let instant_before_configuration = std::time::Instant::now();

    let mut initial_particle_map: std::vec::Vec<data_structure::particle::BasicIndividual> = vec![];
    let configuration_content = std::fs::read_to_string(input_filename)?;
    let deserialized_configuration: serde_json::Value =
        serde_json::from_str(&configuration_content)?;
    let parsed_configuration =
        configuration_parsing::parse_deserialized_configuration(&deserialized_configuration)?;
    for generator_configuration in parsed_configuration.generator_configurations.iter() {
        let initial_particles_from_configuration = match generator_configuration.generator_name {
            "single" => initial_conditions::single::from_json(
                generator_configuration.generator_configuration,
            ),
            "circle" => initial_conditions::circle::from_json(
                generator_configuration.generator_configuration,
            ),
            _ => {
                return Err(Box::new(
                    configuration_parsing::ConfigurationParseError::new(&format!(
                        "Generator name \"{}\" is unknown",
                        generator_configuration.generator_name
                    )),
                ))
            }
        }?;
        initial_particle_map.extend(initial_particles_from_configuration.iter());
    }

    println!(
        "Reading configuration took {}ms",
        instant_before_configuration.elapsed().as_millis()
    );

    match parsed_configuration.evolver_configuration.memory_layout {
        "VecOfPureStruct" => {
            let mut particles_in_time_evolver =
                time_evolution::second_order_euler::new_given_memory_strategy(
                    parsed_configuration
                        .evolver_configuration
                        .number_of_steps_per_time_slice,
                    VectorOfMassNormalizedWithForceFieldGenerator {},
                )?;
            evolve_and_animate(
                &parsed_configuration,
                &mut particles_in_time_evolver,
                initial_particle_map.iter(),
                should_draw_offscreen_on_border,
                output_filename,
            )
        }
        "VecOfBoxedStruct" => {
            let mut particles_in_time_evolver =
                time_evolution::second_order_euler::new_given_memory_strategy(
                    parsed_configuration
                        .evolver_configuration
                        .number_of_steps_per_time_slice,
                    VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator {},
                )?;
            evolve_and_animate(
                &parsed_configuration,
                &mut particles_in_time_evolver,
                initial_particle_map.iter(),
                should_draw_offscreen_on_border,
                output_filename,
            )
        }
        "VecOfDoubleBoxed" => {
            let mut particles_in_time_evolver =
                time_evolution::second_order_euler::new_given_memory_strategy(
                    parsed_configuration
                        .evolver_configuration
                        .number_of_steps_per_time_slice,
                    VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator {},
                )?;
            evolve_and_animate(
                &parsed_configuration,
                &mut particles_in_time_evolver,
                initial_particle_map.iter(),
                should_draw_offscreen_on_border,
                output_filename,
            )
        }
        _ => Err(Box::new(
            configuration_parsing::ConfigurationParseError::new(&format!(
                "Memory layout \"{}\" is unknown",
                parsed_configuration.evolver_configuration.memory_layout
            )),
        )),
    }
}

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
