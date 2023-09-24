/*!
The "universal" floating-point number, a fixed-precision
(sign-magnitude) integer scaled by a power of two.

This module implements general base-2 floating-point numbers
with the [`Float`][crate::float] storage type and the
[`FloatContext`][crate::float] rounding context type.
The [`Float`] type serves as an interchange format
between various number systems.

Unlike IEEE-754 floating-point numbers, the exponent is
theoretically unbounded (In practice, the exponent is stored
as a [`isize`] value, and MPFR limits constrain this exponent
further during computation. For IEEE-754 style floating-point numbers,
see the [`IEEE754`][crate::ieee754] crate
*/

mod number;
mod ops;
mod round;

pub use number::Float;
pub use round::FloatContext;
pub use number::{NAN, NEG_INF, POS_INF};
