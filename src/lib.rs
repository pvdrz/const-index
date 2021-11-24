mod range;
mod slice_index;
mod slice_integer_index;
mod usize;

pub use crate::usize::*;
pub use range::*;
pub use slice_index::*;
pub use slice_integer_index::*;

#[macro_export]
macro_rules! cindex {
    ($val:tt) => {
        $crate::ConstUsize::<$val>::new()
    };
    (..=$max:tt) => {
        $crate::ConstRangeToInclusive::<$max, { $max + 1 }>::new()
    };
    (..$max:tt) => {
        $crate::ConstRangeTo::<$max>::new()
    };
    ($min:tt..=$max:tt) => {
        #[allow(clippy::identity_op)]
        $crate::ConstRangeInclusive::<$min, $max, { $max - $min + 1 }>::new()
    };
    ($min:tt..$max:tt) => {
        #[allow(clippy::identity_op)]
        $crate::ConstRange::<$min, $max, { $max - $min }>::new()
    };
}

mod sealed {
    pub trait Sealed {}

    impl<T> Sealed for [T] {}
}

pub trait ConstGet<T>: sealed::Sealed {
    fn cget<Idx: SliceIndex<[T]>>(&self, index: Idx) -> Option<&Idx::Output>;
    fn cget_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> Option<&mut Idx::Output>;
    unsafe fn cget_unchecked<Idx: SliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output;
    unsafe fn cget_unchecked_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output;
    fn cindex<Idx: SliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output;
    fn cindex_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output;
    fn csplit_at<Idx: SliceIntegerIndex<[T]> + Copy>(
        &self,
        index: Idx,
    ) -> (
        &<Idx::RangeTo as SliceIndex<[T]>>::Output,
        &<Idx::RangeFrom as SliceIndex<[T]>>::Output,
    );
    fn csplit_at_mut<Idx: SliceIntegerIndex<[T]> + Copy>(
        &mut self,
        index: Idx,
    ) -> (
        &mut <Idx::RangeTo as SliceIndex<[T]>>::Output,
        &mut <Idx::RangeFrom as SliceIndex<[T]>>::Output,
    );
}

impl<T> ConstGet<T> for [T] {
    #[inline]
    fn cget<Idx: SliceIndex<[T]>>(&self, index: Idx) -> Option<&Idx::Output> {
        index.get(self)
    }

    #[inline]
    fn cget_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> Option<&mut Idx::Output> {
        index.get_mut(self)
    }

    #[inline]
    unsafe fn cget_unchecked<Idx: SliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output {
        &*index.get_unchecked(self)
    }

    #[inline]
    unsafe fn cget_unchecked_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output {
        &mut *index.get_unchecked_mut(self)
    }

    #[inline]
    fn cindex<Idx: SliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output {
        index.index(self)
    }

    #[inline]
    fn cindex_mut<Idx: SliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output {
        index.index_mut(self)
    }

    fn csplit_at<Idx: SliceIntegerIndex<[T]> + Copy>(
        &self,
        index: Idx,
    ) -> (
        &<Idx::RangeTo as SliceIndex<[T]>>::Output,
        &<Idx::RangeFrom as SliceIndex<[T]>>::Output,
    ) {
        let head = self.cindex(index.range_to());
        let tail = unsafe { self.cget_unchecked(index.range_from()) };

        (head, tail)
    }

    fn csplit_at_mut<Idx: SliceIntegerIndex<[T]> + Copy>(
        &mut self,
        index: Idx,
    ) -> (
        &mut <Idx::RangeTo as SliceIndex<[T]>>::Output,
        &mut <Idx::RangeFrom as SliceIndex<[T]>>::Output,
    ) {
        assert!(index.as_usize() <= self.len());

        let head = unsafe { &mut *index.range_to().get_unchecked_mut(self) };
        let tail = unsafe { self.cget_unchecked_mut(index.range_from()) };

        (head, tail)
    }
}
