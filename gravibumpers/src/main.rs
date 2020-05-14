extern crate data_structure;
extern crate visual_representation;

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
    println!("The first argument should be the mode. Currently implemented: rgb_demo, no_op");
    println!("rgb_demo expects 1 further argument: the filename for the output APNG.");
    println!("read_file expects 1 further argument: the filename of the configuration.");
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
    let mut dummy_sequence: Vec<visual_representation::demonstration::DummyParticleCollection> =
        Vec::new();
    for _ in 0..100 {
        dummy_sequence.push(visual_representation::demonstration::DummyParticleCollection {});
    }
    (*demonstration_animator).animate_sequence(&mut dummy_sequence.iter(), 100, output_filename)
}

fn run_from_configuration_file(
    command_line_arguments: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("This will become GraviBumpers!");
    if command_line_arguments.len() != 3 {
        return print_help();
    }

    let input_filename = &command_line_arguments[2];
    println!("reading configuration from {}", input_filename);

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
    Ok(())
}
