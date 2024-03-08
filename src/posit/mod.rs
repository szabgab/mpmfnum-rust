//! "Posit" numbers as described in the 2022 Posit Standard.
//!
//! This module implements posits with [`PositContext`].
//! The associated storage type is [`Posit`] which represents
//! a posit number.

mod number;
pub mod ops;
mod round;

pub use number::Posit;
pub(crate) use number::PositVal;
pub use round::PositContext;
