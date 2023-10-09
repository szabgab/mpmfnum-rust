//! Floating-point numbers as described in the IEEE 754-2019 standard.
//!
//! This module implements floating-point numbers with [`IEEE754Context`].
//! The associated storage type is [`IEEE754`] which represents an
//! IEEE 754 style floating-point number.

mod number;
mod ops;
mod round;

pub(crate) use number::IEEE754Val;
pub use number::{Exceptions, IEEE754};
pub use ops::*;
pub use round::IEEE754Context;
