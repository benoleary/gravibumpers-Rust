/// This module provides traits and structs for representing particles.
use std::ops::Deref;
use std::ops::DerefMut;

pub mod contiguous_struct;
pub mod mixture;
pub mod struct_of_boxes;

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct IntrinsicPart {
    pub inertial_mass: super::charge::InertialMassUnit,
    pub inverse_squared_charge: super::charge::InverseSquaredChargeUnit,
    pub inverse_fourth_charge: super::charge::InverseFourthChargeUnit,
    pub color_brightness: super::color::RedGreenBlueTriplet,
}

/// The particles have some intrinsic qualities which do not change, unlike their
/// positions and velocities.
#[derive(Clone, Copy, Debug)]
pub struct VariablePart {
    pub position_vector: super::position::DimensionfulVector,
    pub velocity_vector: super::velocity::DimensionfulVector,
}

pub trait IndividualRepresentation {
    fn read_intrinsics<'a>(&'a self) -> &'a IntrinsicPart;
    fn read_variables<'a>(&'a self) -> &'a VariablePart;
}

#[derive(Clone, Copy, Debug)]
pub struct BasicIndividual {
    pub intrinsic_values: IntrinsicPart,
    pub variable_values: VariablePart,
}

impl IndividualRepresentation for BasicIndividual {
    fn read_intrinsics<'a>(&'a self) -> &'a IntrinsicPart {
        &self.intrinsic_values
    }
    fn read_variables<'a>(&'a self) -> &'a VariablePart {
        &self.variable_values
    }
}

impl IndividualRepresentation for &BasicIndividual {
    fn read_intrinsics<'a>(&'a self) -> &'a IntrinsicPart {
        &self.intrinsic_values
    }
    fn read_variables<'a>(&'a self) -> &'a VariablePart {
        &self.variable_values
    }
}

pub fn create_individual_from_representation(
    particle_representation: &impl IndividualRepresentation,
) -> BasicIndividual {
    BasicIndividual {
        intrinsic_values: *particle_representation.read_intrinsics(),
        variable_values: *particle_representation.read_variables(),
    }
}

/// In order to use Euler's method to second order, we keep the instantaneous force experienced by
/// the particle so that we can evaluate the force field at all the points with particles and only
/// then update the positions for a time step, assuming constant forces for the time step. We also
/// prepare a factor which is the common timestep of the evolution divided by the inertial mass,
/// which is used for multiplication with the force, for better efficiency.
pub trait ReadOnlyInForceField: IndividualRepresentation {
    fn into_individual_particle(&self) -> BasicIndividual;
    fn read_experienced_force<'a>(&'a self) -> &'a super::force::DimensionfulVector;
    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::time::OverMassUnit;
}

pub trait WritableInForceField: ReadOnlyInForceField {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut VariablePart;
    fn write_experienced_force<'a>(&'a mut self) -> &'a mut super::force::DimensionfulVector;
}

pub trait CollectionInForceField {
    type MutableElement: WritableInForceField;
    type FixedSizeCollection: super::collection::SingleAndPairwiseFinite<
        MutableElement = Self::MutableElement,
    >;

    fn access_mutable_elements<'a>(&'a mut self) -> &'a mut Self::FixedSizeCollection;

    fn add_particle(
        &mut self,
        particle_to_add: &impl IndividualRepresentation,
        timestep_over_inertial_mass: &super::time::OverMassUnit,
    );
}

pub trait CollectionInForceFieldGenerator {
    type MutableElement: WritableInForceField;
    type CreatedCollection: CollectionInForceField<MutableElement = Self::MutableElement>;

    fn create_collection(&self) -> Self::CreatedCollection;
}

impl IndividualRepresentation for std::boxed::Box<dyn WritableInForceField> {
    fn read_intrinsics<'a>(&self) -> &IntrinsicPart {
        self.deref().read_intrinsics()
    }

    fn read_variables<'a>(&self) -> &VariablePart {
        self.deref().read_variables()
    }
}

impl ReadOnlyInForceField for std::boxed::Box<dyn WritableInForceField> {
    fn into_individual_particle(&self) -> BasicIndividual {
        self.deref().into_individual_particle()
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::force::DimensionfulVector {
        self.deref().read_experienced_force()
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::time::OverMassUnit {
        self.deref().read_timestep_over_inertial_mass()
    }
}

impl WritableInForceField for std::boxed::Box<dyn WritableInForceField> {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut VariablePart {
        self.deref_mut().write_particle_variables()
    }

    fn write_experienced_force<'a>(&'a mut self) -> &'a mut super::force::DimensionfulVector {
        self.deref_mut().write_experienced_force()
    }
}
