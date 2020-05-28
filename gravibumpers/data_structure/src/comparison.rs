/// This checks each element in expected_set for any match in actual_set, where match is defined
/// as each of the data members having a difference less than the value of the data member in
/// tolerances_as_particle (absolute value). If any expected element is not matched, or there are
/// any actual elements which were not matched, an error will be returned. Because of the nature
/// of matchign wtihin a tolerance, if the tolerances are too large, some matches might happen
/// between wrong pairings, and the result might be a false negative.
pub fn unordered_within_tolerance(
    expected_set: &mut dyn std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    actual_set: &mut dyn std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    tolerances_as_particle: &super::IndividualParticle,
) -> Result<(), String> {
    Err(String::from("not yet implemented"))
}
