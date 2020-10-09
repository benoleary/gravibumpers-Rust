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
pub trait ReadOnlyInForceField: IndividualRepresentation + Sized {
    fn into_individual_particle(&self) -> BasicIndividual;
    fn read_experienced_force(&self) -> &super::force::DimensionfulVector;
    fn read_timestep_over_inertial_mass(&self) -> &super::time::OverMassUnit;
}

pub trait WritableInForceField: ReadOnlyInForceField {
    fn write_particle_variables(&mut self) -> &mut VariablePart;
    fn write_experienced_force(&mut self) -> &mut super::force::DimensionfulVector;
}

/// The trait should have a consistent order of iteration.
pub trait IndexedCollectionInForceFieldWithoutAdd:
    std::ops::Index<usize> + std::ops::IndexMut<usize> + Sized
where
    <Self as std::ops::Index<usize>>::Output: ReadOnlyInForceField,
{
    type MutableElement: WritableInForceField;
    fn get_length(&self) -> usize;
    fn update_particles<T>(&mut self, update_particle: T)
    where
        T: Fn(&mut Self::MutableElement) -> ();
    fn apply_pairwise_force(
        &mut self,
        index_for_adding_given_force: usize,
        index_for_subtracting_given_force: usize,
        force_vector: &super::force::DimensionfulVector,
    );
    fn create_time_slice_copy_without_force(&self) -> std::vec::Vec<BasicIndividual>;
}

pub trait IndexedCollectionInForceField: IndexedCollectionInForceFieldWithoutAdd
where
    <Self as std::ops::Index<usize>>::Output: ReadOnlyInForceField,
{
    fn add_particle(
        &mut self,
        particle_to_add: &impl IndividualRepresentation,
        timestep_over_inertial_mass: &super::time::OverMassUnit,
    );
}

pub trait IndexedCollectionInForceFieldGenerator<T: WritableInForceField> {
    type CreatedCollection: IndexedCollectionInForceField<Output = T, MutableElement = T>;

    fn create_collection(&self) -> Self::CreatedCollection;
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

impl ReadOnlyInForceField for MassNormalizedWithForceField {
    fn into_individual_particle(&self) -> BasicIndividual {
        self.particle_description
    }

    fn read_experienced_force<'a>(&'a self) -> &'a super::force::DimensionfulVector {
        &self.experienced_force
    }

    fn read_timestep_over_inertial_mass<'a>(&'a self) -> &'a super::time::OverMassUnit {
        &self.timestep_over_inertial_mass
    }
}

impl WritableInForceField for MassNormalizedWithForceField {
    fn write_particle_variables<'a>(&'a mut self) -> &'a mut VariablePart {
        &mut self.particle_description.variable_values
    }

    fn write_experienced_force<'a>(&'a mut self) -> &'a mut super::force::DimensionfulVector {
        &mut self.experienced_force
    }
}

impl<T> IndexedCollectionInForceFieldWithoutAdd for std::vec::Vec<T>
where
    T: WritableInForceField,
{
    type MutableElement = T;

    fn get_length(&self) -> usize {
        self.len()
    }

    fn update_particles<U>(&mut self, update_particle: U)
    where
        U: Fn(&mut Self::MutableElement) -> (),
    {
        for particle_with_force in self.iter_mut() {
            update_particle(particle_with_force);
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

    fn create_time_slice_copy_without_force(&self) -> std::vec::Vec<BasicIndividual> {
        self.iter()
            .map(|particle_with_force| particle_with_force.into_individual_particle())
            .collect::<std::vec::Vec<BasicIndividual>>()
    }
}

impl IndexedCollectionInForceField for std::vec::Vec<MassNormalizedWithForceField> {
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
}

pub struct MassNormalizedWithForceFieldVectorGenerator {}

impl IndexedCollectionInForceFieldGenerator<MassNormalizedWithForceField>
    for MassNormalizedWithForceFieldVectorGenerator
{
    type CreatedCollection = std::vec::Vec<MassNormalizedWithForceField>;

    fn create_collection(&self) -> Self::CreatedCollection {
        vec![]
    }
}
