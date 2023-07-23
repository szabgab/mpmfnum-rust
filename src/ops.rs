// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ops.rs
//
// Traits for mathematical operations

use crate::{Number, RoundingContext};

/// Rounded addition.
pub trait RoundedAdd: RoundingContext {
    /// Adds two values of type `Self` exactly, rounding according
    /// to the rounding context `ctx`.
    fn add<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded;
}

/// Rounded multiplication.
pub trait RoundedMul: RoundingContext {
    /// Adds two values of type `Self` exactly, rounding according
    /// to the rounding context `ctx`.
    fn mul<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded;
}
