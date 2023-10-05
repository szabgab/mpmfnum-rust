/*!
Common rounding utilities.

This module supports rounding contexts, implementations of rounding
from arbitrary-precision numbers to a particular number format.
*/

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
/// The characteristics of the rounding operation are summarized in a
/// "rounding context". All mathematical evaluation is done under
/// a particular rounding context.
///
/// See [`Number`] for details on the number trait.
///
pub trait RoundingContext {
    /// Result type of operations under this context.
    type Rounded: Number;

    /// Rounds a [`RoundingContext::Rounded`] value to another
    /// [`RoundingContext::Rounded`] value according to this context.
    ///
    /// See [`RoundingContext::mpmf_round`] for a more general implementation
    /// of rounding from formats other than the output format.
    fn round(&self, val: &Self::Rounded) -> Self::Rounded;

    /// Converts any [`Number`] to a [`RoundingContext::Rounded`] value,
    /// rounding the argument according to this context.
    ///
    /// Implementation note:
    /// This is the canonical rounding function, taking any value
    /// satisfying `Number` and rounding it to type `Rounded`.
    /// Implemenations of this trait may want to implement more complicated
    /// "round" function that also return information such as an error term,
    /// lost digits, etc. In this case, the implementation of `round` may
    /// just be a wrapper, discarding the extra information.
    fn mpmf_round<T: Number>(&self, val: &T) -> Self::Rounded;
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
///      ([`NearestTiesToEven`][RoundingMode]).
///   - `roundTiesToAway`: rounds to the nearest representable value.
///      In this case there is a tie, round to the representable value with
///      greater magnitude ([`NearestTiesAwayZero`][RoundingMode]).
/// - three directed modes:
///   - `roundTowardPositive`: rounds to the closest representable value
///     in the direction of positive infinity ([`ToPositive`][RoundingMode]).
///   - `roundTowardNegative`: rounds to the closest representable value
///     in the direction of negative infinity ([`ToNegative`][RoundingMode]).
///   - `roundTowardZero`: rounds to the closest representable value
///     in the direction of zero ([`ToZero`][RoundingMode]).
///
/// Three additional rounding modes are provided including:
/// - [`AwayZero`][RoundingMode]: rounds to the closest representable value
///    away from zero, towards the nearest infinity.
/// - [`ToEven`][RoundingMode]: rounds to the closest representable value
///    whose mantissa has a least significant bit of 0.
/// - [`ToOdd`][RoundingMode]: rounds to the closest representable value
///    whose mantissa has a least significant bit of 1.
///
/// The rounding behavior of zero, infinite, and non-numerical values will be
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
/// and a boolean indicating if the direction should onlybe used
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
