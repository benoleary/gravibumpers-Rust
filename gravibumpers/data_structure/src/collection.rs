/// This module provides traits and implementations for some specialized collections.

/// This trait should allow functions over single elements and over all pairs, and also
/// offer a means of collecting a transformation into an owning iterator.
pub trait SingleAndPairwiseFinite {
    type MutableElement;

    fn get_count(&self) -> usize;

    fn apply_to_every_single<T>(&mut self, update_single: &mut T)
    where
        T: FnMut(&mut Self::MutableElement) -> ();

    fn apply_to_every_pair<IntermediateResult, ReadOnlyDerive, FirstMutate, SecondMutate>(
        &mut self,
        derive_change: &mut ReadOnlyDerive,
        apply_to_first: &mut FirstMutate,
        apply_to_second: &mut SecondMutate,
    ) where
        IntermediateResult: Sized,
        ReadOnlyDerive: FnMut(&Self::MutableElement, &Self::MutableElement) -> IntermediateResult,
        FirstMutate: FnMut(&mut Self::MutableElement, &IntermediateResult) -> (),
        SecondMutate: FnMut(&mut Self::MutableElement, &IntermediateResult) -> ();
}

impl<VectorElement> super::collection::SingleAndPairwiseFinite for std::vec::Vec<VectorElement> {
    type MutableElement = VectorElement;

    fn get_count(&self) -> usize {
        self.len()
    }

    fn apply_to_every_single<T>(&mut self, update_single: &mut T)
    where
        T: FnMut(&mut Self::MutableElement) -> (),
    {
        self.iter_mut().for_each(|mut single_element| {
            update_single(&mut single_element);
        });
    }

    fn apply_to_every_pair<IntermediateResult, ReadOnlyDerive, FirstMutate, SecondMutate>(
        &mut self,
        derive_change: &mut ReadOnlyDerive,
        apply_to_first: &mut FirstMutate,
        apply_to_second: &mut SecondMutate,
    ) where
        IntermediateResult: Sized,
        ReadOnlyDerive: FnMut(&Self::MutableElement, &Self::MutableElement) -> IntermediateResult,
        FirstMutate: FnMut(&mut Self::MutableElement, &IntermediateResult) -> (),
        SecondMutate: FnMut(&mut Self::MutableElement, &IntermediateResult) -> (),
    {
        let number_of_elements = self.len();
        for first_index in 0..(number_of_elements - 1) {
            for second_index in (first_index + 1)..number_of_elements {
                let intermediate_result = derive_change(&self[first_index], &self[second_index]);
                apply_to_first(&mut self[first_index], &intermediate_result);
                apply_to_second(&mut self[second_index], &intermediate_result);
            }
        }
    }
}
