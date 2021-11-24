use core::ops::{Index, IndexMut};

use crate::range::{ConstRange, ConstRangeInclusive, ConstRangeTo, ConstRangeToInclusive};
use crate::usize::ConstUsize;

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<const N: usize> Sealed for ConstUsize<N> {}
    impl<const MIN: usize, const MAX: usize, const LEN: usize> Sealed for ConstRange<MIN, MAX, LEN> {}
    impl<const MIN: usize, const MAX: usize, const LEN: usize> Sealed
        for ConstRangeInclusive<MIN, MAX, LEN>
    {
    }
    impl<const MAX: usize> Sealed for ConstRangeTo<MAX> {}
    impl<const MAX: usize, const LEN: usize> Sealed for ConstRangeToInclusive<MAX, LEN> {}
}

/// This trait is a line by line copy of [`SliceIndex`](core::slice::SliceIndex).
pub unsafe trait SliceIndex<T: ?Sized>: sealed::Sealed {
    type Output;

    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;
    unsafe fn get_unchecked_mut(self, slice: *mut T) -> *mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

unsafe impl<T, const N: usize> SliceIndex<[T]> for ConstUsize<N> {
    type Output = T;

    #[inline]
    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        slice.get(self.value())
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        slice.get_mut(self.value())
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        (slice as *const T).add(self.value())
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        (slice as *mut T).add(self.value())
    }

    #[inline]
    fn index(self, slice: &[T]) -> &Self::Output {
        slice.index(self.value())
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut Self::Output {
        slice.index_mut(self.value())
    }
}

unsafe impl<T, const MIN: usize, const MAX: usize, const LEN: usize> SliceIndex<[T]>
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

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        (slice as *const T).add(MIN) as *const _
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        (slice as *mut T).add(MIN) as *mut _
    }

    fn index(self, slice: &[T]) -> &Self::Output {
        slice.index(self.range()).try_into().unwrap()
    }

    fn index_mut(self, slice: &mut [T]) -> &mut Self::Output {
        slice.index_mut(self.range()).try_into().unwrap()
    }
}

unsafe impl<T, const MIN: usize, const MAX: usize, const LEN: usize> SliceIndex<[T]>
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

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        (slice as *const T).add(MIN) as *const _
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        (slice as *mut T).add(MIN) as *mut _
    }

    fn index(self, slice: &[T]) -> &Self::Output {
        slice.index(self.range()).try_into().unwrap()
    }

    fn index_mut(self, slice: &mut [T]) -> &mut Self::Output {
        slice.index_mut(self.range()).try_into().unwrap()
    }
}

unsafe impl<T, const MAX: usize> SliceIndex<[T]> for ConstRangeTo<MAX> {
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

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        slice as *const T as *const _
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        slice as *mut T as *mut _
    }

    fn index(self, slice: &[T]) -> &Self::Output {
        slice.index(self.range()).try_into().unwrap()
    }

    fn index_mut(self, slice: &mut [T]) -> &mut Self::Output {
        slice.index_mut(self.range()).try_into().unwrap()
    }
}

unsafe impl<T, const MAX: usize, const LEN: usize> SliceIndex<[T]>
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

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        slice as *const T as *const _
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        slice as *mut T as *mut _
    }

    fn index(self, slice: &[T]) -> &Self::Output {
        slice.index(self.range()).try_into().unwrap()
    }

    fn index_mut(self, slice: &mut [T]) -> &mut Self::Output {
        slice.index_mut(self.range()).try_into().unwrap()
    }
}
