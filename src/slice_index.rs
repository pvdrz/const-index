use core::ops::{Index, IndexMut};

use crate::range::{ConstRange, ConstRangeInclusive, ConstRangeTo, ConstRangeToInclusive};
use crate::usize::ConstUsize;

pub unsafe trait ConstSliceIndex<T: ?Sized> {
    type Output;

    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;
    unsafe fn get_unchecked_mut(self, slice: *mut T) -> *mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

unsafe impl<T, const N: usize> ConstSliceIndex<[T]> for ConstUsize<N> {
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

unsafe impl<T, const MIN: usize, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
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

unsafe impl<T, const MIN: usize, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
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

unsafe impl<T, const MAX: usize> ConstSliceIndex<[T]> for ConstRangeTo<MAX> {
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

unsafe impl<T, const MAX: usize, const LEN: usize> ConstSliceIndex<[T]>
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
