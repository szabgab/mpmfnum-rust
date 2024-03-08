use std::cmp::Ordering;
use std::cmp::{max, min};

use num_traits::Zero;
use rug::{Float, Integer};

use gmp_mpfr_sys::mpfr;

use crate::Real;

/// An arbitrary-precision, floating-point numbers with unbounded exponent.
///
/// The associated [`RoundingContext`][crate::RoundingContext]
/// implementation is [`RFloatContext`][crate::rfloat::RFloatContext].
/// See [`RFloatContext`][crate::rfloat::RFloatContext] for more details
/// on numerical properties of the [`RFloat`] type.
///
/// All operations canonicalize -0 to +0 (no sign bit).
#[derive(Debug, Clone)]
pub enum RFloat {
    /// A finite (real) number specified by the canonical triple
    /// of sign, exponent, significand.
    Real(bool, isize, Integer),
    /// A positive infinity.
    PosInfinity,
    /// A negative infinity.
    NegInfinity,
    /// Not a real number; either an undefined or infinte result.
    Nan,
}

// Implements the `Real` trait for  RFloat`.
// See  RFloat` for a description of the trait and its members.
impl Real for RFloat {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> Option<bool> {
        match self {
            RFloat::Real(s, _, _) => Some(*s),
            RFloat::PosInfinity => Some(false),
            RFloat::NegInfinity => Some(true),
            RFloat::Nan => None,
        }
    }

    fn exp(&self) -> Option<isize> {
        match self {
            RFloat::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*exp)
                }
            }
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn e(&self) -> Option<isize> {
        // (exp - 1) + len(c)
        match self {
            RFloat::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some((exp - 1) + c.significant_bits() as isize)
                }
            }
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn n(&self) -> Option<isize> {
        match self {
            // exp - 1
            RFloat::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(exp - 1)
                }
            }
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn c(&self) -> Option<Integer> {
        match self {
            RFloat::Real(_, _, c) => Some(c.clone()),
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn m(&self) -> Option<Integer> {
        match self {
            RFloat::Real(s, _, c) => {
                if *s {
                    Some(-c.clone())
                } else {
                    Some(c.clone())
                }
            }
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn prec(&self) -> Option<usize> {
        match self {
            RFloat::Real(_, _, c) => Some(c.significant_bits() as usize),
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Nan => None,
        }
    }

    fn is_nar(&self) -> bool {
        match self {
            RFloat::Real(_, _, _) => false,
            RFloat::PosInfinity => true,
            RFloat::NegInfinity => true,
            RFloat::Nan => true,
        }
    }

    fn is_finite(&self) -> bool {
        matches!(self, RFloat::Real(_, _, _))
    }

    fn is_infinite(&self) -> bool {
        match self {
            RFloat::Real(_, _, _) => false,
            RFloat::PosInfinity => true,
            RFloat::NegInfinity => true,
            RFloat::Nan => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            RFloat::Real(_, _, c) => c.is_zero(),
            RFloat::PosInfinity => false,
            RFloat::NegInfinity => false,
            RFloat::Nan => false,
        }
    }

    fn is_negative(&self) -> Option<bool> {
        match self {
            RFloat::Real(s, _, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*s)
                }
            }
            RFloat::PosInfinity => Some(false),
            RFloat::NegInfinity => Some(true),
            RFloat::Nan => None,
        }
    }

    fn is_numerical(&self) -> bool {
        match self {
            RFloat::Real(_, _, _) => true,
            RFloat::PosInfinity => true,
            RFloat::NegInfinity => true,
            RFloat::Nan => false,
        }
    }
}

impl RFloat {
    /// Constructs the canonical zero for this format.
    pub fn zero() -> Self {
        RFloat::Real(false, 0, Integer::from(0))
    }

    /// Constructs the canonical +1 for this format.
    pub fn one() -> Self {
        RFloat::Real(false, 0, Integer::from(1))
    }

    /// Returns true if the value is not-a-number.
    pub fn is_nan(&self) -> bool {
        matches!(self, RFloat::Nan)
    }

    /// Canonicalizes this number.
    /// All zeros are mapped to +0.0.
    pub fn canonicalize(&self) -> Self {
        if self.is_zero() {
            RFloat::zero()
        } else {
            self.clone()
        }
    }

    /// Returns the `n`th absolute binary digit.
    /// Only well-defined for finite, non-zero numbers.
    pub fn get_bit(&self, n: isize) -> Option<bool> {
        match self {
            RFloat::Nan => None,
            RFloat::PosInfinity => None,
            RFloat::NegInfinity => None,
            RFloat::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else if n < *exp || n > self.e().unwrap() {
                    // below the least significant digit OR
                    // above the most significant digit
                    Some(false)
                } else {
                    // within the significand
                    Some(c.get_bit((n - exp) as u32))
                }
            }
        }
    }

    /// Constructs a [ RFloat`] value from a [`Real`].
    /// This is the default conversion function from
    /// any implementation of the [`Real`] trait.
    pub fn from_number<N: Real>(val: &N) -> Self {
        // case split by class
        if !val.is_numerical() {
            // Any non-numerical type is NaN
            Self::Nan
        } else if val.is_infinite() {
            // Any infinity is either +/- infinity.
            if val.sign().unwrap() {
                Self::NegInfinity
            } else {
                Self::PosInfinity
            }
        } else if val.is_zero() {
            // Any zero is just +0
            Self::zero()
        } else {
            // Finite, non-zero
            Self::Real(val.sign().unwrap(), val.exp().unwrap(), val.c().unwrap())
        }
    }
}

