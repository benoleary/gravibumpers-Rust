/// This module contains pure struct implementations of the particle traits, and
/// generators for contiguous vectors of them, and generators for vectors of pointers
/// to them.

pub struct MassNormalizedWithForceField {
    particle_description: super::BasicIndividual,
    experienced_force: super::super::force::DimensionfulVector,
    timestep_over_inertial_mass: super::super::time::OverMassUnit,
}

pub fn new_mass_normalized_with_force_field(
    particle_to_add: &impl super::IndividualRepresentation,
    timestep_over_inertial_mass: &super::super::time::OverMassUnit,
) -> MassNormalizedWithForceField {
    MassNormalizedWithForceField {
        particle_description: super::create_individual_from_representation(particle_to_add),
        experienced_force: super::super::force::DimensionfulVector {
            horizontal_component: super::super::force::HorizontalUnit(0.0),
            vertical_component: super::super::force::VerticalUnit(0.0),
        },
        timestep_over_inertial_mass: *timestep_over_inertial_mass,
    }
}

impl super::IndividualRepresentation for MassNormalizedWithForceField {
    fn read_intrinsics<'a>(&'a self) -> &'a super::IntrinsicPart {
        self.particle_description.read_intrinsics()
    }

    fn read_variables<'a>(&'a self) -> &'a super::VariablePart {
        self.particle_description.read_variables()
    }
}

impl super::ReadOnlyInForceField for MassNormalizedWithForceField {
    fn into_individual_particle(&self) -> super::BasicIndividual {
        self.particle_description
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::super::force::DimensionfulVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::super::time::OverMassUnit {
        &self.timestep_over_inertial_mass
    }
}

impl super::WritableInForceField for MassNormalizedWithForceField {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut super::VariablePart {
        &mut self.particle_description.variable_values
    }

    fn write_experienced_force<'a>(
        &'a mut self,
    ) -> &'a mut super::super::force::DimensionfulVector {
        &mut self.experienced_force
    }
}

pub struct VectorOfMassNormalizedWithForceField(pub std::vec::Vec<MassNormalizedWithForceField>);

impl super::CollectionInForceField for VectorOfMassNormalizedWithForceField {
    type MutableElement = MassNormalizedWithForceField;
    type FixedSizeCollection = std::vec::Vec<MassNormalizedWithForceField>;
    fn access_mutable_elements<'a>(&'a mut self) -> &'a mut Self::FixedSizeCollection {
        &mut self.0
    }

    fn add_particle(
        &mut self,
        particle_to_add: &impl super::IndividualRepresentation,
        timestep_over_inertial_mass: &super::super::time::OverMassUnit,
    ) {
        self.0.push(new_mass_normalized_with_force_field(
            particle_to_add,
            timestep_over_inertial_mass,
        ));
    }
}

pub struct VectorOfMassNormalizedWithForceFieldGenerator {}

impl super::CollectionInForceFieldGenerator for VectorOfMassNormalizedWithForceFieldGenerator {
    type MutableElement = MassNormalizedWithForceField;
    type CreatedCollection = VectorOfMassNormalizedWithForceField;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfMassNormalizedWithForceField(vec![])
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedWithForceField(
    pub std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>,
);

impl super::CollectionInForceField for VectorOfDynamicBoxedMassNormalizedWithForceField {
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
        self.0
            .push(std::boxed::Box::new(MassNormalizedWithForceField {
                particle_description: super::create_individual_from_representation(particle_to_add),
                experienced_force: super::super::force::DimensionfulVector {
                    horizontal_component: super::super::force::HorizontalUnit(0.0),
                    vertical_component: super::super::force::VerticalUnit(0.0),
                },
                timestep_over_inertial_mass: *timestep_over_inertial_mass,
            }));
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator {}

impl super::CollectionInForceFieldGenerator
    for VectorOfDynamicBoxedMassNormalizedWithForceFieldGenerator
{
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type CreatedCollection = VectorOfDynamicBoxedMassNormalizedWithForceField;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfDynamicBoxedMassNormalizedWithForceField(vec![])
    }
}

const NUMBER_OF_JUNK_COPIES: usize = 10000;

pub struct MassNormalizedWithForceFieldAndJunk {
    particle_descriptions: [super::BasicIndividual; NUMBER_OF_JUNK_COPIES],
    experienced_force: super::super::force::DimensionfulVector,
    timestep_over_inertial_mass: super::super::time::OverMassUnit,
    current_index: usize,
}

fn create_array_of_copied_individuals_from_representation(
    particle_to_add: &impl super::IndividualRepresentation,
) -> [super::BasicIndividual; NUMBER_OF_JUNK_COPIES] {
    [super::create_individual_from_representation(particle_to_add); NUMBER_OF_JUNK_COPIES]
}

pub fn new_mass_normalized_with_force_field_and_junk(
    particle_to_add: &impl super::IndividualRepresentation,
    timestep_over_inertial_mass: &super::super::time::OverMassUnit,
) -> MassNormalizedWithForceFieldAndJunk {
    MassNormalizedWithForceFieldAndJunk {
        particle_descriptions: create_array_of_copied_individuals_from_representation(
            particle_to_add,
        ),
        experienced_force: super::super::force::DimensionfulVector {
            horizontal_component: super::super::force::HorizontalUnit(0.0),
            vertical_component: super::super::force::VerticalUnit(0.0),
        },
        timestep_over_inertial_mass: *timestep_over_inertial_mass,
        current_index: 0,
    }
}

impl super::IndividualRepresentation for MassNormalizedWithForceFieldAndJunk {
    fn read_intrinsics<'a>(&'a self) -> &'a super::IntrinsicPart {
        self.particle_descriptions[self.current_index].read_intrinsics()
    }

    fn read_variables<'a>(&'a self) -> &'a super::VariablePart {
        self.particle_descriptions[self.current_index].read_variables()
    }
}

