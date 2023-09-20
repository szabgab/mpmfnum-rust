/*!
Multi-precision floating-point numbers as described by
the IEEE 754-2019 standard.

This module implements IEEE 754 floating-point numbers with the
[`IEEE754`][crate::ieee754] type and IEEE 754 rounding behavior with
the [`Context`][crate::ieee754] type.
*/

mod number;
mod ops;
mod round;

pub use number::{Exceptions, IEEE754Val, IEEE754};
pub use ops::*;
pub use round::Context;
