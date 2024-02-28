use crate::Real;

/// Universal trait for rounding contexts.
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
/// See [`Real`] for details on the number trait.
///
pub trait RoundingContext {
    /// Result type of operations under this context.
    type Format: Real;

    /// Rounds any [`Real`] value to a [`RoundingContext::Format`] value,
    /// rounding according to this [`RoundingContext`].
    fn round<T: Real>(&self, val: &T) -> Self::Format;
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
///   - `roundTiesToEven` rounds to the nearest representable value.
///      In this case there is a tie, rounds to the closest representable value
///      whose mantissa has a least significant bit of 0
///      ([`NearestTiesToEven`][RoundingMode]).
///   - `roundTiesToAway` rounds to the nearest representable value.
///      In this case there is a tie, rounds to the closest representable value
///      with greater magnitude ([`NearestTiesAwayZero`][RoundingMode]).
/// - three directed modes:
///   - `roundTowardPositive` rounds to the closest representable value
///     in the direction of positive infinity ([`ToPositive`][RoundingMode]).
///   - `roundTowardNegative` rounds to the closest representable value
///     in the direction of negative infinity ([`ToNegative`][RoundingMode]).
///   - `roundTowardZero` rounds to the closest representable value
///     in the direction of zero ([`ToZero`][RoundingMode]).
///
/// Three additional rounding modes are provided including:
/// - [`AwayZero`][RoundingMode] rounds to the closest representable value
///    away from zero, towards the nearest infinity.
/// - [`ToEven`][RoundingMode] rounds to the closest representable value
///    whose mantissa has a least significant bit of 0.
/// - [`ToOdd`][RoundingMode] rounds to the closest representable value
///    whose mantissa has a least significant bit of 1.
///
/// The rounding behavior of zero, infinite values, and non-numerical values
/// will be unaffected by rounding mode.
///
#[derive(Clone, Copy, Debug)]
pub enum RoundingMode {
    /// Rounds to the nearest representable value.
    /// In this case there is a tie, rounds to the closest representable value
    /// whose mantissa has a least significant bit of 0.
    NearestTiesToEven,
    /// Rounds to the nearest representable value.
    /// In this case there is a tie, rounds to the closest representable value
    /// with greater (or same) magnitude.
    NearestTiesAwayZero,
    /// Rounds to the closest representable value in the direction
    /// of positive infinity.
    ToPositive,
    /// Rounds to the closest representable value in the direction
    /// of negative infinity.
    ToNegative,
    /// Rounds to the closest representable value with smaller
    /// (or same) magnitude.
    ToZero,
    /// Rounds to the closest representable value with greater
    /// (or same) magnitude.
    AwayZero,
    /// Rounds to the closest representable value whose mantissa has
    /// a least significant bit of 0.
    ToEven,
    /// Rounds to the closest representable value whose mantissa has
    /// a least significant bit of 1.
    ToOdd,
}

impl RoundingMode {
    /// Converts a rounding mode and sign into a rounding direction
    /// and a boolean indication if the direction is for tie-breaking only.
    pub fn to_direction(self, sign: bool) -> (bool, RoundingDirection) {
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

/// Directed rounding.
///
/// We can translate _sign_ of an unrounded number and a [`RoundingMode`],
/// into a [`RoundingDirection`] and a boolean indicating if the direction
/// should only be used for tie-breaking (see [`RoundingMode::to_direction`]).
/// It is usually easier to implement rounding using the latter pair of values.
#[derive(Clone, Debug)]
pub enum RoundingDirection {
    /// Rounds to the closest representable value with smaller
    /// (or same) magnitude.
    ToZero,
    /// Rounds to the closest representable value with greater
    /// (or same) magnitude.
    AwayZero,
    /// Rounds to the closest representable value whose mantissa has
    /// a least significant bit of 0.
    ToEven,
    /// Rounds to the closest representable value whose mantissa has
    /// a least significant bit of 1.
    ToOdd,
}
