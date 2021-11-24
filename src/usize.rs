#[derive(Clone, Copy)]
pub struct ConstUsize<const N: usize>();

impl<const N: usize> ConstUsize<N> {
    #[inline]
    pub fn new() -> Self {
        Self()
    }

    #[inline]
    pub(crate) const fn value(&self) -> usize {
        N
    }
}

impl<const N: usize> Default for ConstUsize<N> {
    fn default() -> Self {
        Self::new()
    }
}
