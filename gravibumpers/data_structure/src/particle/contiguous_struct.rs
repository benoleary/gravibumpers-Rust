/// This module contains a pure struct implementation of the particle traits, and a
/// generator for a contiguous vector of them, and a generator for a vector of pointers
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
