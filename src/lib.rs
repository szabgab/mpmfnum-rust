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
//! The library defines a universal trait for all number types [`Number`],
//! and a universal trait for "rounding contexts" [`RoundingContext`].
//! Supported number systems include:
//!  - floating-point numbers with unbounded significands/exponents [`Rational`][crate::rational::Rational],
//!  - floating-point numbers with unbounded exponent [`Float`][crate::float::Float],
//!  - IEEE 754 style floating-point numbers [`IEEE754`][crate::ieee754::IEEE754],
//!  - fixed-point numbers [`Fixed`][crate::fixed::Fixed].
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
