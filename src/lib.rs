// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// lib.rs
//
// Top-level file of the entire crate.
// Exports all public functions
//

/*!
    `mpmfnum` is a numbers library supporting numerous number formats,
    precisions, and rounding modes, hence "Multi-Precision, Multi-Format"
    (MPMF). Unlike other number libraries like
    [BigInt](https://docs.rs/num-bigint/latest/num_bigint/) or
    [Rug](https://docs.rs/rug/latest/rug/) (MPFR), this library
    emphasizes a clean abstraction of various computer number systems
    rather than high-performance for arbitrary-precision numbers.

    This is the API documentation.

    The library defines a universal trait for all number types [`Number`],
    and a universal trait for "rounding contexts" [`RoundingContext`].
    Supported number systems include fixed-width rational numbers,
    integers, fixed-point numbers, IEEE 754 floating-point numbers,
    and more.
*/

/// Floating-point numbers as described by the IEEE 754 standard.
pub mod ieee754;

/// "Universal" number trait.
pub mod number;

/// Arithmetic and more.
pub mod ops;

/// Fixed-width rational numbers.
pub mod rational;

/// Rounding "context" trait
pub mod round;

mod util;

pub use number::Number;
pub use round::RoundingContext;
