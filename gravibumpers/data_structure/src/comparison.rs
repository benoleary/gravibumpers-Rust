/// This module exists to provide helper functions to some tests, so has no #[cfg(test)] of its
/// own.

/// This checks each element in expected_set for any match in actual_set, where match is defined
/// as each of the data members having a difference less than the value of the data member in
/// tolerances_as_particle (absolute value). If any expected element is not matched, or there are
/// any actual elements which were not matched, an error will be returned. Because of the nature
/// of matching within a tolerance, if the tolerances are too large, some matches might happen
/// between wrong pairings, and the result might be a false negative.
pub fn unordered_particles_match_within_tolerance(
    expected_set: &mut impl std::iter::ExactSizeIterator<Item = impl super::ParticleRepresentation>,
    actual_set: impl std::iter::ExactSizeIterator<Item = impl super::ParticleRepresentation>,
    tolerances_as_particle: &impl super::ParticleRepresentation,
) -> Result<(), String> {
    let expected_length = expected_set.len();
    if actual_set.len() != expected_length {
        return Err(String::from(format!(
            "Expected length {}, actual length {}",
            expected_length,
            actual_set.len()
        )));
    }

    if expected_length == 0 {
        return Ok(());
    }

    let mut unmatched_expecteds: std::vec::Vec<super::IndividualParticle> =
        std::vec::Vec::with_capacity(expected_length);

    let first_expected = expected_set
        .next()
        .expect("Expected length was {} which should be > 0 yet there was no first element");

    let mut previous_unmatched_length = expected_length;

    let mut unmatched_actuals =
        list_unmatched_particles(&first_expected, actual_set, tolerances_as_particle);

    // If there was a match, we expect 1 less actual to come back from the above function.
    if unmatched_actuals.len() == previous_unmatched_length {
        unmatched_expecteds.push(super::create_individual_from_representation(
            &first_expected,
        ));
    } else {
        previous_unmatched_length = unmatched_actuals.len();
    }

    // We loop over the remaining expecteds using the vector of unmatched actuals from the previous
    // iteration. We could not do this for the first expected because Rust will not let us.
    for expected_particle in expected_set {
        unmatched_actuals = list_unmatched_particles(
            &expected_particle,
            unmatched_actuals.into_iter(),
            tolerances_as_particle,
        );

        // If there was a match, we expect 1 less actual to come back from the above function.
        if unmatched_actuals.len() == previous_unmatched_length {
            unmatched_expecteds.push(super::create_individual_from_representation(
                &expected_particle,
            ));
        } else {
            previous_unmatched_length = unmatched_actuals.len();
        }
    }

    if (unmatched_expecteds.len() != 0) || (unmatched_actuals.len() != 0) {
        Err(String::from(format!(
            "Unmatched expecteds = {:?}, unmatched actuals = {:?}",
            unmatched_expecteds, unmatched_actuals,
        )))
    } else {
        Ok(())
    }
}

pub fn ordered_sequences_match_unordered_particles_within_tolerance(
    expected_sequence: impl std::iter::ExactSizeIterator<
        Item = impl std::iter::ExactSizeIterator<Item = impl super::ParticleRepresentation>,
    >,
    actual_sequence: impl std::iter::ExactSizeIterator<
        Item = impl std::iter::ExactSizeIterator<Item = impl super::ParticleRepresentation>,
    >,
    tolerances_as_particle: &impl super::ParticleRepresentation,
) -> Result<(), String> {
    let number_of_time_slices = actual_sequence.len();
    if expected_sequence.len() != number_of_time_slices {
        return Err(String::from(format!(
            "Lengths of sequences did not match: expected {}, actual {}",
            expected_sequence.len(),
            number_of_time_slices,
        )));
    }

    let mut mismatched_time_slice_messages: std::vec::Vec<String> = vec![];

    for (sequence_index, (mut expected_set, actual_set)) in
        expected_sequence.zip(actual_sequence).enumerate()
    {
        let result_for_time_slice = unordered_particles_match_within_tolerance(
            &mut expected_set,
            actual_set,
            tolerances_as_particle,
        );

        if result_for_time_slice.is_err() {
            mismatched_time_slice_messages.push(String::from(format!(
                "Time slice {} did not match: {}",
                sequence_index,
                result_for_time_slice.unwrap_err(),
            )));
        }
    }

    if mismatched_time_slice_messages.is_empty() {
        Ok(())
    } else {
        Err(String::from(format!(
            "Mismatch: {:?}",
            mismatched_time_slice_messages,
        )))
    }
}

