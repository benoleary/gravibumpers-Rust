/// This module exists to provide helper functions to some tests, so has no #[cfg(test)] of its
/// own.

/// This checks each element in expected_set for any match in actual_set, where match is defined
/// as each of the data members having a difference less than the value of the data member in
/// tolerances_as_particle (absolute value). If any expected element is not matched, or there are
/// any actual elements which were not matched, an error will be returned. Because of the nature
/// of matching within a tolerance, if the tolerances are too large, some matches might happen
/// between wrong pairings, and the result might be a false negative.
pub fn unordered_within_tolerance(
    expected_set: &mut impl std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    actual_set: impl std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    tolerances_as_particle: &super::IndividualParticle,
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
        unmatched_expecteds.push(first_expected.clone());
    } else {
        previous_unmatched_length = unmatched_actuals.len();
    }

    // We loop over the remaining expecteds using the vector of unmatched actuals from the previous
    // iteration. We could not do this for the first expected because Rust will not let us.
    for expected_particle in expected_set {
        unmatched_actuals = list_unmatched_particles(
            &expected_particle,
            unmatched_actuals.iter().cloned(),
            tolerances_as_particle,
        );

        // If there was a match, we expect 1 less actual to come back from the above function.
        if unmatched_actuals.len() == previous_unmatched_length {
            unmatched_expecteds.push(expected_particle.clone());
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

fn list_unmatched_particles(
    expected_particle: &super::IndividualParticle,
    unmatched_actuals: impl std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    tolerances_as_particle: &super::IndividualParticle,
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
            returned_unmatcheds.push(unmatched_actual.clone());
        }
    }

    returned_unmatcheds
}

fn particle_within_tolerance(
    expected_particle: &super::IndividualParticle,
    actual_particle: &super::IndividualParticle,
    tolerances_as_particle: &super::IndividualParticle,
) -> bool {
    intrinsics_within_tolerance(
        &expected_particle.intrinsic_values,
        &actual_particle.intrinsic_values,
        &tolerances_as_particle.intrinsic_values,
    ) && variables_within_tolerance(
        &expected_particle.variable_values,
        &actual_particle.variable_values,
        &tolerances_as_particle.variable_values,
    )
}

fn intrinsics_within_tolerance(
    expected_intrinsics: &super::ParticleIntrinsics,
    actual_intrinsics: &super::ParticleIntrinsics,
    tolerances_as_intrinsics: &super::ParticleIntrinsics,
) -> bool {
    (expected_intrinsics.inertial_mass.0 - actual_intrinsics.inertial_mass.0).abs()
        < tolerances_as_intrinsics.inertial_mass.0.abs()
        && (expected_intrinsics.attractive_charge.0 - actual_intrinsics.attractive_charge.0).abs()
            <= tolerances_as_intrinsics.attractive_charge.0.abs()
        && (expected_intrinsics.attractive_charge.0 - actual_intrinsics.attractive_charge.0).abs()
            <= tolerances_as_intrinsics.attractive_charge.0.abs()
        && (expected_intrinsics.repulsive_charge.0 - actual_intrinsics.repulsive_charge.0).abs()
            <= tolerances_as_intrinsics.repulsive_charge.0.abs()
        && (expected_intrinsics.red_brightness.0 - actual_intrinsics.red_brightness.0).abs()
            <= tolerances_as_intrinsics.red_brightness.0.abs()
        && (expected_intrinsics.green_brightness.0 - actual_intrinsics.green_brightness.0).abs()
            <= tolerances_as_intrinsics.green_brightness.0.abs()
        && (expected_intrinsics.blue_brightness.0 - actual_intrinsics.blue_brightness.0).abs()
            <= tolerances_as_intrinsics.blue_brightness.0.abs()
}

fn variables_within_tolerance(
    expected_variables: &super::ParticleVariables,
    actual_variables: &super::ParticleVariables,
    tolerances_as_variables: &super::ParticleVariables,
) -> bool {
    (expected_variables.horizontal_position.0 - actual_variables.horizontal_position.0).abs()
        < tolerances_as_variables.horizontal_position.0.abs()
        && (expected_variables.vertical_position.0 - actual_variables.vertical_position.0).abs()
            <= tolerances_as_variables.vertical_position.0.abs()
        && (expected_variables.horizontal_velocity.0 - actual_variables.horizontal_velocity.0).abs()
            <= tolerances_as_variables.horizontal_velocity.0.abs()
        && (expected_variables.vertical_velocity.0 - actual_variables.vertical_velocity.0).abs()
            <= tolerances_as_variables.vertical_velocity.0.abs()
}
