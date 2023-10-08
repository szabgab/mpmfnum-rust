//! Number formats, precisions, and rounding modes.
//!
//! `mpmfnum` is a numbers library for emulation various number formats,
//! at multiple precisions with many rounding modes. Hence its name
//! "Multi-Precision, Multi-Format" (MPMF). Unlike other number libraries like
//! [BigInt](https://docs.rs/num-bigint/latest/num_bigint/) or
//! [Rug](https://docs.rs/rug/latest/rug/) (MPFR), this library emphasizes
//! a clean abstraction of various computer number systems rather than
//! high-performance computation.
//! 
//! This library embraces design principles found in the [FPCore](https://fpbench.org/) standard.
//! Most importantly, numerical programs need only be specified by
//! (i) real-number mathematical operations and (ii) rounding.
//! Number formats, e.g., `double` or `float`, are not first-class,
//! as in most programming languages.
//! Rather, they should be viewed side effects of rounding and should
//! be de-emphasized or eliminated entirely.
//! Furthermore, all values within a numerical program should be
//! viewed as (extended) real numbers.
//! 
//! These design principles are reflected in the two primary traits of this library:
//! 
//!  - [`Real`] is an extended real value;
//!  - [`RoundingContext`] is a rounding operation on [`Real`] values.
//! 
//! For implementation purposes, we restrict any [`Real`] value to be of
//! the form `(-1)^s * c * 2^exp` where `s` is 0 or 1, `c` is a
//! non-negative integer, and `exp` is an integer.
//! Implementations of  [`RoundingContext`] may support any of
//! the numerous operations (as traits) found under [`crate::ops`].
//! Operations provided by this library are correctly rounded.
//! 
//! `mpmfnum` supports various number systems through implementations of [`RoundingContext`]:
//!
//!  - [`RationalContext`][crate::rational::RationalContext]
//!     rounds a [`Real`] value to an arbitrary-precision, floating-point numbers
//!     with unbounded exponent
//!  - [`FloatContext`][crate::float::FloatContext]
//!     rounds a [`Real`] value to a fixed-precision, floating-point numbers
//!     with unbounded exponent
//!  - [`IEEE754Context`][crate::ieee754::IEEE754Context]
//!     rounds a [`Real`] value to a floating-point number as described by
//!     the IEEE 754 standard
//!  - [`FixedContext`][crate::fixed::FixedContext]
//!     rounds a [`Real`] value to a fixed-point
//!
//! Planned support or posits and more!
//!

pub mod fixed;
pub mod float;
pub mod ieee754;
pub mod math;
pub mod ops;
pub mod rational;
pub mod real;

mod number;
mod round;
mod util;

pub use crate::number::Real;
pub use crate::round::RoundingContext;
pub use crate::round::RoundingMode;
