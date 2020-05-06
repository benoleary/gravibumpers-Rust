extern crate data_structure;
extern crate visual_representation;

use visual_representation::SequenceAnimator;

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
    let mut dummy_sequence: Vec<visual_representation::demonstration::DummyParticleCollection> =
        Vec::new();
    for _ in 0..100 {
        dummy_sequence.push(visual_representation::demonstration::DummyParticleCollection {});
    }
    (*test_animator)
        .animate_sequence(&mut dummy_sequence.iter(), 100, "demonstration.apng")
        .unwrap()
}
