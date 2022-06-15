use anchor_lang::{error, error_code, prelude::Result};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

use crate::{Number, U192};

#[error_code]
pub enum NumericalError {
    #[msg("overflow on checked add")]
    AdditionOverflow,
    #[msg("overflow on checked add")]
    MultiplicationOverflow,
    #[msg("underflow on checked sub")]
    SubtractionUnderflow,
    #[msg("division by zero")]
    ZeroDivision,
}

pub trait TryAddAssign: CheckedAdd {
    fn try_add_assign(&mut self, amount: Self) -> Result<()> {
        *self = self
            .checked_add(&amount)
            .ok_or_else(|| error!(NumericalError::AdditionOverflow))?;
        Ok(())
    }
}

pub trait TrySubAssign: CheckedSub {
    fn try_sub_assign(&mut self, amount: Self) -> Result<()> {
        *self = self
            .checked_sub(&amount)
            .ok_or_else(|| error!(NumericalError::SubtractionUnderflow))?;
        Ok(())
    }
}
impl<T: CheckedAdd> TryAddAssign for T {}
impl<T: CheckedSub> TrySubAssign for T {}

pub trait ToNumber {
    fn to_number(self) -> Number;
}

impl<T: Into<U192>> ToNumber for T {
    fn to_number(self) -> Number {
        self.into().into()
    }
}

pub trait NumberAddAssign {
    fn try_number_add_assign(&mut self, amount: impl Into<Number>) -> Result<()>;
}

impl NumberAddAssign for [u8; 24] {
    fn try_number_add_assign(&mut self, amount: impl Into<Number>) -> Result<()> {
        *self = Number::from_bits(*self)
            .checked_add(&amount.into())
            .ok_or_else(|| error!(NumericalError::AdditionOverflow))?
            .into();

        Ok(())
    }
}

pub trait NumberSubAssign {
    fn try_number_sub_assign(&mut self, amount: impl Into<Number>) -> Result<()>;
}

impl NumberSubAssign for [u8; 24] {
    fn try_number_sub_assign(&mut self, amount: impl Into<Number>) -> Result<()> {
        *self = Number::from_bits(*self)
            .checked_sub(&amount.into())
            .ok_or_else(|| error!(NumericalError::SubtractionUnderflow))?
            .into();

        Ok(())
    }
}

pub trait SafeAdd: CheckedAdd {
    fn safe_add(&self, amount: Self) -> Result<Self> {
        self.checked_add(&amount)
            .ok_or_else(|| error!(NumericalError::AdditionOverflow))
    }
}
pub trait SafeDiv: CheckedDiv {
    fn safe_div(&self, amount: Self) -> Result<Self> {
        self.checked_div(&amount)
            .ok_or_else(|| error!(NumericalError::ZeroDivision))
    }
}
pub trait SafeMul: CheckedMul {
    fn safe_mul(&self, amount: Self) -> Result<Self> {
        self.checked_mul(&amount)
            .ok_or_else(|| error!(NumericalError::MultiplicationOverflow))
    }
}
pub trait SafeSub: CheckedSub {
    fn safe_sub(&self, amount: Self) -> Result<Self> {
        self.checked_sub(&amount)
            .ok_or_else(|| error!(NumericalError::SubtractionUnderflow))
    }
}

impl<T: CheckedAdd> SafeAdd for T {}
impl<T: CheckedDiv> SafeDiv for T {}
impl<T: CheckedMul> SafeMul for T {}
impl<T: CheckedSub> SafeSub for T {}
