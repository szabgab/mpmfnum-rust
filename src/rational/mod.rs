/*!
The "universal" digital number format:
a rational number represented in (unnormalized) scientific notation.

This module implements unbounded digital numbers with the
[`Rational`][crate::rational] type. The [`Rational`] type
serves as an interchange format between various number systems.
*/

mod number;
mod ops;
mod round;

pub use number::Rational;
pub use number::{NAN, NEG_INF, POS_INF};
pub use round::RationalContext;
