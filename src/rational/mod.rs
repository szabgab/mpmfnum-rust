// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/mod.rs
//
// Top-level of the rational module.
// Exports public functions

mod types;

pub use types::Rational;
pub use types::{NAN, NEG_INF, POS_INF};