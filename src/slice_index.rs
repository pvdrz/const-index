use crate::range::{ConstRange, ConstRangeInclusive, ConstRangeTo, ConstRangeToInclusive};

pub trait ConstSliceIndex<T: ?Sized> {
    type Output;

    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
}

impl<T, const MIN: usize, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
    for ConstRange<MIN, MAX, LEN>
{
    type Output = [T; LEN];

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        slice
            .get(self.range())
            .map(|slice| slice.try_into().unwrap())
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        slice
            .get_mut(self.range())
            .map(|slice| slice.try_into().unwrap())
    }
}

impl<T, const MIN: usize, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
    for ConstRangeInclusive<MIN, MAX, LEN>
{
    type Output = [T; LEN];

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        slice
            .get(self.range())
            .map(|slice| slice.try_into().unwrap())
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        slice
            .get_mut(self.range())
            .map(|slice| slice.try_into().unwrap())
    }
}

impl<T, const MAX: usize> ConstSliceIndex<[T]> for ConstRangeTo<MAX> {
    type Output = [T; MAX];

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        slice
            .get(self.range())
            .map(|slice| slice.try_into().unwrap())
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        slice
            .get_mut(self.range())
            .map(|slice| slice.try_into().unwrap())
    }
}

impl<T, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
    for ConstRangeToInclusive<MAX, LEN>
{
    type Output = [T; LEN];

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        slice
            .get(self.range())
            .map(|slice| slice.try_into().unwrap())
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        slice
            .get_mut(self.range())
            .map(|slice| slice.try_into().unwrap())
    }
}
