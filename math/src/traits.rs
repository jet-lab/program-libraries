use anchor_lang::{error, error_code, prelude::Result};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

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
