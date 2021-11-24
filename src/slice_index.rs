use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

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

    impl Sealed for usize {}
    impl Sealed for Range<usize> {}
    impl Sealed for RangeFrom<usize> {}
    impl Sealed for RangeFull {}
    impl Sealed for RangeInclusive<usize> {}
    impl Sealed for RangeTo<usize> {}
    impl Sealed for RangeToInclusive<usize> {}
}

/// This trait is a line by line copy of [`SliceIndex`](core::slice::SliceIndex).
pub unsafe trait SliceIndex<T: ?Sized>: sealed::Sealed {
    type Output: ?Sized;

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

unsafe impl<T> SliceIndex<[T]> for usize {
    type Output = T;

    #[inline]
    fn get(self, slice: &[T]) -> Option<&T> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const T {
        (slice as *const T).add(self)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut T {
        (slice as *mut T).add(self)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &T {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        slice.index_mut(self)
    }
}

unsafe impl<T> SliceIndex<[T]> for Range<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe {
            core::ptr::slice_from_raw_parts(
                (slice as *const T).add(self.start),
                self.end - self.start,
            )
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe {
            core::ptr::slice_from_raw_parts_mut(
                (slice as *mut T).add(self.start),
                self.end - self.start,
            )
        }
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice.index_mut(self)
    }
}

unsafe impl<T> SliceIndex<[T]> for RangeFrom<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { (self.start..(*slice).len()).get_unchecked(slice) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { (self.start..(*slice).len()).get_unchecked_mut(slice) }
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice.index_mut(self)
    }
}

unsafe impl<T> SliceIndex<[T]> for RangeFull {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        slice
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        slice
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice.index_mut(self)
    }
}

fn into_slice_range(range: RangeInclusive<usize>) -> Range<usize> {
    let exclusive_end = *range.end() + 1;
    let start = if range.is_empty() {
        exclusive_end
    } else {
        *range.start()
    };
    start..exclusive_end
}

unsafe impl<T> SliceIndex<[T]> for RangeInclusive<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        // SAFETY: the caller has to uphold the safety contract for `get_unchecked`.
        unsafe { into_slice_range(self).get_unchecked(slice) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        // SAFETY: the caller has to uphold the safety contract for `get_unchecked_mut`.
        unsafe { into_slice_range(self).get_unchecked_mut(slice) }
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice.index_mut(self)
    }
}

unsafe impl<T> SliceIndex<[T]> for RangeTo<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        slice.get(self)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        slice.get_mut(self)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        // SAFETY: the caller has to uphold the safety contract for `get_unchecked`.
        unsafe { (0..self.end).get_unchecked(slice) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        // SAFETY: the caller has to uphold the safety contract for `get_unchecked_mut`.
        unsafe { (0..self.end).get_unchecked_mut(slice) }
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice.index(self)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice.index_mut(self)
    }
}
