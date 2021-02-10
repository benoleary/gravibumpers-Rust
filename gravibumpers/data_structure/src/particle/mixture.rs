/// This module contains generators for vectors of pointers alternating between 2 implementations
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

pub struct VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunk {
    pub boxed_representations: std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>,
    pub number_with_junk_per_without_junk: usize,
}

impl super::CollectionInForceField for VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunk {
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type FixedSizeCollection = std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>;
    fn access_mutable_elements<'a>(&'a mut self) -> &'a mut Self::FixedSizeCollection {
        &mut self.boxed_representations
    }

    fn add_particle(
        &mut self,
        particle_to_add: &impl super::IndividualRepresentation,
        timestep_over_inertial_mass: &super::super::time::OverMassUnit,
    ) {
        self.boxed_representations.push(
            if (self.boxed_representations.len() % self.number_with_junk_per_without_junk) == 0 {
                std::boxed::Box::new(
                    super::contiguous_struct::new_mass_normalized_with_force_field(
                        particle_to_add,
                        timestep_over_inertial_mass,
                    ),
                )
            } else {
                std::boxed::Box::new(
                    super::contiguous_struct::new_mass_normalized_with_force_field_and_junk(
                        particle_to_add,
                        timestep_over_inertial_mass,
                    ),
                )
            },
        );
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunkGenerator {
    pub number_with_junk_per_without_junk: usize,
}

impl super::CollectionInForceFieldGenerator
    for VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunkGenerator
{
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type CreatedCollection = VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunk;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfDynamicBoxedMassNormalizedStructsWithAndWithoutJunk {
            boxed_representations: vec![],
            number_with_junk_per_without_junk: self.number_with_junk_per_without_junk,
        }
    }
}
