// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/ops.rs
//
// Implementations of operators

use std::cmp::min;
use std::ops::{Add, Mul, Neg, Sub};

use rug::Float;

use gmp::mpz::Mpz;
use gmp::sign::Sign;
use gmp_mpfr_sys::mpfr;

use crate::ops::*;
use crate::rational::*;
use crate::RoundingContext;

macro_rules! mpfr_1ary {
    ($name:ident; $mpfr:ident; $cname:expr) => {
        #[doc = "Applies `"]
        #[doc = $cname]
        #[doc = "`to two [`Rational`] numbers with `p` precision using MPFR, rounding to odd."]
        pub fn $name(&self, p: usize) -> Self {
            use mpfr::{rnd_t::RNDZ, PREC_MAX, PREC_MIN};
            assert!(
                p as i64 >= PREC_MIN && p as i64 <= PREC_MAX,
                "precision must be between {} and {}",
                PREC_MIN,
                PREC_MAX
            );

            let mut dst = Float::new(p as u32);
            let src = Float::from(self.clone());
            let t = unsafe { mpfr::$mpfr(dst.as_raw_mut(), src.as_raw(), RNDZ) };

            Rational::from(dst).with_ternary(t)
        }
    };
}

macro_rules! mpfr_2ary {
    ($name:ident; $mpfr:ident; $cname:expr) => {
        #[doc = "Applies `"]
        #[doc = $cname]
        #[doc = "`to two [`Rational`] numbers with `p` precision using MPFR, rounding to odd."]
        pub fn $name(&self, other: &Self, p: usize) -> Self {
            use mpfr::{rnd_t::RNDZ, PREC_MAX, PREC_MIN};
            assert!(
                p as i64 >= PREC_MIN && p as i64 <= PREC_MAX,
                "precision must be between {} and {}",
                PREC_MIN,
                PREC_MAX
            );

            let mut dst = Float::new(p as u32);
            let src1 = Float::from(self.clone());
            let src2 = Float::from(other.clone());
            let t = unsafe { mpfr::$mpfr(dst.as_raw_mut(), src1.as_raw(), src2.as_raw(), RNDZ) };

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
                    let c1 = c1 << (*exp1 - exp) as usize;
                    let c2 = c2 << (*exp2 - exp) as usize;

                    // add signed integers
                    let m = match (*s1, *s2) {
                        (false, false) => c1 + c2,
                        (false, true) => c1 - c2,
                        (true, false) => c2 - c1,
                        (true, true) => -(c1 + c2),
                    };

                    // compose result
                    Self::Real(m.sign() == Sign::Negative, exp, m.abs())
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
                    Self::Real(s1 != s2, exp1 + exp2, c1 * c2)
                }
            }
        }
    }

    /// Applies a correction to a [`Rational`] type from an MPFR ternary
    /// value to translate round-to-zero to round odd.
    fn with_ternary(mut self, t: i32) -> Self {
        if let Rational::Real(s, exp, c) = &self {
            if t != 0 && c.tstbit(0) {
                self = Rational::Real(*s, *exp, c + Mpz::from(1));
            }
        }

        self
    }

    // Unary operators
    mpfr_1ary!(sqrt_with_mpfr; sqrt; "sqrt");
    mpfr_1ary!(cbrt_with_mpfr; cbrt; "cbrt");
    mpfr_1ary!(exp_with_mpfr; exp; "exp");
    mpfr_1ary!(exp2_with_mpfr; exp2; "exp2");
    mpfr_1ary!(exp10_with_mpfr; exp10; "exp10");
    mpfr_1ary!(log_with_mpfr; log; "log");
    mpfr_1ary!(log2_with_mpfr; log2; "log2");
    mpfr_1ary!(log10_with_mpfr; log10; "log10");

    // Binary operators
    mpfr_2ary!(add_with_mpfr; add; "add");
    mpfr_2ary!(sub_with_mpfr; sub; "sub");
    mpfr_2ary!(mul_with_mpfr; mul; "mul");
    mpfr_2ary!(div_with_mpfr; div; "div");
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

impl RoundedAdd<Context> for Rational {
    fn add(&self, other: &Self, ctx: &Context) -> Rational {
        ctx.round(&self.add_exact(other))
    }
}

impl RoundedMul<Context> for Rational {
    fn mul(&self, other: &Self, ctx: &Context) -> Rational {
        ctx.round(&self.mul_exact(other))
    }
}
