/*!
Multi-precision fixed-point numbers.

This module implements fixed-point numbers with the
[`Fixed`][crate::ieee754] type.
*/

mod round;
mod types;

pub use round::{Context, Overflow};
pub use types::{Exceptions, Fixed};
