// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/types.rs
//
// The rational number type

use std::cmp::min;
use std::cmp::Ordering;

use gmp::mpz::Mpz;
use gmp::sign::Sign;

use rug::float::prec_min;
use rug::float::Special;
use rug::Float;

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
    Real(bool, isize, Mpz),
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
        match self {
            // (exp - 1) + len(c)
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some((exp - 1) + (c.bit_length() as isize))
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

    fn c(&self) -> Option<Mpz> {
        match self {
            Rational::Real(_, _, c) => Some(c.clone()),
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn m(&self) -> Option<Mpz> {
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
            Rational::Real(_, _, c) => {
                if c.is_zero() {
                    0
                } else {
                    c.bit_length()
                }
            }
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
    /// Constructs zero.
    pub fn zero() -> Self {
        Rational::Real(false, 0, Mpz::from(0))
    }

    /// Constructs positive one.
    pub fn one() -> Self {
        Rational::Real(false, 0, Mpz::from(1))
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
                    // non-zero, finite <?> non-zero finit

                    // normalize: inefficient but slow
                    let n1 = exp1 - 1;
                    let n2 = exp2 - 1;
                    let n = min(n1, n2);

                    // compare ordinals
                    let mut ord1 = c1 << (n1 - n) as usize;
                    let mut ord2 = c2 << (n2 - n) as usize;

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
        match &val {
            Rational::Nan => Float::with_val(prec_min(), Special::Nan),
            Rational::Infinite(s) => {
                if *s {
                    Float::with_val(prec_min(), Special::NegInfinity)
                } else {
                    Float::with_val(prec_min(), Special::Infinity)
                }
            }
            Rational::Real(s, exp, c) => {
                if c.is_zero() {
                    Float::with_val(prec_min(), 0.0)
                } else {
                    let mut f = Float::new(val.p() as u32);

                    unsafe {
                        // set `f` to `c * 2^exp`
                        let ptr = c.inner() as *const mpz_t;
                        let t =
                            mpfr::set_z_2exp(f.as_raw_mut(), ptr, *exp as i64, mpfr::rnd_t::RNDN);
                        assert_eq!(t, 0, "should have been exact");

                        // negate if necessary
                        if *s {
                            let t = mpfr::neg(f.as_raw_mut(), f.as_raw(), mpfr::rnd_t::RNDN);
                            assert_eq!(t, 0, "should have been exact");
                        }
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
            let mut m = Mpz::zero();
            let exp: isize;

            unsafe {
                let ptr = m.inner_mut() as *mut mpz_t;
                exp = mpfr::get_z_2exp(ptr, val.as_raw()) as isize;
            }

            Self::Real(m.sign() == Sign::Negative, exp, m.abs()).canonicalize()
        }
    }
}
