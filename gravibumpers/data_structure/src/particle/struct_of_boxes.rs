/// This module contains an implementation of the particle traits which boxes its internal
/// data members, and a generator for a vector of pointers to instances of that implementation.

pub struct MassNormalizedBoxesWithForceField {
    intrinsic_values: std::boxed::Box<super::IntrinsicPart>,
    variable_values: std::boxed::Box<super::VariablePart>,
    experienced_force: std::boxed::Box<super::super::force::DimensionfulVector>,
    timestep_over_inertial_mass: std::boxed::Box<super::super::time::OverMassUnit>,
}

pub fn new_mass_normalized_boxes_with_force_field(
    particle_to_add: &impl super::IndividualRepresentation,
    timestep_over_inertial_mass: &super::super::time::OverMassUnit,
) -> MassNormalizedBoxesWithForceField {
    let basic_individual = super::create_individual_from_representation(particle_to_add);
    MassNormalizedBoxesWithForceField {
        intrinsic_values: std::boxed::Box::new(basic_individual.intrinsic_values),
        variable_values: std::boxed::Box::new(basic_individual.variable_values),
        experienced_force: std::boxed::Box::new(super::super::force::DimensionfulVector {
            horizontal_component: super::super::force::HorizontalUnit(0.0),
            vertical_component: super::super::force::VerticalUnit(0.0),
        }),
        timestep_over_inertial_mass: std::boxed::Box::new(*timestep_over_inertial_mass),
    }
}

impl super::IndividualRepresentation for MassNormalizedBoxesWithForceField {
    fn read_intrinsics<'a>(&'a self) -> &'a super::IntrinsicPart {
        &self.intrinsic_values
    }

    fn read_variables<'a>(&'a self) -> &'a super::VariablePart {
        &self.variable_values
    }
}

impl super::ReadOnlyInForceField for MassNormalizedBoxesWithForceField {
    fn into_individual_particle(&self) -> super::BasicIndividual {
        super::create_individual_from_representation(self)
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::super::force::DimensionfulVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::super::time::OverMassUnit {
        &self.timestep_over_inertial_mass
    }
}

impl super::WritableInForceField for MassNormalizedBoxesWithForceField {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut super::VariablePart {
        &mut self.variable_values
    }

    fn write_experienced_force<'a>(
        &'a mut self,
    ) -> &'a mut super::super::force::DimensionfulVector {
        &mut self.experienced_force
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedBoxesWithForceField(
    pub std::vec::Vec<std::boxed::Box<dyn super::WritableInForceField>>,
);

impl super::CollectionInForceField for VectorOfDynamicBoxedMassNormalizedBoxesWithForceField {
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
        self.0.push(std::boxed::Box::new(
            new_mass_normalized_boxes_with_force_field(
                particle_to_add,
                timestep_over_inertial_mass,
            ),
        ));
    }
}

pub struct VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator {}

impl super::CollectionInForceFieldGenerator
    for VectorOfDynamicBoxedMassNormalizedBoxesWithForceFieldGenerator
{
    type MutableElement = std::boxed::Box<dyn super::WritableInForceField>;
    type CreatedCollection = VectorOfDynamicBoxedMassNormalizedBoxesWithForceField;

    fn create_collection(&self) -> Self::CreatedCollection {
        VectorOfDynamicBoxedMassNormalizedBoxesWithForceField(vec![])
    }
}
