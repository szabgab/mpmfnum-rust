use std::cmp::Ordering;
use std::cmp::{max, min};

use num_traits::{Signed, Zero};
use rug::Float as MPFRFloat;
use rug::Integer;

use gmp_mpfr_sys::gmp::mpz_t;
use gmp_mpfr_sys::mpfr;

use crate::Number;

/// The floating-point number format.
/// 
/// This is not an IEEE-754 style floating-point number.
/// This type defines a base-2 scientific number `(-1)^s * c * 2^e`
/// where `c` is a fixed-precision unsigned-integer and
/// `e` is theoretically unbounded  any integer
/// (In practice, this is an [`isize`] value).
/// 
/// Any [`Float`] value may encode a non-real number (see [`NAN`]) which is
/// interpreted as a NaN (neither finite nor infinite). All operations
/// canonicalize -0 to +0 (no sign bit).
#[derive(Debug, Clone)]
pub enum Float {
    /// A finite (real) number specified by the canonical triple
    /// of sign, exponent, significand.
    Real(bool, isize, Integer),
    /// An infinite number (signed to indicate direction).
    Infinite(bool),
    /// Not a real number; either an undefined or infinte result.
    Nan,
}

/// An instantiation of [`Float::Nan`].
pub const NAN: Float = Float::Nan;

/// An instantiation of [`Float::Infinite`] with positive sign.
pub const POS_INF: Float = Float::Infinite(false);

/// An instantiation of [`Float::Infinite`] with negative sign.
pub const NEG_INF: Float = Float::Infinite(true);

// Implements the `Number` trait for `Float`.
// See `Float` for a description of the trait and its members.
impl Number for Float {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        match self {
            Float::Real(s, _, _) => *s,
            Float::Infinite(s) => *s,
            Float::Nan => false,
        }
    }

    fn exp(&self) -> Option<isize> {
        match self {
            Float::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*exp)
                }
            }
            Float::Infinite(_) => None,
            Float::Nan => None,
        }
    }

    fn e(&self) -> Option<isize> {
        // (exp - 1) + len(c)
        match self {
            Float::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some((exp - 1) + c.significant_bits() as isize)
                }
            }
            Float::Infinite(_) => None,
            Float::Nan => None,
        }
    }

    fn n(&self) -> Option<isize> {
        match self {
            // exp - 1
            Float::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(exp - 1)
                }
            }
            Float::Infinite(_) => None,
            Float::Nan => None,
        }
    }

    fn c(&self) -> Option<Integer> {
        match self {
            Float::Real(_, _, c) => Some(c.clone()),
            Float::Infinite(_) => None,
            Float::Nan => None,
        }
    }

    fn m(&self) -> Option<Integer> {
        match self {
            Float::Real(s, _, c) => {
                if *s {
                    Some(-c.clone())
                } else {
                    Some(c.clone())
                }
            }
            Float::Infinite(_) => None,
            Float::Nan => None,
        }
    }

    fn p(&self) -> usize {
        match self {
            Float::Real(_, _, c) => c.significant_bits() as usize,
            Float::Infinite(_) => 0,
            Float::Nan => 0,
        }
    }

    fn is_nar(&self) -> bool {
        match self {
            Float::Real(_, _, _) => false,
            Float::Infinite(_) => true,
            Float::Nan => true,
        }
    }

    fn is_finite(&self) -> bool {
        match self {
            Float::Real(_, _, _) => true,
            Float::Infinite(_) => false,
            Float::Nan => false,
        }
    }

    fn is_infinite(&self) -> bool {
        match self {
            Float::Real(_, _, _) => false,
            Float::Infinite(_) => true,
            Float::Nan => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Float::Real(_, _, c) => c.is_zero(),
            Float::Infinite(_) => false,
            Float::Nan => false,
        }
    }

    fn is_negative(&self) -> Option<bool> {
        match self {
            Float::Real(s, _, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*s)
                }
            }
            Float::Infinite(s) => Some(*s),
            Float::Nan => None,
        }
    }

    fn is_numerical(&self) -> bool {
        match self {
            Float::Real(_, _, _) => true,
            Float::Infinite(_) => true,
            Float::Nan => false,
        }
    }
}

impl Float {
    /// Constructs the canonical zero for this format.
    pub fn zero() -> Self {
        Float::Real(false, 0, Integer::from(0))
    }

    /// Constructs the canonical +1 for this format.
    pub fn one() -> Self {
        Float::Real(false, 0, Integer::from(1))
    }

    /// Returns true if the number is [`NAN`].
    pub fn is_nan(&self) -> bool {
        matches!(self, Float::Nan)
    }

    /// Canonicalizes this number.
    /// All zeros are mapped to [`Float::Real(false, 0, 0)`].
    pub fn canonicalize(&self) -> Self {
        if self.is_zero() {
            Float::zero()
        } else {
            self.clone()
        }
    }

