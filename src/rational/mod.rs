/*!
The "universal" digital number, a rational number
represented in (unnormalized) scientific notation.

This module implements unbounded digital numbers with the
[`Rational`][crate::rational] type. The [`Rational`] type serves as an
interchange format between various number systems.
*/

mod number;
mod ops;
mod round;

pub use number::{Rational, NAN, NEG_INF, POS_INF};
pub use round::Context;
