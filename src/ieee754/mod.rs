// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/mod.rs
//
// Top-level of the rational module.
// Exports public functions
//

mod round;
mod types;

pub use types::{Exceptions, IEEE754};
