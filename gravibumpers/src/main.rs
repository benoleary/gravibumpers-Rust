extern crate data_structure;
extern crate visual_representation;

fn main() {
    println!("This will become GraviBumpers!");

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

    let test_animator = visual_representation::apng::new(Box::new(
        visual_representation::demonstration::DemonstrationMapper {},
    ));
    let mut dummy_sequence: Vec<Box<dyn data_structure::ParticleCollection>> = Vec::new();
    for _ in 0..40 {
        let dummy_collection: Box<dyn data_structure::ParticleCollection> =
            Box::new(visual_representation::demonstration::DummyParticleCollection {});
        dummy_sequence.push(dummy_collection);
    }
    test_animator
        .animate_sequence(&dummy_sequence, 250, "demonstration.apng")
        .unwrap()
}
