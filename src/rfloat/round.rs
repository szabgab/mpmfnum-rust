use rug::Integer;

use crate::rfloat::RFloat;
use crate::round::RoundingDirection;
use crate::util::*;
use crate::{Real, RoundingContext, RoundingMode};

/// Result type of [`RFloatContext::round_prepare`].
pub(crate) struct RoundPrepareResult {
    pub num: RFloat,
    pub halfway_bit: bool,
    pub sticky_bit: bool,
}

/// Rounding contexts for floating-point numbers.
///
/// Rounding a number to a (bounded) rational number takes three parameters:
/// a maximum precision (see [`Real::p`]), the minimum absolute digit
/// (see [`Real::n`]), and a rounding mode [`RoundingMode`].
/// Rounding will theoretically work for all real values.
/// The requested precision may be as small as one or zero bits,
/// but there is no way to place an upper bound on the resulting exponent;
/// infinity and NaN will not be rounded.
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
/// The rounding mode affects how "lost" binary digits are handled.
/// The possible rounding modes that can be specified are
/// defined by [`RoundingMode`].
///
#[derive(Clone, Debug)]
pub struct RFloatContext {
    max_p: Option<usize>,
    min_n: Option<isize>,
    rm: RoundingMode,
}

impl RFloatContext {
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

    /// Splits a [`Real`] at binary digit `n`, returning two [`RFloat`] values:
    ///
    ///  - all significant digits above position `n`
    ///  - all significant digits at or below position `n`
    ///
    /// The sum of the resulting values will be exactly the input
    /// number, that is, it "splits" a number.
    pub fn split_at<T: Real>(num: &T, n: isize) -> (RFloat, RFloat) {
        if num.is_nar() {
            panic!("must be real {:?}", num);
        } else if num.is_zero() {
            let s = num.sign();
            let high = RFloat::Real(s, n + 1, Integer::from(0));
            let low = RFloat::Real(s, n, Integer::from(0));
            (high, low)
        } else {
            // number components
            let s = num.sign();
            let e = num.e().unwrap();
            let exp = num.exp().unwrap();
            let c = num.c().unwrap();

            // case split by split point offset
            if n >= e {
                // split point is above the significant digits
                let high = RFloat::Real(s, n + 1, Integer::from(0));
                let low = RFloat::Real(s, exp, c);
                (high, low)
            } else if n < exp {
                // split point is below the significant digits
                let high = RFloat::Real(s, exp, c);
                let low = RFloat::Real(s, n, Integer::from(0));
                (high, low)
            } else {
                // split point is within the significant digits
                let offset = n - (exp - 1);
                let mask = bitmask(offset as usize);
                let c_high = c.clone() >> offset;
                let c_low = c & mask;

                let high = RFloat::Real(s, n + 1, c_high);
                let low = RFloat::Real(s, exp, c_low);
                (high, low)
            }
        }
    }

    /// Rounding parameters necessary to complete rounding under
    /// this context for a given [`Real`]: the maximum precision `p` allowed
    /// and the minimum absolute digit `n`.
    pub fn round_params<T: Real>(&self, num: &T) -> (Option<usize>, isize) {
        match (self.max_p, self.min_n) {
            (None, None) => {
                // unreachable
                panic!(
                    "at least one rounding parameter must be specified: max_p={:?}, min_n={:?}",
                    self.max_p, self.min_n
                );
            }
            (None, Some(min_n)) => {
                // fixed-point rounding:
                // limited by n, precision is unbounded
                (None, min_n)
            }
            (Some(max_p), None) => {
                // floating-point rounding:
                // limited by precision, exponent is unbounded
                match num.e() {
                    // finite, non-zero => find the first lost digit
                    Some(e) => (Some(max_p), e - (max_p as isize)),
                    // zero or non-real => produce something reasonable
                    None => (Some(max_p), 0),
                }
            }
            (Some(max_p), Some(min_n)) => {
                // floating-point rounding with subnormalization:
                // limited by precision or exponent
                match num.e() {
                    // finite, non-zero => find the first lost digit
                    Some(e) => {
                        let unbounded_n = e - (max_p as isize);
                        let n = std::cmp::max(min_n, unbounded_n);
                        (Some(max_p), n)
                    }
                    // zero or non-real => produce something reasonable
                    None => (Some(max_p), 0),
                }
            }
        }
    }

