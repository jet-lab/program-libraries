#[cfg(not(feature = "fixed-point"))]
mod functions;
#[cfg(not(feature = "fixed-point"))]
mod number;
#[cfg(not(feature = "fixed-point"))]
mod number_128;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "fixed-point")]
pub mod fixed_point;

#[doc(inline)]
pub use functions::*;

#[doc(inline)]
pub use number::*;

#[doc(inline)]
pub use number_128::*;
