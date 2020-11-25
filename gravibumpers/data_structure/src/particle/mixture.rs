/// This module contains a generator for a vector of pointers alternating between 2 implementations
/// of the particle traits.

pub struct VectorOfDynamicBoxedMassNormalizedStructsAndBoxes(
    pub std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>,
);

impl super::CollectionInForceField for VectorOfDynamicBoxedMassNormalizedStructsAndBoxes {
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type FixedSizeCollection = std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>;
    fn access_mutable_elements<'a>(&'a mut self) -> &'a mut Self::FixedSizeCollection {
        &mut self.0
    }

    fn add_particle(
        &mut self,
        particle_to_add: &impl super::IndividualRepresentation,
        timestep_over_inertial_mass: &super::super::time::OverMassUnit,
    ) {
        self.0.push(if (self.0.len() % 2) == 0 {
            std::boxed::Box::new(
                super::contiguous_struct::new_mass_normalized_with_force_field(
                    particle_to_add,
                    timestep_over_inertial_mass,
                ),
            )
        } else {
            std::boxed::Box::new(
                super::struct_of_boxes::new_mass_normalized_boxes_with_force_field(
                    particle_to_add,
                    timestep_over_inertial_mass,
                ),
            )
        });
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedStructsAndBoxesGenerator {}

impl super::CollectionInForceFieldGenerator
    for VectorOfDynamicBoxedMassNormalizedStructsAndBoxesGenerator
{
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type CreatedCollection = VectorOfDynamicBoxedMassNormalizedStructsAndBoxes;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfDynamicBoxedMassNormalizedStructsAndBoxes(vec![])
    }
}
