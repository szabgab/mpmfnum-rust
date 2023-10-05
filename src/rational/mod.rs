//! Floating-point numbers with unbounded significand and exponent.
//! 
//! This module implements floating-point numbers with the
//! [`Rational`][crate::rational] type. The [`Rational`] type serves
//! as an interchange format between various number systems.
//! As the name suggests, [`Rational`] is really just rational numbers
//! encoded in scientific notation.
//! 

mod number;
mod ops;
mod round;

pub use number::Rational;
pub use number::{NAN, NEG_INF, POS_INF};
pub use round::RationalContext;
