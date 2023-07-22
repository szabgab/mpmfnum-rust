// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// lib.rs
//
// Top-level file of the entire crate.
// Exports all public functions
//

pub mod ieee754;
pub mod number;
pub mod rational;
mod util;

pub use number::Number;
