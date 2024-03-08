//! Fixed-precision, floating-pont numbers with unbounded exponent.
//!
//! This module implements floating-point numbers with [`FloatContext`].
//! The associated storage type is [`Float`] which represents a
//! floating-point numbers with fixed-precision significand and
//! unounded exponent.
//!
//! Unlike IEEE 754 floating-point numbers,
//! the exponent is theoretically unbounded (in practice, the exponent
//! is an [`isize`] value and MPFR limits constrain this exponent further
//! during computation).
//!
//! For IEEE 754 style floating-point numbers,
//! see the [`IEEE754`][crate::ieee754] crate.

mod number;
pub mod ops;
mod round;

pub use number::{Exceptions, Float};
pub use round::FloatContext;