    /// Returns the `n`th absolute binary digit.
    pub fn get_bit(&self, n: isize) -> bool {
        match self {
            Float::Nan => false,
            Float::Infinite(_) => false,
            Float::Real(_, _, c) if c.is_zero() => false,
            Float::Real(_, exp, c) => {
                let e = self.e().unwrap();
                let exp = *exp;
                if n < exp || n > e {
                    // below the least significant digit or above
                    // the most significant digit
                    false
                } else {
                    c.get_bit((n - exp) as u32)
                }
            }
        }
    }

    /// Constructs a [`Float`] value from a [`Number`].
    /// This is the default conversion function from
    /// any implementation of the [`Number`] trait.
    pub fn from_number<N: Number>(val: &N) -> Self {
        // case split by class
        if !val.is_numerical() {
            // Any non-numerical type is NaN
            Self::Nan
        } else if val.is_infinite() {
            // Any infinity is either +/- infinity.
            Self::Infinite(val.sign())
        } else if val.is_zero() {
            // Any zero is just +0
            Self::zero()
        } else {
            // Finite, non-zero
            Self::Real(val.sign(), val.exp().unwrap(), val.c().unwrap())
        }
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Float::Nan, _) => None,
            (_, Float::Nan) => None,
            (Float::Infinite(s1), Float::Infinite(s2)) => {
                if *s1 == *s2 {
                    // infinities of the same sign
                    Some(Ordering::Equal)
                } else if *s1 {
                    // -Inf < +Inf
                    Some(Ordering::Less)
                } else {
                    // +Inf > -Inf
                    Some(Ordering::Greater)
                }
            }
            (Float::Infinite(s), _) => {
                if *s {
                    // -Inf < finite
                    Some(Ordering::Less)
                } else {
                    // +Inf > finite
                    Some(Ordering::Greater)
                }
            }
            (_, Float::Infinite(s)) => {
                if *s {
                    // finite > -Inf
                    Some(Ordering::Greater)
                } else {
                    // finite < +Inf
                    Some(Ordering::Less)
                }
            }
            (Float::Real(s1, exp1, c1), Float::Real(s2, exp2, c2)) => {
                // finite <?> finite
                // check for zero
                if c1.is_zero() && c2.is_zero() {
                    Some(Ordering::Equal)
                } else if c1.is_zero() {
                    if *s2 {
                        // 0 > -finite
                        Some(Ordering::Greater)
                    } else {
                        // 0 < finite
                        Some(Ordering::Less)
                    }
                } else if c2.is_zero() {
                    if *s1 {
                        // -finite < 0
                        Some(Ordering::Less)
                    } else {
                        // finite > 0
                        Some(Ordering::Greater)
                    }
                } else {
                    // non-zero, finite <?> non-zero, finite

                    // normalize: inefficient but slow
                    let n1 = exp1 - 1;
                    let n2 = exp2 - 1;
                    let n = min(n1, n2);

                    // compare ordinals
                    let mut ord1 = Integer::from(c1 << (n1 - n));
                    let mut ord2 = Integer::from(c2 << (n2 - n));

                    if *s1 {
                        ord1 = -ord1;
                    }

                    if *s2 {
                        ord2 = -ord2;
                    }

                    Some(ord1.cmp(&ord2))
                }
            }
        }
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            Some(_) => false,
            None => false,
        }
    }
}

impl From<Float> for MPFRFloat {
    fn from(val: Float) -> Self {
        use rug::float::*;
        match val {
            Float::Nan => MPFRFloat::with_val(prec_min(), Special::Nan),
            Float::Infinite(s) => {
                if s {
                    rug::Float::with_val(prec_min(), Special::NegInfinity)
                } else {
                    rug::Float::with_val(prec_min(), Special::Infinity)
                }
            }
            Float::Real(s, exp, c) => {
                if c.is_zero() {
                    MPFRFloat::with_val(prec_min(), 0.0)
                } else {
                    let mut f = MPFRFloat::new(max(1, c.significant_bits()));
                    let rnd = mpfr::rnd_t::RNDN;
                    let exp = exp as i64;
                    let m = if s { -c } else { c };

                    unsafe {
                        // set `f` to `c * 2^exp`
                        let src_ptr = m.as_raw() as *const mpz_t;
                        let dest_ptr = f.as_raw_mut();
                        let t = mpfr::set_z_2exp(dest_ptr, src_ptr, exp, rnd);
                        assert_eq!(t, 0, "should have been exact");
                    }

                    f
                }
            }
        }
    }
}

impl From<MPFRFloat> for Float {
    fn from(val: MPFRFloat) -> Self {
        if val.is_nan() {
            Self::Nan
        } else if val.is_infinite() {
            Self::Infinite(val.is_sign_negative())
        } else if val.is_zero() {
            Self::zero()
        } else {
            let mut m = Integer::zero();
            let exp: isize;

            unsafe {
                let ptr = m.as_raw_mut() as *mut mpz_t;
                exp = mpfr::get_z_2exp(ptr, val.as_raw()) as isize;
            }

            Self::Real(m.is_negative(), exp, m.abs()).canonicalize()
        }
    }
}
