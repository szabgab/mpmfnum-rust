use std::cmp::Ordering;
use std::cmp::{max, min};

use num_traits::{Signed, Zero};
use rug::{Float, Integer};

use gmp_mpfr_sys::gmp::mpz_t;
use gmp_mpfr_sys::mpfr;

use crate::Number;

/// The rational number format.
///
/// This is not a traditional rational number `p/q` where `p` and `q`
/// are integers (canonically, `p` is signed). Instead, this type defines
/// a _fixed-width_ rational number `(-1)^s * c * 2^e` where `c` is a
/// binary-encoded integer with a maximum bitwidth. Like rational numbers,
/// `e` is theoretically unbounded and may be as large or small as needed.
/// Rational numbers may encode a non-real number (see [`NAN`]) which is
/// interpreted as a NaN (neither finite nor infinite). All operations
/// canonicalize -0 to +0 (no sign bit).
#[derive(Debug, Clone)]
pub enum Rational {
    /// A finite (real) number specified by the canonical triple
    /// of sign, exponent, significand.
    Real(bool, isize, Integer),
    /// An infinite number (signed to indicate direction).
    Infinite(bool),
    /// Not a real number; either an undefined or infinte result.
    Nan,
}

/// An instantiation of [`Rational::Nan`].
pub const NAN: Rational = Rational::Nan;

/// An instantiation of [`Rational::Infinite`] with positive sign.
pub const POS_INF: Rational = Rational::Infinite(false);

/// An instantiation of [`Rational::Infinite`] with negative sign.
pub const NEG_INF: Rational = Rational::Infinite(true);

// Implements the `Number` trait for `Rational`.
// See `Rational` for a description of the trait and its members.
impl Number for Rational {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        match self {
            Rational::Real(s, _, _) => *s,
            Rational::Infinite(s) => *s,
            Rational::Nan => false,
        }
    }

    fn exp(&self) -> Option<isize> {
        match self {
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*exp)
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn e(&self) -> Option<isize> {
        // (exp - 1) + len(c)
        match self {
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some((exp - 1) + c.significant_bits() as isize)
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn n(&self) -> Option<isize> {
        match self {
            // exp - 1
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(exp - 1)
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn c(&self) -> Option<Integer> {
        match self {
            Rational::Real(_, _, c) => Some(c.clone()),
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn m(&self) -> Option<Integer> {
        match self {
            Rational::Real(s, _, c) => {
                if *s {
                    Some(-c.clone())
                } else {
                    Some(c.clone())
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn p(&self) -> usize {
        match self {
            Rational::Real(_, _, c) => c.significant_bits() as usize,
            Rational::Infinite(_) => 0,
            Rational::Nan => 0,
        }
    }

    fn is_nar(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => false,
            Rational::Infinite(_) => true,
            Rational::Nan => true,
        }
    }

    fn is_finite(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => true,
            Rational::Infinite(_) => false,
            Rational::Nan => false,
        }
    }

    fn is_infinite(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => false,
            Rational::Infinite(_) => true,
            Rational::Nan => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Rational::Real(_, _, c) => c.is_zero(),
            Rational::Infinite(_) => false,
            Rational::Nan => false,
        }
    }

    fn is_negative(&self) -> Option<bool> {
        match self {
            Rational::Real(s, _, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*s)
                }
            }
            Rational::Infinite(s) => Some(*s),
            Rational::Nan => None,
        }
    }

    fn is_numerical(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => true,
            Rational::Infinite(_) => true,
            Rational::Nan => false,
        }
    }
}

impl Rational {
    /// Constructs the canonical zero for this format.
    pub fn zero() -> Self {
        Rational::Real(false, 0, Integer::from(0))
    }

    /// Constructs the canonical +1 for this format.
    pub fn one() -> Self {
        Rational::Real(false, 0, Integer::from(1))
    }

    /// Returns true if the number is [`NAN`].
    pub fn is_nan(&self) -> bool {
        matches!(self, Rational::Nan)
    }

    /// Canonicalizes this number.
    /// All zeros are mapped to [`Rational::Real(false, 0, 0)`].
    pub fn canonicalize(&self) -> Self {
        if self.is_zero() {
            Rational::zero()
        } else {
            self.clone()
        }
    }

    /// Returns the `n`th absolute binary digit.
    pub fn get_bit(&self, n: isize) -> bool {
        match self {
            Rational::Nan => false,
            Rational::Infinite(_) => false,
            Rational::Real(_, _, c) if c.is_zero() => false,
            Rational::Real(_, exp, c) => {
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

    /// Constructs a [`Rational`] value from a [`Number`].
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

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Rational::Nan, _) => None,
            (_, Rational::Nan) => None,
            (Rational::Infinite(s1), Rational::Infinite(s2)) => {
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
            (Rational::Infinite(s), _) => {
                if *s {
                    // -Inf < finite
                    Some(Ordering::Less)
                } else {
                    // +Inf > finite
                    Some(Ordering::Greater)
                }
            }
            (_, Rational::Infinite(s)) => {
                if *s {
                    // finite > -Inf
                    Some(Ordering::Greater)
                } else {
                    // finite < +Inf
                    Some(Ordering::Less)
                }
            }
            (Rational::Real(s1, exp1, c1), Rational::Real(s2, exp2, c2)) => {
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

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            Some(_) => false,
            None => false,
        }
    }
}

impl From<Rational> for Float {
    fn from(val: Rational) -> Self {
        use rug::float::*;
        match val {
            Rational::Nan => Float::with_val(prec_min(), Special::Nan),
            Rational::Infinite(s) => {
                if s {
                    Float::with_val(prec_min(), Special::NegInfinity)
                } else {
                    Float::with_val(prec_min(), Special::Infinity)
                }
            }
            Rational::Real(s, exp, c) => {
                if c.is_zero() {
                    Float::with_val(prec_min(), 0.0)
                } else {
                    let mut f = Float::new(max(1, c.significant_bits()));
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

impl From<Float> for Rational {
    fn from(val: Float) -> Self {
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
