// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ops.rs
//
// Traits for mathematical operations

use crate::{Number, RoundingContext};

/// Rounding addition.
// pub trait RoundedAdd<C: RoundingContext>: Number {
//     //
// }

/// Rounding multiplication.
pub trait RoundedMul<C: RoundingContext>: Number {
    /// Multiplies two values of type `Self` exactly, rounding according
    /// to the rounding context `ctx`.
    fn mul(&self, other: &Self, ctx: &C) -> C::Rounded;
}
