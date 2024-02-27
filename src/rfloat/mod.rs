//! Floating-point numbers with unbounded significand and exponent.
//!
//! This module implements floating-point numbers with [`RFloatContext`].
//! The associated storage type is [`RFloat`] which represents
//! binary floating-point numbers with unbounded significand and
//! unbounded exponent.
//!
//! The [`RFloat`] type serves as an interchange format between
//! binary numbers since it is the least restrictive.
//!

mod number;
mod round;

pub use number::RFloat;
pub use number::{NAN, NEG_INF, POS_INF};
pub use round::RFloatContext;
