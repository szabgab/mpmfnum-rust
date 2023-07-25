// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/ops.rs
//
// Implementations of operators

use std::cmp::min;
use std::ops::{Add, Mul, Neg, Sub};

use num_traits::{Zero, Signed};
use rug::{Float, Integer};
use gmp_mpfr_sys::mpfr;

use crate::rational::*;

macro_rules! mpfr_1ary {
    ($name:ident; $mpfr:ident; $cname:expr) => {
        #[doc = "Applies `"]
        #[doc = $cname]
        #[doc = "` to a [`Rational`] number with `p`
                 precision using MPFR, rounding to odd."]
        pub fn $name(&self, p: usize) -> Self {
            use mpfr::{rnd_t::RNDZ, PREC_MAX, PREC_MIN};
            assert!(
                p as i64 > PREC_MIN && p as i64 <= PREC_MAX,
                "precision must be between {} and {}",
                PREC_MIN + 1,
                PREC_MAX
            );

            // compute with `p - 1` bits
            let mut dst = Float::new((p - 1) as u32);
            let src = Float::from(self.clone());
            let t = unsafe { mpfr::$mpfr(dst.as_raw_mut(), src.as_raw(), RNDZ) };

            // apply correction to get the last bit
            Rational::from(dst).with_ternary(t)
        }
    };
}

macro_rules! mpfr_2ary {
    ($name:ident; $mpfr:ident; $cname:expr) => {
        #[doc = "Applies `"]
        #[doc = $cname]
        #[doc = "` to two [`Rational`] numbers with `p`
                 precision using MPFR, rounding to odd."]
        pub fn $name(&self, other: &Self, p: usize) -> Self {
            use mpfr::{rnd_t::RNDZ, PREC_MAX, PREC_MIN};
            assert!(
                p as i64 > PREC_MIN && p as i64 <= PREC_MAX,
                "precision must be between {} and {}",
                PREC_MIN + 1,
                PREC_MAX
            );

            // compute with `p - 1` bits
            let mut dst = Float::new((p - 1) as u32);
            let src1 = Float::from(self.clone());
            let src2 = Float::from(other.clone());
            let t = unsafe { mpfr::$mpfr(dst.as_raw_mut(), src1.as_raw(), src2.as_raw(), RNDZ) };

            // apply correction to get the last bit
            Rational::from(dst).with_ternary(t)
        }
    };
}

impl Rational {
    /// Adds two numbers of type [`Rational`] exactly.
    /// Addition of non-real values follows the usual IEEE 754 rules.
    pub fn add_exact(&self, other: &Self) -> Self {
        match (&self, other) {
            // invalid arguments means invalid result
            (Self::Nan, _) => Self::Nan,
            (_, Self::Nan) => Self::Nan,
            // infinities
            (Self::Infinite(s1), Self::Infinite(s2)) => {
                if *s1 == *s2 {
                    Self::Infinite(*s1)
                } else {
                    Self::Nan
                }
            }
            (Self::Infinite(s), _) | (_, Self::Infinite(s)) => Self::Infinite(*s),
            // finite
            (Self::Real(s1, exp1, c1), Self::Real(s2, exp2, c2)) => {
                if c2.is_zero() {
                    // x + 0 = x
                    Self::Real(*s1, *exp1, c1.clone())
                } else if c1.is_zero() {
                    // 0 + y = y
                    Self::Real(*s2, *exp2, c2.clone())
                } else {
                    // need to normalize mantissas:
                    // resulting exponent is the minimum of the
                    // exponent of the arguments
                    let exp = min(*exp1, *exp2);
                    let c1 = Integer::from(c1 << (*exp1 - exp));
                    let c2 = Integer::from(c2 << (*exp2 - exp));

                    // add signed integers
                    let m = match (*s1, *s2) {
                        (false, false) => c1 + c2,
                        (false, true) => c1 - c2,
                        (true, false) => c2 - c1,
                        (true, true) => -(c1 + c2),
                    };

                    // compose result
                    Self::Real(m.is_negative(), exp, m.abs())
                }
            }
        }
    }

