use core::ops::{Range, RangeInclusive, RangeTo, RangeToInclusive};

pub struct ConstRange<const MIN: usize, const MAX: usize, const LEN: usize>();

impl<const MIN: usize, const MAX: usize, const LEN: usize> ConstRange<MIN, MAX, LEN> {
    #[inline]
    pub fn new() -> Self {
        assert_eq!(LEN, MAX - MIN);
        Self()
    }

    #[inline]
    pub(crate) const fn range(&self) -> Range<usize> {
        MIN..MAX
    }
}

impl<const MIN: usize, const MAX: usize, const LEN: usize> Default for ConstRange<MIN, MAX, LEN> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstRangeInclusive<const MIN: usize, const MAX: usize, const LEN: usize>();

impl<const MIN: usize, const MAX: usize, const LEN: usize> ConstRangeInclusive<MIN, MAX, LEN> {
    #[inline]
    pub fn new() -> Self {
        assert_eq!(LEN, MAX - MIN + 1);
        Self()
    }

    #[inline]
    pub(crate) const fn range(&self) -> RangeInclusive<usize> {
        MIN..=MAX
    }
}

impl<const MIN: usize, const MAX: usize, const LEN: usize> Default
    for ConstRangeInclusive<MIN, MAX, LEN>
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstRangeTo<const MAX: usize>();

impl<const MAX: usize> ConstRangeTo<MAX> {
    #[inline]
    pub fn new() -> Self {
        Self()
    }

    #[inline]
    pub(crate) const fn range(&self) -> RangeTo<usize> {
        ..MAX
    }
}

impl<const MAX: usize> Default for ConstRangeTo<MAX> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstRangeToInclusive<const MAX: usize, const LEN: usize>();

impl<const MAX: usize, const LEN: usize> ConstRangeToInclusive<MAX, LEN> {
    #[inline]
    pub fn new() -> Self {
        assert_eq!(MAX + 1, LEN);
        Self()
    }

    #[inline]
    pub(crate) const fn range(&self) -> RangeToInclusive<usize> {
        ..=MAX
    }
}

impl<const MAX: usize, const LEN: usize> Default for ConstRangeToInclusive<MAX, LEN> {
    fn default() -> Self {
        Self::new()
    }
}