impl super::ReadOnlyInForceField for MassNormalizedWithForceFieldAndJunk {
    fn into_individual_particle(&self) -> super::BasicIndividual {
        self.particle_descriptions[self.current_index]
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::super::force::DimensionfulVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::super::time::OverMassUnit {
        &self.timestep_over_inertial_mass
    }
}

impl super::WritableInForceField for MassNormalizedWithForceFieldAndJunk {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut super::VariablePart {
        let next_index = self.current_index + 1;
        self.particle_descriptions[next_index] = self.particle_descriptions[self.current_index];
        self.current_index = next_index;
        &mut self.particle_descriptions[self.current_index].variable_values
    }

    fn write_experienced_force<'a>(
        &'a mut self,
    ) -> &'a mut super::super::force::DimensionfulVector {
        &mut self.experienced_force
    }
}

pub struct VectorOfMassNormalizedWithForceFieldAndJunk(
    pub std::vec::Vec<MassNormalizedWithForceFieldAndJunk>,
);

impl super::CollectionInForceField for VectorOfMassNormalizedWithForceFieldAndJunk {
    type MutableElement = MassNormalizedWithForceFieldAndJunk;
    type FixedSizeCollection = std::vec::Vec<MassNormalizedWithForceFieldAndJunk>;
    fn access_mutable_elements<'a>(&'a mut self) -> &'a mut Self::FixedSizeCollection {
        &mut self.0
    }

    fn add_particle(
        &mut self,
        particle_to_add: &impl super::IndividualRepresentation,
        timestep_over_inertial_mass: &super::super::time::OverMassUnit,
    ) {
        self.0.push(new_mass_normalized_with_force_field_and_junk(
            particle_to_add,
            timestep_over_inertial_mass,
        ));
    }
}

pub struct VectorOfMassNormalizedWithForceFieldAndJunkGenerator {}

impl super::CollectionInForceFieldGenerator
    for VectorOfMassNormalizedWithForceFieldAndJunkGenerator
{
    type MutableElement = MassNormalizedWithForceFieldAndJunk;
    type CreatedCollection = VectorOfMassNormalizedWithForceFieldAndJunk;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfMassNormalizedWithForceFieldAndJunk(vec![])
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunk(
    pub std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>,
);

impl super::CollectionInForceField for VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunk {
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
        self.0
            .push(std::boxed::Box::new(MassNormalizedWithForceFieldAndJunk {
                particle_descriptions: create_array_of_copied_individuals_from_representation(
                    particle_to_add,
                ),
                experienced_force: super::super::force::DimensionfulVector {
                    horizontal_component: super::super::force::HorizontalUnit(0.0),
                    vertical_component: super::super::force::VerticalUnit(0.0),
                },
                timestep_over_inertial_mass: *timestep_over_inertial_mass,
                current_index: 0,
            }));
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunkGenerator {}

impl super::CollectionInForceFieldGenerator
    for VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunkGenerator
{
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type CreatedCollection = VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunk;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfDynamicBoxedMassNormalizedWithForceFieldAndJunk(vec![])
    }
}
