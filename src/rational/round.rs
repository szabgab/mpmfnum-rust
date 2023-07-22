// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/round.rs
//
// Rounding for the rational type

use std::ops::BitAnd;

use gmp::mpz::*;

use crate::number::Number;
use crate::rational::Rational;
use crate::util::*;

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
#[derive(Clone, Debug)]
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
pub enum RoundingDirection {
    ToZero,
    AwayZero,
    ToEven,
    ToOdd,
}

impl RoundingMode {
    /// Converts a rounding mode and sign into a rounding direction
    /// and a boolean indication if the direction is for tie-breaking only.
    fn to_direction(&self, sign: bool) -> (bool, RoundingDirection) {
        match (&self, sign) {
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
#[derive(Clone, Debug)]
pub struct Context {
    max_p: Option<usize>,
    min_n: Option<isize>,
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
    pub fn with_min_n(mut self, min_n: isize) -> Self {
        self.min_n = Some(min_n);
        self
    }

    /// Sets the rounding mode.
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Clears the maximum allowable precision.
    pub fn without_max_precision(mut self) -> Self {
        self.max_p = None;
        self
    }

    /// Clears the minimum least absolute digit.
    pub fn without_min_n(mut self) -> Self {
        self.min_n = None;
        self
    }

    /// Rounds a [`Number`] type to a [`Rational`]. The function returns
    /// a pair: the actual rounding value, and an [`Option`] containing
    /// the lost binary digits encoded as a rational number if the rounded
    /// result was finite or [`None`] otherwise.
    pub fn round<T: Number>(&self, num: &T) -> (Rational, Option<Rational>) {
        assert!(
            self.max_p.is_some() || self.min_n.is_some(),
            "must specify either maximum precision or least absolute digit"
        );

        // case split by class
        if num.is_zero() {
            // zero
            (Rational::zero(), None)
        } else if num.is_infinite() {
            // infinite number
            let s = num.is_negative().unwrap();
            (Rational::Infinite(s), None)
        } else if num.is_nar() {
            // other non-real
            (Rational::Nan, None)
        } else {
            // finite, non-zero value

            // step 1: compute the first digit we will split off
            let p: Option<usize>;
            let n: isize;

            if self.max_p.is_none() {
                // fixed-point rounding:
                // limited by n, precision is unbounded
                p = None;
                n = self.min_n.unwrap();
            } else {
                // floating-point rounding:
                // limited by precision
                p = self.max_p;
                let unbounded_n = num.e().unwrap() - (p.unwrap() as isize);
                if self.min_n.is_some() {
                    // exponent is unbounded
                    n = unbounded_n;
                } else {
                    // exponent is not unbounded, so we may have subnormalization
                    // either limits by precision or smallest representable bit
                    n = std::cmp::max(self.min_n.unwrap(), unbounded_n);
                }
            }

            // step 2: split the significand

            // truncated result
            let sign = num.sign();
            let mut exp = num.exp().unwrap();
            let mut c = num.c().unwrap();

            // rounding bits
            let half_bit: bool;
            let sticky_bit: bool;
            let c_lost: Mpz;

            // the amount we need to shift by
            let offset = n - (exp - 1);
            match offset.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    // shifting off bits
                    let truncated = c.clone() >> (offset as usize);
                    c_lost = c.bitand(bitmask(offset as usize));
                    c = truncated;
                    exp += offset;
                    half_bit = c_lost.tstbit((offset - 1) as usize);
                    sticky_bit = !c_lost.bitand(bitmask((offset - 1) as usize)).is_zero();
                },
                std::cmp::Ordering::Equal => {
                    // keeping all the bits
                    half_bit = false;
                    sticky_bit = false;
                },
                std::cmp::Ordering::Less => {
                    // need to adding padding to the right,
                    // exactly -offset binary digits
                    c <<= -offset as usize;
                    exp += offset;
                    half_bit = false;
                    sticky_bit = false;
                }
            }

            // sanity check
            assert_eq!(exp, n + 1, "exponent not in the right place!");

            // step 3: rounding
            // need to decide if we should increment
            let (is_nearest, rd) = self.rm.to_direction(sign);
            let increment: bool;

            if is_nearest {
                // nearest, directed if halfway
                if !half_bit {
                    // below halfway, so we truncate
                    increment = false;
                } else if sticky_bit {
                    // above halfway, so we increment
                    increment = true;
                } else {
                    // exactly half way ([half, sticky] == '10')
                    match rd {
                        RoundingDirection::ToZero => {
                            // always truncate
                            increment = false;
                        },
                        RoundingDirection::AwayZero => {
                            // round away since not exact
                            increment = true;
                        },
                        RoundingDirection::ToEven => {
                            // round away if odd
                            increment = !is_even(exp, &c);
                        },
                        RoundingDirection::ToOdd => {
                            // round away if even
                            increment = is_even(exp, &c);
                        },
                    };
                }
            } else {
                // directed
                match rd {
                    RoundingDirection::ToZero => {
                        // always truncate
                        increment = false;
                    },
                    RoundingDirection::AwayZero => {
                        // round away if not exact
                        increment = half_bit || sticky_bit;
                    }
                    RoundingDirection::ToEven => {
                        // round away if odd
                        increment = !is_even(exp, &c);
                    }
                    RoundingDirection::ToOdd => {
                        // round away if even
                        increment = is_even(exp, &c);
                    }
                };
            }

            // step 4: apply the correction
            if increment {
                c += 1;
                if p.is_some() && c.bit_length() > p.unwrap() {
                    c >>= 1;
                    exp += 1;
                }
            }

            // step 5: compose result
            let rounded = Rational::Real(sign, exp, c);

            // TODO: summarize lost bits
            (rounded, None)
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
