//! Exact arithmetic.
//!
//! Only a small subset of mathematical operators can be implemented
//! for finite numbers like [`RFloat`][crate::rfloat::RFloat].
//! The rounding function for exact arithmetic is just the identity
//! function. If only we could actually compute with real numbers...
//!

mod ops;
mod round;

pub use round::RealContext;
