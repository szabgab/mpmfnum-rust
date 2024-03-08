//! Exact arithmetic.
//!
//! This module implements exact arithmetic with [`RealContext`].
//! The associated storage type is [`RFloat`][crate::rfloat::RFloat]
//! from the [`rfloat`][crate::rfloat] crate.
//!
//! Only a small subset of mathematical operators can be implemented
//! for finite numbers. The rounding function for exact arithmetic is
//! just the identity function. If only we could actually compute with
//! real numbers...
//!

mod ops;
mod round;

pub use round::RealContext;