impl PartialOrd for RFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RFloat::Nan, _) => None,
            (_, RFloat::Nan) => None,
            (RFloat::PosInfinity, RFloat::PosInfinity) => Some(Ordering::Equal),
            (RFloat::NegInfinity, RFloat::NegInfinity) => Some(Ordering::Equal),
            (RFloat::PosInfinity, _) => Some(Ordering::Greater),
            (RFloat::NegInfinity, _) => Some(Ordering::Less),
            (_, RFloat::NegInfinity) => Some(Ordering::Greater),
            (_, RFloat::PosInfinity) => Some(Ordering::Less),
            (RFloat::Real(s1, exp1, c1), RFloat::Real(s2, exp2, c2)) => {
                // finite <?> finite
                // check for zero values
                match (c1.is_zero(), c2.is_zero()) {
                    (true, true) => {
                        // both zero => equal
                        Some(Ordering::Equal)
                    }
                    (true, false) => {
                        if *s2 {
                            // 0 > -finite
                            Some(Ordering::Greater)
                        } else {
                            // 0 < finite
                            Some(Ordering::Less)
                        }
                    }
                    (false, true) => {
                        if *s1 {
                            // -finite < 0
                            Some(Ordering::Less)
                        } else {
                            // finite > 0
                            Some(Ordering::Greater)
                        }
                    }
                    (false, false) => {
                        // finite, non-zero <?> finite, non-zero
                        // check by increasing order of complexity: signs first
                        if *s1 != *s2 {
                            if *s1 {
                                // self < 0 < other
                                Some(Ordering::Less)
                            } else {
                                // self > 0 > other
                                Some(Ordering::Greater)
                            }
                        } else {
                            // signs are the same, so we need to check magnitude
                            // use the normalized exponent first (position of the MSB)
                            let e1 = (exp1 - 1) + (c1.significant_bits() as isize);
                            let e2 = (exp2 - 1) + (c2.significant_bits() as isize);
                            let mag_cmp = match e1.cmp(&e2) {
                                Ordering::Less => Ordering::Less,
                                Ordering::Greater => Ordering::Greater,
                                Ordering::Equal => {
                                    // slow path: need to normalize
                                    let n1 = exp1 - 1;
                                    let n2 = exp2 - 1;
                                    let n = min(n1, n2);

                                    // compare ordinals
                                    let ord1 = Integer::from(c1 << (n1 - n));
                                    let ord2 = Integer::from(c2 << (n2 - n));
                                    ord1.cmp(&ord2)
                                }
                            };

                            // need to possibly flip if negative
                            if *s1 {
                                Some(mag_cmp.reverse())
                            } else {
                                Some(mag_cmp)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialEq for RFloat {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            Some(_) => false,
            None => false,
        }
    }
}

impl From<RFloat> for Float {
    fn from(val: RFloat) -> Self {
        use rug::float::*;
        match val {
            RFloat::Nan => Float::with_val(prec_min(), Special::Nan),
            RFloat::PosInfinity => Float::with_val(prec_min(), Special::Infinity),
            RFloat::NegInfinity => Float::with_val(prec_min(), Special::NegInfinity),
            RFloat::Real(s, exp, c) => {
                if c.is_zero() {
                    Float::with_val(prec_min(), 0.0)
                } else {
                    let mut f = Float::new(max(1, c.significant_bits()));
                    let rnd = mpfr::rnd_t::RNDN;
                    let exp = exp as i64;
                    let m = if s { -c } else { c };

                    unsafe {
                        // set `f` to `c * 2^exp`
                        let t = mpfr::set_z_2exp(f.as_raw_mut(), m.as_raw(), exp, rnd);
                        assert_eq!(t, 0, "should have been exact");
                    }

                    f
                }
            }
        }
    }
}

impl From<Float> for RFloat {
    fn from(val: Float) -> Self {
        if val.is_nan() {
            Self::Nan
        } else if val.is_infinite() {
            if val.is_sign_negative() {
                Self::NegInfinity
            } else {
                Self::PosInfinity
            }
        } else if val.is_zero() {
            Self::zero()
        } else {
            let mut m = Integer::zero();
            let exp: isize;

            unsafe {
                exp = mpfr::get_z_2exp(m.as_raw_mut(), val.as_raw()) as isize;
            }

            Self::Real(m.is_negative(), exp, m.abs()).canonicalize()
        }
    }
}
