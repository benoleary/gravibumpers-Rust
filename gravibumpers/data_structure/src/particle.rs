/// This module provides traits and structs for representing particles.

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
    fn read_intrinsics(&self) -> &IntrinsicPart;
    fn read_variables(&self) -> &VariablePart;
}

#[derive(Clone, Copy, Debug)]
pub struct BasicIndividual {
    pub intrinsic_values: IntrinsicPart,
    pub variable_values: VariablePart,
}

impl IndividualRepresentation for BasicIndividual {
    fn read_intrinsics(&self) -> &IntrinsicPart {
        &self.intrinsic_values
    }
    fn read_variables(&self) -> &VariablePart {
        &self.variable_values
    }
}

impl IndividualRepresentation for &BasicIndividual {
    fn read_intrinsics(&self) -> &IntrinsicPart {
        &self.intrinsic_values
    }
    fn read_variables(&self) -> &VariablePart {
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
pub trait IndividualInForceField: IndividualRepresentation + Sized {
    fn into_individual_particle(&self) -> BasicIndividual;
    fn read_experienced_force<'a>(&'a self) -> &'a super::force::DimensionfulVector;
    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::time::OverMassUnit;
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut VariablePart;
    fn write_experienced_force<'a>(&'a mut self) -> &'a mut super::force::DimensionfulVector;
}

/// The trait should have a consistent order of iteration.
pub trait IndexedCollectionInForceField<'a>:
    std::ops::Index<usize> + std::ops::IndexMut<usize> + Sized
where
    <Self as std::ops::Index<usize>>::Output: IndividualInForceField + 'a,
    Self: 'a,
{
    type ImmutableIterator: std::iter::ExactSizeIterator<
        Item = &'a <Self as std::ops::Index<usize>>::Output,
    >;
    type MutableIterator: std::iter::ExactSizeIterator<
        Item = &'a mut <Self as std::ops::Index<usize>>::Output,
    >;
    fn add_particle(
        &mut self,
        particle_to_add: &impl IndividualRepresentation,
        timestep_over_inertial_mass: &super::time::OverMassUnit,
    );
    fn get_length(&self) -> usize;
    fn get_immutable_iterator(&'a mut self) -> Self::ImmutableIterator;
    fn get_mutable_iterator(&'a mut self) -> Self::MutableIterator;
    fn reset_forces(&mut self);
    fn apply_pairwise_force(
        &mut self,
        index_for_adding_given_force: usize,
        index_for_subtracting_given_force: usize,
        force_vector: &super::force::DimensionfulVector,
    );
}

pub struct MassNormalizedWithForceField {
    particle_description: BasicIndividual,
    experienced_force: super::force::DimensionfulVector,
    timestep_over_inertial_mass: super::time::OverMassUnit,
}

impl IndividualRepresentation for MassNormalizedWithForceField {
    fn read_intrinsics(&self) -> &IntrinsicPart {
        self.particle_description.read_intrinsics()
    }

    fn read_variables(&self) -> &VariablePart {
        self.particle_description.read_variables()
    }
}

impl IndividualInForceField for MassNormalizedWithForceField {
    fn into_individual_particle(&self) -> BasicIndividual {
        self.particle_description
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::force::DimensionfulVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::time::OverMassUnit {
        &self.timestep_over_inertial_mass
    }

    fn write_particle_variables<'a>(&'a mut self) -> &'a mut VariablePart {
        &mut self.particle_description.variable_values
    }

    fn write_experienced_force<'a>(&'a mut self) -> &'a mut super::force::DimensionfulVector {
        &mut self.experienced_force
    }
}

impl<'a> IndexedCollectionInForceField<'a> for std::vec::Vec<MassNormalizedWithForceField> {
    type ImmutableIterator = std::slice::Iter<'a, MassNormalizedWithForceField>;
    type MutableIterator = std::slice::IterMut<'a, MassNormalizedWithForceField>;
    fn add_particle(
        &mut self,
        particle_to_add: &impl IndividualRepresentation,
        timestep_over_inertial_mass: &super::time::OverMassUnit,
    ) {
        self.push(MassNormalizedWithForceField {
            particle_description: create_individual_from_representation(particle_to_add),
            experienced_force: super::force::DimensionfulVector {
                horizontal_component: super::force::HorizontalUnit(0.0),
                vertical_component: super::force::VerticalUnit(0.0),
            },
            timestep_over_inertial_mass: *timestep_over_inertial_mass,
        })
    }

    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_immutable_iterator(&'a mut self) -> Self::ImmutableIterator {
        self.iter()
    }

    fn get_mutable_iterator(&'a mut self) -> Self::MutableIterator {
        self.iter_mut()
    }

    fn reset_forces(&mut self) {
        for particle_with_force in self.iter_mut() {
            let mut force_on_particle = particle_with_force.write_experienced_force();
            force_on_particle.horizontal_component = super::force::HorizontalUnit(0.0);
            force_on_particle.vertical_component = super::force::VerticalUnit(0.0);
        }
    }

    fn apply_pairwise_force(
        &mut self,
        index_for_adding_given_force: usize,
        index_for_subtracting_given_force: usize,
        force_vector: &super::force::DimensionfulVector,
    ) {
        *self[index_for_adding_given_force].write_experienced_force() += *force_vector;
        *self[index_for_subtracting_given_force].write_experienced_force() -= *force_vector;
    }
}
