use num_traits::Zero;
use rug::Integer;

use crate::rfloat::RFloat;
use crate::round::RoundingDirection;
use crate::{Real, RoundingContext, RoundingMode, Split};

/// Rounding contexts for floating-point numbers with
/// unbounded significand and unbounded exponent.
///
/// The associated storage type is [`RFloat`].
///
/// Values rounded under this context are base-2 numbers
/// in scientific notation `(-1)^s * c * 2^exp` where `c` is
/// a theoreticaly unbounded unsigned integer and the exponent
/// `exp` is an unbounded signed integer.
///
/// An [`RFloatContext`] takes three parameters:
///
///  - (optional) maximum precision (see [`Real::p`]),
///  - (optional) minimum absolute digit,
///  - and rounding mode [`RoundingMode`].
///
/// The requested precision may be as small as 1 binary digit.
/// There is no way to restrict the maximum value.
/// Infinity and NaN will not be rounded.
///
/// There are three possible rounding behaviors:
///  
///  - only `min_n` is specified,
///  - only `max_p` is specified,
///  - or both are specified.
///
/// In the first case, rounding will behave as with fixed-point numbers
/// with unbounded precision but the exponent `exp` must be more than `min_n`.
/// For example, if `min_n == 1`, then the rounded result will be an integer.
/// In the second case, the rounding will behave as with floating-point numbers,
/// adjusting `c` so that it has at most `max_p` bits.
/// In the third case, `min_n` takes precedence, so the result may have less
/// than `max_p` precision even if the input has at least `max_p` precision.
/// This behavior may be used to emulate IEEE 754 subnormalization.
///
/// At least one parameter must be given or rounding will panic.
/// The rounding mode affects how "lost" binary digits are handled.
/// The possible rounding modes that can be specified
/// are defined by [`RoundingMode`].
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
    pub fn with_max_p(mut self, max_p: usize) -> Self {
        assert!(max_p >= 1, "minimum precision must be at least 1");
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
    pub fn without_max_p(mut self) -> Self {
        self.max_p = None;
        self
    }

    /// Clears the minimum least absolute digit.
    pub fn without_min_n(mut self) -> Self {
        self.min_n = None;
        self
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
    /// by possibly incrementing the mantissa; the rounding decision
    /// is based on rounding mode and rounding bits.
    pub(crate) fn round_finalize(split: Split, rm: RoundingMode) -> RFloat {
        // truncated result
        let s = split.num().sign().unwrap();
        let mut exp = split.n() + 1;
        let mut c = match split.num().c() {
            Some(c) => c,
            None => Integer::zero(),
        };
    
        // rounding bits
        let (halfway_bit, sticky_bit) = split.rs();

        // correct if needed
        if Self::round_increment(s, &c, halfway_bit, sticky_bit, rm) {
            c += 1;
            match split.max_p() {
                None => (),
                Some(max_p) => {
                    let p = c.significant_bits() as usize;
                    if p > max_p {
                        // maximum precision exceeded so we shift one digit down
                        // and increment the exponent
                        c >>= 1;
                        exp += 1;
                    }
                }
            }
        }

        RFloat::Real(s, exp, c)
    }
}

impl Default for RFloatContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundingContext for RFloatContext {
    type Format = RFloat;

    fn round<T: Real>(&self, num: &T) -> Self::Format {
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
            if num.is_negative().unwrap() {
                RFloat::NegInfinity
            } else {
                RFloat::PosInfinity
            }
        } else if num.is_nar() {
            // other non-real
            RFloat::Nan
        } else {
            // finite, non-zero value

            // step 1: compute the first digit we will split off
            let (p, n) = self.round_params(num);

            // step 2: split the significand at binary digit `n`
            let split = Split::new(num, p, n);

            // step 3...: use the split to finish the rounding
            self.round_split(split)
        }
    }

    fn round_split(&self, split: Split) -> Self::Format {
        // step 3: finalize the rounding
        let rounded = Self::round_finalize(split, self.rm);

        // return the rounded number
        rounded.canonicalize()
    }
}
