/*!
The "universal" digital number, a rational number
represented in (unnormalized) scientific notation.

This module implements unbounded digital numbers with the
[`Rational`][crate::rational] type. The [`Rational`] type serves as an
interchange format between various number systems.
*/

mod ops;
mod round;
mod types;

pub use round::Context;
pub use types::Rational;
pub use types::{NAN, NEG_INF, POS_INF};
