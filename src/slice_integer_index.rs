use core::ops::{RangeFrom, RangeTo};

use crate::{range::ConstRangeTo, slice_index::SliceIndex, usize::ConstUsize};

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<const N: usize> Sealed for ConstUsize<N> {}
    impl Sealed for usize {}
}

pub trait SliceIntegerIndex<T: ?Sized>: sealed::Sealed {
    type RangeTo: SliceIndex<T>;
    type RangeFrom: SliceIndex<T>;

    fn as_usize(self) -> usize;
    fn range_to(self) -> Self::RangeTo;
    fn range_from(self) -> Self::RangeFrom;
}

impl<T, const N: usize> SliceIntegerIndex<[T]> for ConstUsize<N> {
    type RangeTo = ConstRangeTo<N>;
    type RangeFrom = RangeFrom<usize>;

    fn as_usize(self) -> usize {
        N
    }

    fn range_to(self) -> Self::RangeTo {
        ConstRangeTo::new()
    }

    fn range_from(self) -> Self::RangeFrom {
        N..
    }
}

impl<T> SliceIntegerIndex<[T]> for usize {
    type RangeTo = RangeTo<usize>;
    type RangeFrom = RangeFrom<usize>;

    fn as_usize(self) -> usize {
        self
    }

    fn range_to(self) -> Self::RangeTo {
        ..self
    }

    fn range_from(self) -> Self::RangeFrom {
        self..
    }
}