    /// Rounding utility function: splits a [`Real`] at binary digit `n`,
    /// returning the digits above that position as a [`RFloat`] number,
    /// the next digit at the `n`th position (also called the guard bit),
    /// and an inexact bit if there are any lower order digits (also called
    /// the sticky bit).
    pub(crate) fn round_prepare<T: Real>(num: &T, n: isize) -> RoundPrepareResult {
        // split number at the `n`th digit
        let (high, low) = Self::split_at(num, n);

        // split the lower part at the `n-1`th digit
        let (half, low) = Self::split_at(&low, n - 1);

        // compute the rounding bits
        let halfway_bit = half.get_bit(n);
        let sticky_bit = !low.is_zero();

        // compose result
        RoundPrepareResult {
            num: high,
            halfway_bit,
            sticky_bit,
        }
    }

    /// Rounding utility function: given the truncated result and rounding
    /// bits, should the truncated result be incremented to produce
    /// the final rounded result?
    fn round_increment(
        sign: bool,
        c: &Integer,
        half_bit: bool,
        sticky_bit: bool,
        rm: RoundingMode,
    ) -> bool {
        let (is_nearest, rd) = rm.to_direction(sign);
        match (is_nearest, half_bit, sticky_bit, rd) {
            (_, false, false, _) => {
                // exact => truncate
                false
            }
            (true, false, _, _) => {
                // nearest, below the halfway point => truncate
                false
            }
            (true, true, true, _) => {
                // nearest, above the halfway point => increment
                true
            }
            (true, true, false, RoundingDirection::ToZero) => {
                // nearest, exactly halfway, ToZero => truncate
                false
            }
            (true, true, false, RoundingDirection::AwayZero) => {
                // nearest, exactly halfway, AwayZero => increment
                true
            }
            (true, true, false, RoundingDirection::ToEven) => {
                // nearest, exactly halfway, ToEven => increment if odd
                c.is_odd()
            }
            (true, true, false, RoundingDirection::ToOdd) => {
                // nearest, exactly halfway, ToOdd => increment if even
                c.is_even()
            }
            (false, _, _, RoundingDirection::ToZero) => {
                // directed, toZero => always truncate
                false
            }
            (false, _, _, RoundingDirection::AwayZero) => {
                // directed, alwaysZero => increment
                true
            }
            (false, _, _, RoundingDirection::ToEven) => {
                // directed, toEven => increment if odd
                c.is_odd()
            }
            (false, _, _, RoundingDirection::ToOdd) => {
                // directed, toOdd => increment if even
                c.is_even()
            }
        }
    }

    /// Rounding utility function: finishes the rounding procedure
    /// by possibly incrementing the mantissa; the decision is
    /// based on rounding mode and rounding bits.
    pub(crate) fn round_finalize(
        split: RoundPrepareResult,
        p: Option<usize>,
        rm: RoundingMode,
    ) -> RFloat {
        // truncated result
        let (sign, mut exp, mut c) = match split.num {
            RFloat::Real(s, exp, c) => (s, exp, c),
            _ => panic!("unreachable"),
        };

        // rounding bits
        let halfway_bit = split.halfway_bit;
        let sticky_bit = split.sticky_bit;

        // correct if needed
        if Self::round_increment(sign, &c, halfway_bit, sticky_bit, rm) {
            c += 1;
            if p.is_some() && c.significant_bits() as usize > p.unwrap() {
                c >>= 1;
                exp += 1;
            }
        }

        RFloat::Real(sign, exp, c)
    }

    /// Rounds a finite [`Real`].
    ///
    /// Called by the public [`Real::round`] function.
    fn round_finite<T: Real>(&self, num: &T) -> RFloat {
        // step 1: compute the first digit we will split off
        let (p, n) = self.round_params(num);

        // step 2: split the significand at binary digit `n`
        let split = Self::round_prepare(num, n);

        // step 3: finalize the rounding
        let rounded = Self::round_finalize(split, p, self.rm);

        // return the rounded number
        rounded.canonicalize()
    }
}

impl Default for RFloatContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundingContext for RFloatContext {
    type Rounded = RFloat;

    fn round<T: Real>(&self, num: &T) -> Self::Rounded {
        assert!(
            self.max_p.is_some() || self.min_n.is_some(),
            "must specify either maximum precision or least absolute digit"
        );

        // case split by class
        if num.is_zero() {
            // zero
            RFloat::zero()
        } else if num.is_infinite() {
            // infinite number
            let s = num.is_negative().unwrap();
            RFloat::Infinite(s)
        } else if num.is_nar() {
            // other non-real
            RFloat::Nan
        } else {
            // finite, non-zero value
            self.round_finite(num)
        }
    }
}
