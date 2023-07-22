// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/ops.rs
//
// Implementations of operators

use std::ops::Mul;

use crate::ops::*;
use crate::rational::*;
use crate::Number;
use crate::RoundingContext;

impl Rational {
    /// Multiplies two numbers of type [`Rational`] exactly.
    /// Panics if the arguments are not both real numbers.
    fn mul_exact(&self, other: &Self) -> Self {
        assert!(!self.is_nar() && !other.is_nar(), "must be real numbers");
        if self.is_zero() || other.is_zero() {
            // finite * zero is zero
            Self::zero()
        } else {
            // non-zero * non-zero is non-zero
            let sign = self.sign() != other.sign();
            let exp = self.exp().unwrap() + other.exp().unwrap();
            let c = self.c().unwrap() * other.c().unwrap();
            Self::Real(sign, exp, c)
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_exact(&rhs)
    }
}

impl RoundedMul<Context> for Rational {
    fn mul(&self, other: &Self, ctx: &Context) -> <Context as RoundingContext>::Rounded {
        match (&self, other) {
            // Invalid arguments means invalid result
            (Self::Nan, _) => Self::Nan,
            (_, Self::Nan) => Self::Nan,
            // Infinities
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
            },
            (Self::Real(_, _, _), Self::Real(_, _, _)) => {
                ctx.round(&self.mul_exact(other))
            }
        }
    }
}
