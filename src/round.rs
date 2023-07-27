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
    type Rounded: Number;

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

/// Rounding modes for rounding contexts.
///
/// The following enumeration encodes a list of rounding modes to handle
/// correcting the mantissa when losing binary digits due to rounding.
/// These modes are general enough to implement rounding for other number
/// formats like IEEE 754 floating-point.
///
/// The IEEE 754 standard specifies five rounding modes:
///
/// - two "nearest" modes:
///   - `roundTiesToEven`: rounds to the nearest representable value.
///      In this case there is a tie, round to the representable value whose
///      mantissa has a least significant bit of 0
///      ([`RoundingMode::NearestTiesToEven`]).
///   - `roundTiesToAway`: rounds to the nearest representable value.
///      In this case there is a tie, round to the representable value with
///      greater magnitude ([`RoundingMode::NearestTiesAwayZero`]).
/// - three directed modes:
///   - `roundTowardPositive`: rounds to the closest representable value
///     in the direction of positive infinity ([`RoundingMode::ToPositive`]).
///   - `roundTowardNegative`: rounds to the closest representable value
///     in the direction of negative infinity ([`RoundingMode::ToNegative]).
///   - `roundTowardZero`: rounds to the closest representable value
///     in the direction of zero ([`RoundingMode::ToZero`]).
///
/// Three additional rounding modes are provided including:
/// - [`RoundingMode::AwayZero`]: rounds to the closest representable value
///    away from zero, towards the nearest infinity.
/// - [`RoundingMode::ToEven`]`: rounds to the closest representable value
///    whose mantissa has a least significant bit of 0.
/// - [`RoundingMode::ToOdd`]`: rounds to the closest representable value
///    whose mantissa has a least significant bit of 1.
///
/// The rounding behavior of zero, infinite, and not-numerical values will be
/// unaffected by rounding mode.
///
#[derive(Clone, Copy, Debug)]
pub enum RoundingMode {
    NearestTiesToEven,
    NearestTiesAwayZero,
    ToPositive,
    ToNegative,
    ToZero,
    AwayZero,
    ToEven,
    ToOdd,
}

/// Rounding direction rather than rounding _mode_.
/// Given the sign of an unrounded number and a rounding mode,
/// we can transform the rounding mode into a rounding direction
/// and a boolean indicating if the direction should only be used
/// for tie-breaking.
#[derive(Clone, Debug)]
pub(crate) enum RoundingDirection {
    ToZero,
    AwayZero,
    ToEven,
    ToOdd,
}

impl RoundingMode {
    /// Converts a rounding mode and sign into a rounding direction
    /// and a boolean indication if the direction is for tie-breaking only.
    pub(crate) fn to_direction(self, sign: bool) -> (bool, RoundingDirection) {
        match (self, sign) {
            (RoundingMode::NearestTiesToEven, _) => (true, RoundingDirection::ToEven),
            (RoundingMode::NearestTiesAwayZero, _) => (true, RoundingDirection::AwayZero),
            (RoundingMode::ToPositive, false) => (false, RoundingDirection::AwayZero),
            (RoundingMode::ToPositive, true) => (false, RoundingDirection::ToZero),
            (RoundingMode::ToNegative, false) => (false, RoundingDirection::ToZero),
            (RoundingMode::ToNegative, true) => (false, RoundingDirection::AwayZero),
            (RoundingMode::ToZero, _) => (false, RoundingDirection::ToZero),
            (RoundingMode::AwayZero, _) => (false, RoundingDirection::AwayZero),
            (RoundingMode::ToEven, _) => (false, RoundingDirection::ToEven),
            (RoundingMode::ToOdd, _) => (false, RoundingDirection::ToOdd),
        }
    }
}
