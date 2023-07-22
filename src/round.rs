// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// round.rs
//
// Rounding trait

use crate::Number;

/// Rounding context.
///
/// Most mathematical operators on digital numbers can be decomposed
/// into two steps: first, a mathematically-correct operation over
/// real numbers, interpreting digital numbers as real numbers; second,
/// a rounding operation to limit the number significant digits and decide
/// how the "lost" digits will affect the final output. Thus, rounding
/// enforces a particular "format" for digital numbers, but they should
/// just be considered unbounded real numbers when in isolation.
/// The characteristics of the rounding operation may be summarized as
/// in a "rounding context". All mathematicaly evaluation is done under
/// a particular rounding context.
///
/// See [`Number`] for details on the number trait.
///
pub trait RoundingContext {
    /// The result of rounded operations under this context.
    type Rounded;

    /// Converts any [`Number`] to [`RoundingContext::Rounded`], rounding
    /// the argument according to this context.
    ///
    /// Implementation note:
    /// This is the canonical rounding function, taking any value
    /// satisfying `Number` and rounding it to type `Rounded`.
    /// Implemenations of this trait may want to implement more complicated
    /// "round" function that also return information such as an error term,
    /// lost digits, etc.
    /// In this case, the implementation of `round` is just
    /// wrapper, discarding the extra information.
    fn round<T: Number>(&self, num: &T) -> Self::Rounded;
}
