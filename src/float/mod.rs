/*!
Floating-point numbers with unbounded exponent.

A fixed-precision (sign-magnitude) integer scaled by a power of two.
This format more closely adheres to numbers in libraries like MPFR.

This module implements floating-point numbers with
the [`Float`][crate::float] storage type and the
[`FloatContext`][crate::float] rounding context type.

Unlike IEEE 754 floating-point numbers, the exponent is
theoretically unbounded (In practice, the exponent is stored
as a [`isize`] value, and MPFR limits constrain this exponent
further during computation. For IEEE 754 style floating-point numbers,
see the [`IEEE754`][crate::ieee754] crate
*/

mod number;
mod ops;
mod round;

pub use number::{Exceptions, Float};
pub use ops::*;
pub use round::FloatContext;
