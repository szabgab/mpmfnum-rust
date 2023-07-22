// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// number.rs
//
// Number trait
//

#![allow(unused_imports)]

use gmp::mpz::*;

use crate::RoundingContext;

/// The "digital" number representing a (projective) real number format.
///
/// All computer number systems share some characteristics.
/// They all can be represented by a finite-precision number in
/// scientific notation: `(-1)^s * c * b^exp` where `s` is the sign,
/// `c` is the integer significand, `b` is the radix, and `exp` is
/// the exponent. Specifically, `s` is either `0` or `1`, `c` is
/// non-negative, and `b` is positive. Number systems can usually be
/// split into two broad groups: floating-point or fixed-point, where
/// the "point" refers to the position of the "ones" place within `c`, if
/// `c` were extended to an infinite sequence of digits in either direction.
/// Number systems may encode non-real numbers, notably infinity or NaN.
///
/// See [`RoundingContext`] for details on rounding.
///
pub trait Number {
    /// Returns the radix of a number.
    /// It must be strictly positive.
    fn radix() -> usize;

    /// Returns true if the number's sign bit is true.
    /// For number formats with no notion of sign bit, the result
    /// will always be false.
    fn sign(&self) -> bool;

    /// Viewing this number as `(-1)^s * c * b^exp` where `c` is an integer,
    /// returns `exp`. Only well-defined for finite, non-zero numbers.
    fn exp(&self) -> Option<isize>;

    /// Viewing this number as `(-1)^s * f * b^e` where `f` is a binary
    /// fraction between 1 and 2, returns the exponent `e`. This is the
    /// preferred IEEE 754 interpretation of an exponent. Only well-defined
    /// for finite, non-zero numbers.
    fn e(&self) -> Option<isize>;

    /// The "least absolute exponent", the place below the least
    /// significant digit of the mantissa. Always equal to `self.exp() - 1`.
    /// For integer formats, this is just -1. Only well-defined for finite,
    /// non-zero numbers.
    fn n(&self) -> Option<isize>;

    /// Viewing this number as `(-1)^s * c * b^exp` where `c` is an integer,
    /// returns `c`. Only well-defined for finite, non-zero numbers.
    fn c(&self) -> Option<Mpz>;

    /// Viewing this number as `(-1)^s * c * b^exp` where `c` is an integer,
    /// returns `(-1)^s * c`, the signed significand. Only well-defined for
    /// finite, non-zero numbers.
    fn m(&self) -> Option<Mpz>;

    /// Precision of the significand.
    /// This is just `floor(logb(c))` where `b` is the radix and `c` is
    /// the integer significand. For binary formats (`b == 2`), this
    /// is just the number of bits required to encode `c`. For values that
    /// do not encode numbers, intervals, or even limiting behavior,
    /// the result is 0.
    fn p(&self) -> usize;

    /// Returns true if this number is not a real number.
    /// Example: NaN or +/-Inf from the IEEE 754 standard.
    fn is_nar(&self) -> bool;

    /// Returns true if this number is finite.
    /// For values that do not encode numbers, intervals, or even limiting
    /// behavior, the result is false.
    fn is_finite(&self) -> bool;

    /// Returns true if this number if infinite.
    /// For values that do not encode numbers, intervals, or even limiting
    /// behavior, the result is false.
    fn is_infinite(&self) -> bool;

    /// Returns true if this number is zero.
    fn is_zero(&self) -> bool;

    /// Returns true if this number is negative.
    /// This is not always well-defined, so the result is an Option.
    /// This is not necessarily the same as the sign bit (the IEEE 754
    /// standard differentiates between -0.0 and +0.0).
    fn is_negative(&self) -> Option<bool>;

    /// Returns true if this number represents a numerical value:
    /// either a finite number, interval, or some limiting value.
    fn is_numerical(&self) -> bool;
}
