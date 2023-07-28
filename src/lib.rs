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
Supported number systems include IEEE 754 floating-point numbers.
Planned number systems include fixed-point numbers, integers,
posits, logarithmic numbers, and more.
*/

pub mod ieee754;
pub mod number;
pub mod ops;
pub mod rational;
pub mod round;
mod util;

pub use crate::ieee754::IEEE754;
pub use crate::number::Number;
pub use crate::round::RoundingContext;
pub use crate::round::RoundingMode;