fn list_unmatched_particles(
    expected_particle: &impl super::ParticleRepresentation,
    unmatched_actuals: impl std::iter::ExactSizeIterator<Item = impl super::ParticleRepresentation>,
    tolerances_as_particle: &impl super::ParticleRepresentation,
) -> std::vec::Vec<super::IndividualParticle> {
    let mut found_match = false;
    let mut returned_unmatcheds: std::vec::Vec<super::IndividualParticle> =
        std::vec::Vec::with_capacity(unmatched_actuals.len());
    for unmatched_actual in unmatched_actuals {
        if !found_match
            && particle_within_tolerance(
                expected_particle,
                &unmatched_actual,
                tolerances_as_particle,
            )
        {
            found_match = true;
        } else {
            returned_unmatcheds.push(super::create_individual_from_representation(
                &unmatched_actual,
            ));
        }
    }

    returned_unmatcheds
}

fn particle_within_tolerance(
    expected_particle: &impl super::ParticleRepresentation,
    actual_particle: &impl super::ParticleRepresentation,
    tolerances_as_particle: &impl super::ParticleRepresentation,
) -> bool {
    intrinsics_within_tolerance(
        expected_particle.read_intrinsics(),
        &actual_particle.read_intrinsics(),
        &tolerances_as_particle.read_intrinsics(),
    ) && variables_within_tolerance(
        &expected_particle.read_variables(),
        &actual_particle.read_variables(),
        &tolerances_as_particle.read_variables(),
    )
}

fn intrinsics_within_tolerance(
    expected_intrinsics: &super::ParticleIntrinsics,
    actual_intrinsics: &super::ParticleIntrinsics,
    tolerances_as_intrinsics: &super::ParticleIntrinsics,
) -> bool {
    (expected_intrinsics.inertial_mass.0 - actual_intrinsics.inertial_mass.0).abs()
        < tolerances_as_intrinsics.inertial_mass.0.abs()
        && (expected_intrinsics.inverse_squared_charge.0
            - actual_intrinsics.inverse_squared_charge.0)
            .abs()
            <= tolerances_as_intrinsics.inverse_squared_charge.0.abs()
        && (expected_intrinsics.inverse_squared_charge.0
            - actual_intrinsics.inverse_squared_charge.0)
            .abs()
            <= tolerances_as_intrinsics.inverse_squared_charge.0.abs()
        && (expected_intrinsics.inverse_fourth_charge.0 - actual_intrinsics.inverse_fourth_charge.0)
            .abs()
            <= tolerances_as_intrinsics.inverse_fourth_charge.0.abs()
        && (expected_intrinsics.color_brightness.get_red().0
            - actual_intrinsics.color_brightness.get_red().0)
            .abs()
            <= tolerances_as_intrinsics.color_brightness.get_red().0.abs()
        && (expected_intrinsics.color_brightness.get_green().0
            - actual_intrinsics.color_brightness.get_green().0)
            .abs()
            <= tolerances_as_intrinsics
                .color_brightness
                .get_green()
                .0
                .abs()
        && (expected_intrinsics.color_brightness.get_blue().0
            - actual_intrinsics.color_brightness.get_blue().0)
            .abs()
            <= tolerances_as_intrinsics.color_brightness.get_blue().0.abs()
}

fn variables_within_tolerance(
    expected_variables: &super::ParticleVariables,
    actual_variables: &super::ParticleVariables,
    tolerances_as_variables: &super::ParticleVariables,
) -> bool {
    positions_within_tolerance(
        &expected_variables.position_vector,
        &actual_variables.position_vector,
        &tolerances_as_variables.position_vector,
    ) && velocities_within_tolerance(
        &expected_variables.velocity_vector,
        &actual_variables.velocity_vector,
        &tolerances_as_variables.velocity_vector,
    )
}

fn positions_within_tolerance(
    expected_vector: &super::PositionVector,
    actual_vector: &super::PositionVector,
    tolerances_as_vector: &super::PositionVector,
) -> bool {
    (expected_vector.horizontal_component.0 - actual_vector.horizontal_component.0).abs()
        < tolerances_as_vector.horizontal_component.0.abs()
        && (expected_vector.vertical_component.0 - actual_vector.vertical_component.0).abs()
            <= tolerances_as_vector.vertical_component.0.abs()
}

fn velocities_within_tolerance(
    expected_vector: &super::VelocityVector,
    actual_vector: &super::VelocityVector,
    tolerances_as_vector: &super::VelocityVector,
) -> bool {
    (expected_vector.horizontal_component.0 - actual_vector.horizontal_component.0).abs()
        < tolerances_as_vector.horizontal_component.0.abs()
        && (expected_vector.vertical_component.0 - actual_vector.vertical_component.0).abs()
            <= tolerances_as_vector.vertical_component.0.abs()
}
