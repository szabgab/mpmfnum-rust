/*!
Multi-precision fixed-point numbers.

This module implements fixed-point numbers with the
[`Fixed`][crate::fixed] type.
*/

mod number;
mod ops;
mod round;

pub use number::{Exceptions, Fixed};
pub use round::{FixedContext, Overflow};
