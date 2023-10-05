//! Number formats, precisions, and rounding modes.
//!
//! `mpmfnum` is a numbers library for emulation various number formats,
//! at multiple precisions with many rounding modes. Hence, its name,
//! "Multi-Precision, Multi-Format" (MPMF).
//!
//! Unlike other number libraries like
//! [BigInt](https://docs.rs/num-bigint/latest/num_bigint/) or
//! [Rug](https://docs.rs/rug/latest/rug/) (MPFR), this library emphasizes
//! a clean abstraction of various computer number systems rather than
//! high-performance computation.
//!
//! This is the API documentation.
//!
//! The library centers around two traits:
//!  - [`Real`] - extended real numbers
//!  - [`RoundingContext`] - descriptions of rounding behaviors
//!
//! Supported number systems include:
//!  - [`Rational`][crate::rational::Rational] -
//!     floating-point numbers with unbounded significand and exponent
//!  - [`Float`][crate::float::Float] -
//!     floating-point numbers with unbounded exponent
//!  - [`IEEE754`][crate::ieee754::IEEE754] -
//!     IEEE 754 style floating-point numbers
//!  - [`Fixed`][crate::fixed::Fixed] -
//!     fixed-point numbers
//!
//! Planned number systems include posits, logarithmic numbers, and more.
//!

pub mod fixed;
pub mod float;
pub mod ieee754;
pub mod math;
pub mod ops;
pub mod rational;

mod real;
mod round;
mod util;

pub use crate::real::Real;
pub use crate::round::RoundingContext;
pub use crate::round::RoundingMode;
