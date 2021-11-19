mod range;
mod slice_index;
mod usize;

pub use crate::usize::*;
pub use range::*;
pub use slice_index::*;

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

pub trait ConstGet<T> {
    fn cget<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> Option<&Idx::Output>;
    fn cget_mut<Idx: ConstSliceIndex<[T]>>(&mut self, index: Idx) -> Option<&mut Idx::Output>;
    unsafe fn cget_unchecked<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output;
    unsafe fn cget_unchecked_mut<Idx: ConstSliceIndex<[T]>>(
        &mut self,
        index: Idx,
    ) -> &mut Idx::Output;
    fn cindex<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output;
    fn cindex_mut<Idx: ConstSliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output;
    fn csplit_at<const N: usize>(&self, index: ConstUsize<N>) -> (&[T; N], &Self);
    fn csplit_at_mut<const N: usize>(&mut self, index: ConstUsize<N>) -> (&mut [T; N], &mut Self);
}

impl<T> ConstGet<T> for [T] {
    #[inline]
    fn cget<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> Option<&Idx::Output> {
        index.get(self)
    }

    #[inline]
    fn cget_mut<Idx: ConstSliceIndex<[T]>>(&mut self, index: Idx) -> Option<&mut Idx::Output> {
        index.get_mut(self)
    }

    #[inline]
    unsafe fn cget_unchecked<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output {
        &*index.get_unchecked(self)
    }

    #[inline]
    unsafe fn cget_unchecked_mut<Idx: ConstSliceIndex<[T]>>(
        &mut self,
        index: Idx,
    ) -> &mut Idx::Output {
        &mut *index.get_unchecked_mut(self)
    }

    #[inline]
    fn cindex<Idx: ConstSliceIndex<[T]>>(&self, index: Idx) -> &Idx::Output {
        index.index(self)
    }

    #[inline]
    fn cindex_mut<Idx: ConstSliceIndex<[T]>>(&mut self, index: Idx) -> &mut Idx::Output {
        index.index_mut(self)
    }

    fn csplit_at<const N: usize>(&self, _index: ConstUsize<N>) -> (&[T; N], &Self) {
        let head = self.cindex(cindex!(..N));
        let tail = unsafe { &*self.get_unchecked(N..) };
        (head, tail)
    }

    fn csplit_at_mut<const N: usize>(&mut self, _index: ConstUsize<N>) -> (&mut [T; N], &mut Self) {
        assert!(N <= self.len());

        let head = unsafe { &mut *cindex!(..N).get_unchecked_mut(self) };
        let tail = unsafe { &mut *self.get_unchecked_mut(N..) };

        (head, tail)
    }
}
