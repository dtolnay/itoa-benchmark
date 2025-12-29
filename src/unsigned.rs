use rand::distr::uniform::SampleUniform;
use std::ops::Sub;

pub(crate) trait Unsigned:
    Copy + SampleUniform + From<bool> + Sub<Output = Self> + PartialOrd
{
    const ONE: Self;
    const TEN: Self;
    const MAX: Self;
    fn saturating_pow(self, exp: u32) -> Self;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
}

macro_rules! impl_unsigned {
    ($T:ty) => {
        impl Unsigned for $T {
            const ONE: Self = 1;
            const TEN: Self = 10;
            const MAX: Self = Self::MAX;
            fn saturating_pow(self, exp: u32) -> Self {
                self.saturating_pow(exp)
            }
            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }
        }
    };
}

impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
