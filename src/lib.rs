mod range;
mod slice_index;

pub use range::*;
pub use slice_index::*;

#[macro_export]
macro_rules! crange {
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

pub trait ConstGet<Idx> {
    type Output;
    fn cget(&self, index: Idx) -> Option<&Self::Output>;
    fn cget_mut(&mut self, index: Idx) -> Option<&mut Self::Output>;
}

impl<T, I> ConstGet<I> for [T]
where
    I: ConstSliceIndex<[T]>,
{
    type Output = I::Output;

    fn cget(&self, index: I) -> Option<&Self::Output> {
        index.get(self)
    }

    fn cget_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        index.get_mut(self)
    }
}
