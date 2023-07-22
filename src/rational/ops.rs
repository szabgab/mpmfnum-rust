// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/ops.rs
//
// Implementations of operators

use std::cmp::min;
use std::ops::{Add, Mul, Neg};

use gmp::sign::Sign;

use crate::ops::*;
use crate::rational::*;
use crate::RoundingContext;

impl Rational {
    /// Adds two numbers of type [`Rational`] exactly.
    /// Addition of non-real values follows the usual IEEE 754 rules.
    fn add_exact(&self, other: &Self) -> Self {
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
    fn mul_exact(&self, other: &Self) -> Self {
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

impl RoundedAdd<Context> for Rational {
    fn add(&self, other: &Self, ctx: &Context) -> Rational {
        ctx.round(&self.add_exact(&other))
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_exact(&rhs)
    }
}

impl RoundedMul<Context> for Rational {
    fn mul(&self, other: &Self, ctx: &Context) -> Rational {
        ctx.round(&self.mul_exact(&other))
    }
}
