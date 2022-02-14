use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

const PRECISION: i32 = 10;
const ONE: i128 = 10_000_000_000;

const POWERS_OF_TEN: &'static [i128] = &[
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
];

/// A fixed-point decimal number 128 bits wide
#[derive(Pod, Zeroable, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Number128(i128);

impl Number128 {
    pub const ONE: Self = Self(ONE);
    pub const ZERO: Self = Self(0);

    /// Convert this number to fit in a u64
    ///
    /// The precision of the number in the u64 is based on the
    /// exponent provided.
    pub fn as_u64(&self, exponent: impl Into<i32>) -> u64 {
        let extra_precision = PRECISION + exponent.into();
        let mut prec_value = POWERS_OF_TEN[extra_precision.abs() as usize];

        if extra_precision < 0 {
            prec_value = ONE / prec_value;
        }

        let target_value = self.0 / prec_value;
        if target_value > std::u64::MAX as i128 {
            panic!("cannot convert to u64 due to overflow");
        }

        if target_value < 0 {
            panic!("cannot convert to u64 because value < 0");
        }

        target_value as u64
    }

    /// Convert another integer
    pub fn from_decimal(value: impl Into<i128>, exponent: impl Into<i32>) -> Self {
        let extra_precision = PRECISION + exponent.into();
        let mut prec_value = POWERS_OF_TEN[extra_precision.abs() as usize];

        if extra_precision < 0 {
            prec_value = ONE / prec_value;
        }

        Self(value.into() * prec_value)
    }

    /// Convert from basis points
    pub fn from_bps(basis_points: u16) -> Self {
        Self::from_decimal(basis_points, crate::BPS_EXPONENT)
    }
}

impl std::fmt::Display for Number128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo optimize
        let rem = self.0 % ONE;
        let decimal_digits = PRECISION as usize;
        let rem_str = rem.to_string();
        // regular padding like {:010} doesn't work with U192
        let decimals = "0".repeat(decimal_digits - rem_str.len()) + &*rem_str;
        let stripped_decimals = decimals.trim_end_matches('0');
        let pretty_decimals = if stripped_decimals.is_empty() {
            "0"
        } else {
            stripped_decimals
        };
        if self.0 < ONE {
            write!(f, "0.{}", pretty_decimals)?;
        } else {
            let int = self.0 / ONE;
            write!(f, "{}.{}", int, pretty_decimals)?;
        }
        Ok(())
    }
}

impl Add<Number128> for Number128 {
    type Output = Self;

    fn add(self, rhs: Number128) -> Self::Output {
        Self(self.0.checked_add(rhs.0).unwrap())
    }
}

impl AddAssign<Number128> for Number128 {
    fn add_assign(&mut self, rhs: Number128) {
        self.0 = self.0.checked_add(rhs.0).unwrap();
    }
}

impl Sub<Number128> for Number128 {
    type Output = Self;

    fn sub(self, rhs: Number128) -> Self::Output {
        Self(self.0.checked_sub(rhs.0).unwrap())
    }
}

impl SubAssign<Number128> for Number128 {
    fn sub_assign(&mut self, rhs: Number128) {
        self.0 = self.0.checked_sub(rhs.0).unwrap();
    }
}

impl Mul<Number128> for Number128 {
    type Output = Number128;

    fn mul(self, rhs: Number128) -> Self::Output {
        Self(self.0.checked_mul(rhs.0).unwrap().div(ONE))
    }
}

impl MulAssign<Number128> for Number128 {
    fn mul_assign(&mut self, rhs: Number128) {
        self.0.mul_assign(rhs.0);
        self.0.div_assign(ONE);
    }
}

impl Div<Number128> for Number128 {
    type Output = Number128;

    fn div(self, rhs: Number128) -> Self::Output {
        Self(self.0.mul(ONE).div(rhs.0))
    }
}

impl DivAssign<Number128> for Number128 {
    fn div_assign(&mut self, rhs: Number128) {
        self.0.mul_assign(ONE);
        self.0.div_assign(rhs.0);
    }
}

impl<T: Into<i128>> Mul<T> for Number128 {
    type Output = Number128;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0.mul(rhs.into()))
    }
}

impl<T: Into<i128>> Div<T> for Number128 {
    type Output = Number128;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0.div(rhs.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_equals_zero() {
        assert_eq!(Number128::ZERO, Number128::from_decimal(0, 0));
    }

    #[test]
    fn one_equals_one() {
        assert_eq!(Number128::ONE, Number128::from_decimal(1, 0));
    }

    #[test]
    fn one_plus_one_equals_two() {
        assert_eq!(
            Number128::from_decimal(2, 0),
            Number128::ONE + Number128::ONE
        );
    }

    #[test]
    fn one_minus_one_equals_zero() {
        assert_eq!(Number128::ONE - Number128::ONE, Number128::ZERO);
    }

    #[test]
    fn one_times_one_equals_one() {
        assert_eq!(Number128::ONE, Number128::ONE * Number128::ONE);
    }

    #[test]
    fn one_divided_by_one_equals_one() {
        assert_eq!(Number128::ONE, Number128::ONE / Number128::ONE);
    }

    #[test]
    fn ten_div_100_equals_point_1() {
        assert_eq!(
            Number128::from_decimal(1, -1),
            Number128::from_decimal(1, 1) / Number128::from_decimal(100, 0)
        );
    }

    #[test]
    fn multiply_by_u64() {
        assert_eq!(
            Number128::from_decimal(3, 1),
            Number128::from_decimal(1, 1) * 3u64
        )
    }
}
