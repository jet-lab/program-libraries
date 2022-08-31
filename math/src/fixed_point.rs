use std::ops::{Add, Div, Mul, Sub};

use bytemuck::{Pod, Zeroable};

pub const FP32_ONE: u128 = 1 << 32;

#[derive(Pod, Zeroable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fp32(u128);

impl Fp32 {
    pub const ONE: Self = Self(FP32_ONE);
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(u128::MAX);
    pub const MIN: Self = Self(u128::MIN);

    pub fn as_u64(self) -> Option<u64> {
        static BOUND: u128 = u64::MAX as u128;
        if self.0 > BOUND {
            None
        } else {
            Some(self.0 as u64)
        }
    }

    pub fn as_u64_ciel(&self) -> Option<u64> {
        let add_one = (!(self.0 as u32)).wrapping_add(1) as u128;
        self.0.checked_add(add_one).map(|n| n as u64)
    }

    pub fn as_u64_floor(&self) -> u64 {
        (self.0 >> 32) as u64
    }
}

impl<T: Into<u128>> From<T> for Fp32 {
    fn from(n: T) -> Fp32 {
        Fp32(n.into() * FP32_ONE)
    }
}

impl Add<Fp32> for Fp32 {
    type Output = Fp32;

    fn add(self, rhs: Fp32) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Sub<Fp32> for Fp32 {
    type Output = Fp32;

    fn sub(self, rhs: Fp32) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Mul<Fp32> for Fp32 {
    type Output = Fp32;

    fn mul(self, rhs: Fp32) -> Self::Output {
        Self(self.0.mul(rhs.0).div(FP32_ONE))
    }
}

impl Div<Fp32> for Fp32 {
    type Output = Fp32;

    fn div(self, rhs: Fp32) -> Self::Output {
        Self(self.0.mul(FP32_ONE).div(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_equals_zero() {
        assert_eq!(Fp32::ZERO, Fp32::from(0u64));
    }

    #[test]
    fn one_equals_one() {
        assert_eq!(Fp32::ONE, Fp32::from(1u64));
    }

    #[test]
    fn one_plus_one_equals_two() {
        assert_eq!(Fp32::from(2u64), Fp32::ONE + Fp32::ONE);
    }

    #[test]
    fn one_minus_one_equals_zero() {
        assert_eq!(Fp32::ONE - Fp32::from(1u64), Fp32::ZERO);
    }

    #[test]
    fn one_times_one_equals_one() {
        assert_eq!(Fp32::ONE, Fp32::ONE * Fp32::ONE);
        assert_eq!(Fp32::ONE, Fp32::from(1u64) * Fp32::ONE);
    }

    #[test]
    fn one_divided_by_one_equals_one() {
        assert_eq!(Fp32::ONE, Fp32::ONE / Fp32::ONE);
        assert_eq!(Fp32::ONE, Fp32::from(1u64) / Fp32::ONE);
    }
}
