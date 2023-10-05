use std::{
    cmp::min,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::{Signed, Zero};

use crate::{
    ops::{RoundedAdd, RoundedMul, RoundedNeg, RoundedSub},
    rational::Rational,
    Real, RoundingContext,
};

use super::RealContext;

impl RoundedNeg for RealContext {
    fn neg<N: Real>(&self, src: &N) -> Self::Rounded {
        let src = self.round(src); // convert (exactly) to Rational
        match src {
            Rational::Real(s, exp, c) => Rational::Real(!s, exp, c),
            Rational::Infinite(s) => Rational::Infinite(!s),
            Rational::Nan => Rational::Nan,
        }
    }
}

impl RoundedAdd for RealContext {
    fn add<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
    where
        N1: Real,
        N2: Real,
    {
        let src1 = self.round(src1); // convert (exactly) to Rational
        let src2 = self.round(src2); // convert (exactly) to Rational
        match (src1, src2) {
            // invalid arguments means invalid result
            (Rational::Nan, _) | (_, Rational::Nan) => Rational::Nan,
            // infinities
            (Rational::Infinite(s1), Rational::Infinite(s2)) => {
                if s1 == s2 {
                    Rational::Infinite(s1)
                } else {
                    Rational::Nan
                }
            }
            (Rational::Infinite(s), _) | (_, Rational::Infinite(s)) => Rational::Infinite(s),
            // finite
            (Rational::Real(s1, exp1, c1), Rational::Real(s2, exp2, c2)) => {
                if c2.is_zero() {
                    // x + 0 = x
                    Rational::Real(s1, exp1, c1)
                } else if c1.is_zero() {
                    // 0 + y = y
                    Rational::Real(s2, exp2, c2)
                } else {
                    // need to normalize mantissas:
                    // resulting exponent is the minimum of the
                    // exponent of the arguments
                    let exp = min(exp1, exp2);
                    let c1 = c1 << (exp1 - exp);
                    let c2 = c2 << (exp2 - exp);

                    // add signed integers
                    let m = match (s1, s2) {
                        (false, false) => c1 + c2,
                        (false, true) => c1 - c2,
                        (true, false) => c2 - c1,
                        (true, true) => -(c1 + c2),
                    };

                    // compose result
                    Rational::Real(m.is_negative(), exp, m.abs())
                }
            }
        }
    }
}

impl RoundedSub for RealContext {
    fn sub<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
    where
        N1: Real,
        N2: Real,
    {
        self.add(src1, &self.neg(src2))
    }
}

impl RoundedMul for RealContext {
    fn mul<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
    where
        N1: Real,
        N2: Real,
    {
        let src1 = self.round(src1); // convert (exactly) to Rational
        let src2 = self.round(src2); // convert (exactly) to Rational
        match (src1, src2) {
            // invalid arguments means invalid result
            (Rational::Nan, _) | (_, Rational::Nan) => Rational::Nan,
            // infinities
            (Rational::Infinite(s1), Rational::Infinite(s2)) => Rational::Infinite(s1 != s2),
            (Rational::Infinite(sinf), Rational::Real(s, _, c))
            | (Rational::Real(s, _, c), Rational::Infinite(sinf)) => {
                if c.is_zero() {
                    // Inf * 0 is undefined
                    Rational::Nan
                } else {
                    // Inf * non-zero is just Inf
                    Rational::Infinite(sinf != s)
                }
            }
            // finite values
            (Rational::Real(s1, exp1, c1), Rational::Real(s2, exp2, c2)) => {
                if c1.is_zero() || c2.is_zero() {
                    // finite * zero is zero
                    Rational::zero()
                } else {
                    // non-zero * non-zero is non-zero
                    Rational::Real(s1 != s2, exp1 + exp2, c1 * c2)
                }
            }
        }
    }
}

//
//  Convenient trait impls
//

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            Rational::zero()
        } else {
            RealContext::new().neg(&self)
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        RealContext::new().add(&self, &rhs)
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        RealContext::new().sub(&self, &rhs)
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        RealContext::new().mul(&self, &rhs)
    }
}
