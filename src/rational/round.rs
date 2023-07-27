// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/round.rs
//
// Rounding for the rational type

use std::cmp::min;
use std::ops::BitAnd;

use num_traits::Zero;
use rug::Integer;

use crate::rational::Rational;
use crate::round::RoundingDirection;
use crate::util::*;
use crate::{Number, RoundingContext, RoundingMode};

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

    /// Rounding utility function: splits a [`Number`] at binary digit `n`,
    /// returning five values: the position of the least siginficant digit
    /// of `num` above `n`, the binary digits above the `n`th place,
    /// the binary digits at or below the `n`th place, and the two
    /// subsequent binary digits at the digit `n` and `n-1` (the halfway
    /// and sticky rounding bits).
    pub(crate) fn split<T: Number>(num: &T, n: isize) -> (isize, Integer, Integer, bool, bool) {
        // number components
        let exp = num.exp().unwrap();
        let c = num.c().unwrap();

        // shift amount
        let offset = n - (exp - 1);

        // compute the truncated result asnd lost binary digits
        match offset.cmp(&0) {
            std::cmp::Ordering::Greater => {
                // shifting off bits
                let max_lost = c.significant_bits() as usize;
                let exp = exp + offset;
                let truncated = c.clone() >> (offset as usize);
                let c_lost = c.bitand(bitmask(min(offset as usize, max_lost)));
                let half_bit = c_lost.get_bit((offset - 1) as u32);
                let sticky_bit = !c_lost
                    .clone()
                    .bitand(bitmask((offset - 1) as usize))
                    .is_zero();
                (exp, truncated, c_lost, half_bit, sticky_bit)
            }
            std::cmp::Ordering::Equal => {
                // keeping all the bits
                (exp, c, Integer::from(0), false, false)
            }
            std::cmp::Ordering::Less => {
                // need to adding padding to the right,
                // exactly -offset binary digits
                let exp = exp + offset;
                let c = c << -offset as usize;
                (exp, c, Integer::from(0), false, false)
            }
        }
    }

    /// Rounding utility function: given the truncated result and rounding
    /// bits, should the truncated result be incremented to produce
    /// the final rounded result?
    fn round_increment(&self, sign: bool, c: &Integer, half_bit: bool, sticky_bit: bool) -> bool {
        let (is_nearest, rd) = self.rm.to_direction(sign);
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

    /// Rounds a finite [`Number`].
    /// Called by the public [`Number::round`] function
    fn round_finite<T: Number>(&self, num: &T) -> (Rational, Option<Rational>) {
        // step 1: compute the first digit we will split off
        let (p, n) = match (self.max_p, self.min_n) {
            (None, None) => {
                // unreachable
                panic!("unreachable")
            }
            (None, Some(min_n)) => {
                // fixed-point rounding:
                // limited by n, precision is unbounded
                (None, min_n)
            }
            (Some(max_p), None) => {
                // floating-point rounding:
                // limited by precision, exponent is unbounded
                (Some(max_p), num.e().unwrap() - (max_p as isize))
            }
            (Some(max_p), Some(min_n)) => {
                // floating-point rounding:
                // limited by precision or exponent;
                // we may have subnormalization
                let unbounded_n = num.e().unwrap() - (max_p as isize);
                let n = std::cmp::max(min_n, unbounded_n);
                (Some(max_p), n)
            }
        };

        // step 2: split the significand at binary digit `n`
        let sign = num.sign();
        let (mut exp, mut c, c_lost, half_bit, sticky_bit) = Self::split(num, n);

        // sanity check
        assert_eq!(exp, n + 1, "exponent not in the right place!");

        // step 3: correct if needed
        // need to decide if we should increment
        if self.round_increment(sign, &c, half_bit, sticky_bit) {
            c += 1;
            if p.is_some() && c.significant_bits() as usize > p.unwrap() {
                c >>= 1;
                exp += 1;
            }
        }

        // step 4: compose result
        let rounded = Rational::Real(sign, exp, c);
        let exp_lost = num.n().unwrap() + 1;
        let lost = if rounded.is_zero() {
            // all bits lost and the result is rounded
            // so we give `c_lost` the "sign" of the result
            // which is just the sign of `num`.
            Rational::Real(sign, exp_lost, c_lost)
        } else {
            // some bits are lost, `lost` will be unsigned
            Rational::Real(false, exp_lost, c_lost)
        };

        // Returns the rounded number and the binary digits lost
        // as a sum
        (rounded.canonicalize(), Some(lost.canonicalize()))
    }

    /// Rounds a [`Number`] type to a [`Rational`]. The function returns
    /// a pair: the actual rounding value, and an [`Option`] containing
    /// the lost binary digits encoded as a rational number if the rounded
    /// result was finite or [`None`] otherwise. The lost digits _do not_
    /// necessarily represent an error term `err` where
    /// `num = round(num) + err` for every rounding mode, but it is exactly
    /// the error term when rounding is implemented via truncation.
    /// The lost digits are unsigned unless the rounded value is zero, in
    /// which case, the sign is just the sign of `num`.
    pub fn round_residual<T: Number>(&self, num: &T) -> (Rational, Option<Rational>) {
        assert!(
            self.max_p.is_some() || self.min_n.is_some(),
            "must specify either maximum precision or least absolute digit"
        );

        // case split by class
        if num.is_zero() {
            // zero
            (Rational::zero(), Some(Rational::zero()))
        } else if num.is_infinite() {
            // infinite number
            let s = num.is_negative().unwrap();
            (Rational::Infinite(s), None)
        } else if num.is_nar() {
            // other non-real
            (Rational::Nan, None)
        } else {
            // finite, non-zero value
            self.round_finite(num)
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundingContext for Context {
    type Rounded = Rational;

    fn round(&self, val: &Self::Rounded) -> Self::Rounded {
        self.mpmf_round(val)
    }

    fn mpmf_round<T: Number>(&self, num: &T) -> Self::Rounded {
        let (rounded, _) = self.round_residual(num);
        rounded
    }
}
