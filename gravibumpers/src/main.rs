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

    let test_animator = visual_representation::apng::new();
    test_animator.animate_sequence(&vec![], 250).unwrap()
}
