//! Multiple flavors of fixed-point numbers.
//!
//! This module implements fixed-point numbers with [`FixedContext`].
//! The associated storage type is [`Fixed`].

mod number;
mod ops;
mod round;

pub use number::{Exceptions, Fixed};
pub use round::{FixedContext, Overflow};
