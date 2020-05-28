pub fn unordered_within_tolerance(
    expected_set: &mut dyn std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    actual_set: &mut dyn std::iter::ExactSizeIterator<Item = super::IndividualParticle>,
    tolerances_as_particle: &super::IndividualParticle,
) -> Result<(), String> {
    Err(String::from("not yet implemented"))
}
