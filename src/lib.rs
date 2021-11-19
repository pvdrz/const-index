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
    fn csplit_at<const N: usize>(&self, index: ConstUsize<N>) -> (&[T; N], &Self);
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

    fn csplit_at<const N: usize>(&self, _index: ConstUsize<N>) -> (&[T; N], &Self) {
        let head = self.cget(cindex!(..N)).expect("Index out of bounds");
        let tail = unsafe { &*self.get_unchecked(N..) };
        (head, tail)
    }
}
