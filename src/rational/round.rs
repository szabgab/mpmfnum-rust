// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/round.rs
//
// Rounding for the rational type

use gmp::mpz::*;

use crate::number::Number;
use crate::rational::Rational;

/// Rounding modes for [`Context`].
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

/// Rounding contexts for rational numbers.
///
/// Rounding a digital number to a fixed-width rational number takes three
/// parameters: a maximum precision (see [`Number::p`]) and the minimum least
/// absolute digit (see [`Number::n`]), and a rounding mode [`RoundingMode`].
/// Rounding will theoretically work for all real values. The requested
/// precision may be one or zero bits, but there is no way to place an
/// upper bound on the resulting exponent; infinity and NaN will not be
/// rounded.
///
/// There are three possible rounding behaviors: only `min_n` is specified,
/// only `max_p` is specified, or both are specified. In the first case,
/// rounding will behave as with fixed-point numbers with unbounded precision
/// but the exponent `exp` must be more than `min_n`. For example, if
/// `min_n == 1`, then the rounded result will be an integer. In the second
/// case, the rounding will behave as with floating-point numbers, adjusting
/// `c` so that it has at most `max_p` bits. In the third case, `min_n` takes
/// precedence, so the result may have less than `max_p` precision even if
/// the input has at least `max_p` precision. This behavior may be used to
/// emulate IEEE 754 subnormalization. At least one parameter must be given
/// or rounding will panic.
///
/// The rounding mode affects how "lost" binary digits are handled. The
/// possible rounding modes that can be specified are defined by
/// [`RoundingMode`].
///
pub struct Context {
    max_p: Option<usize>,
    min_n: Option<Mpz>,
    rm: RoundingMode,
}

impl Context {
    /// Constructs a rounding arguments with default arguments.
    /// Neither `max_p` nor `min_n` are specified so rounding
    /// will panic. The default rounding mode is
    /// [`RoundingMode::NearestTiesToEven`].
    pub fn new() -> Self {
        Self {
            max_p: None,
            min_n: None,
            rm: RoundingMode::NearestTiesToEven,
        }
    }

    /// Sets the maximum allowable precision.
    pub fn with_max_precision(mut self, max_p: usize) -> Self {
        self.max_p = Some(max_p);
        self
    }

    /// Sets the minimum least absolute digit.
    pub fn with_min_n(mut self, min_n: Mpz) -> Self {
        self.min_n = Some(min_n);
        self
    }

    /// Sets the rounding mode.
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Rounds a [`Number`] type to a [`Rational`].
    pub fn round<T: Number>(&self, num: T) -> Rational {
        assert!(
            self.max_p.is_some() || self.min_n.is_some(),
            "must specify either maximum precision or least absolute digit"
        );

        // case split by class
        if num.is_zero() {
            // zero
            return Rational::zero();
        } else if num.is_infinite() {
            // infinite number
            return Rational::Infinite(num.is_negative().unwrap());
        } else if num.is_nar() {
            // other non-real
            return Rational::Nan;
        } else {
            // finite, non-zero value



            todo!()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