    /// Multiplies two numbers of type [`Rational`] exactly.
    /// Multiplication of non-real values follows the usual
    /// IEEE 754 rules.
    pub fn mul_exact(&self, other: &Self) -> Self {
        match (&self, other) {
            // invalid arguments means invalid result
            (Self::Nan, _) => Self::Nan,
            (_, Self::Nan) => Self::Nan,
            // infinities
            (Self::Infinite(s1), Self::Infinite(s2)) => Self::Infinite(*s1 != *s2),
            (Self::Infinite(sinf), Self::Real(s, _, c))
            | (Self::Real(s, _, c), Self::Infinite(sinf)) => {
                if c.is_zero() {
                    // Inf * 0 is undefined
                    Self::Nan
                } else {
                    // Inf * non-zero is just Inf
                    Self::Infinite(*sinf != *s)
                }
            }
            // finite values
            (Self::Real(s1, exp1, c1), Self::Real(s2, exp2, c2)) => {
                if c1.is_zero() || c2.is_zero() {
                    // finite * zero is zero
                    Self::zero()
                } else {
                    // non-zero * non-zero is non-zero
                    Self::Real(s1 != s2, exp1 + exp2, Integer::from(c1 * c2))
                }
            }
        }
    }

    /// Applies a correction to a [`Rational`] type from an MPFR ternary
    /// value to translate a rounded result of precision `p - 1` obtained
    /// with round-to-zero to a rounded result of precision `p` obtained
    /// with round-to-odd.
    fn with_ternary(mut self, t: i32) -> Self {
        if let Rational::Real(s, exp, c) = &self {
            // the last bit is '1' if the result is inexact (`t != 0`).
            let c = if t == 0 {
                Integer::from(c << 1)
            } else {
                Integer::from(c << 1) + 1
            };
            self = Rational::Real(*s, *exp, c);
        }

        self
    }

    // Unary operators
    mpfr_1ary!(neg_with_mpfr; neg; "(- x)");
    mpfr_1ary!(sqrt_with_mpfr; sqrt; "sqrt(x)");
    mpfr_1ary!(cbrt_with_mpfr; cbrt; "cbrt(x)");
    mpfr_1ary!(exp_with_mpfr; exp; "exp(x)");
    mpfr_1ary!(exp2_with_mpfr; exp2; "2^x");
    // mpfr_1ary!(exp10_with_mpfr; exp10; "exp10(x)");
    mpfr_1ary!(expm1_with_mpfr; expm1; "e^x - 1");
    mpfr_1ary!(log_with_mpfr; log; "ln(x)");
    mpfr_1ary!(log2_with_mpfr; log2; "log2(x)");
    mpfr_1ary!(log10_with_mpfr; log10; "log10(x)");
    mpfr_1ary!(log1p_with_mpfr; log1p; "ln(x + 1)");
    mpfr_1ary!(sin_with_mpfr; sin; "sin(x)");
    mpfr_1ary!(cos_with_mpfr; cos; "cos(x)");
    mpfr_1ary!(tan_with_mpfr; tan; "tan(x)");
    mpfr_1ary!(asin_with_mpfr; asin; "arcsin(x)");
    mpfr_1ary!(acos_with_mpfr; acos; "arccos(x)");
    mpfr_1ary!(atan_with_mpfr; atan; "arctan(x)");
    mpfr_1ary!(sinh_with_mpfr; sinh; "sinh(x)");
    mpfr_1ary!(cosh_with_mpfr; cosh; "cosh(x)");
    mpfr_1ary!(tanh_with_mpfr; tanh; "tanh(x)");
    mpfr_1ary!(asinh_with_mpfr; asinh; "arsinh(x)");
    mpfr_1ary!(acosh_with_mpfr; acosh; "arcosh(x)");
    mpfr_1ary!(atanh_with_mpfr; atanh; "artanh(x)");
    mpfr_1ary!(erf_with_mpfr; erf; "erf(x)");
    mpfr_1ary!(erfc_with_mpfr; erfc; "erfc(x)");
    mpfr_1ary!(tgamma_with_mpfr; gamma; "tgamma(x)");
    mpfr_1ary!(lgamma_with_mpfr; lngamma; "erfc(x)");

    // Binary operators
    mpfr_2ary!(add_with_mpfr; add; "x + y");
    mpfr_2ary!(sub_with_mpfr; sub; "x - y");
    mpfr_2ary!(mul_with_mpfr; mul; "x * y");
    mpfr_2ary!(div_with_mpfr; div; "x / y");
    mpfr_2ary!(pow_with_mpfr; pow; "x ^ y");
    mpfr_2ary!(hypot_with_mpfr; hypot; "sqrt(x^2 + y^2)");
    mpfr_2ary!(fmod_with_mpfr; fmod; "fmod(x, y)");
    mpfr_2ary!(remainder_with_mpfr; remainder; "remainder(x, y)");
    mpfr_2ary!(atan2_with_mpfr; atan2; "arctan(y / x)");
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        match &self {
            Self::Nan => Self::Nan,
            Self::Infinite(s) => Self::Infinite(!s),
            Self::Real(s, exp, c) => Self::Real(!s, *exp, c.clone()).canonicalize(),
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_exact(&rhs)
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.mul_exact(&-rhs)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_exact(&rhs)
    }
}
