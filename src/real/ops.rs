use std::{
    cmp::min,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::{Signed, Zero};

use crate::{
    ops::{RoundedAdd, RoundedMul, RoundedNeg, RoundedSub},
    rfloat::RFloat,
    Real, RoundingContext,
};

use super::RealContext;

impl RoundedNeg for RealContext {
    fn neg<N: Real>(&self, src: &N) -> Self::Rounded {
        let src = self.round(src); // convert (exactly) to RFloat
        match src {
            RFloat::Real(s, exp, c) => RFloat::Real(!s, exp, c),
            RFloat::Infinite(s) => RFloat::Infinite(!s),
            RFloat::Nan => RFloat::Nan,
        }
    }
}

impl RoundedAdd for RealContext {
    fn add<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
    where
        N1: Real,
        N2: Real,
    {
        let src1 = self.round(src1); // convert (exactly) to RFloat
        let src2 = self.round(src2); // convert (exactly) to RFloat
        match (src1, src2) {
            // invalid arguments means invalid result
            (RFloat::Nan, _) | (_, RFloat::Nan) => RFloat::Nan,
            // infinities
            (RFloat::Infinite(s1), RFloat::Infinite(s2)) => {
                if s1 == s2 {
                    RFloat::Infinite(s1)
                } else {
                    RFloat::Nan
                }
            }
            (RFloat::Infinite(s), _) | (_, RFloat::Infinite(s)) => RFloat::Infinite(s),
            // finite
            (RFloat::Real(s1, exp1, c1), RFloat::Real(s2, exp2, c2)) => {
                if c2.is_zero() {
                    // x + 0 = x
                    RFloat::Real(s1, exp1, c1)
                } else if c1.is_zero() {
                    // 0 + y = y
                    RFloat::Real(s2, exp2, c2)
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
                    RFloat::Real(m.is_negative(), exp, m.abs())
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
        let src1 = self.round(src1); // convert (exactly) to RFloat
        let src2 = self.round(src2); // convert (exactly) to RFloat
        match (src1, src2) {
            // invalid arguments means invalid result
            (RFloat::Nan, _) | (_, RFloat::Nan) => RFloat::Nan,
            // infinities
            (RFloat::Infinite(s1), RFloat::Infinite(s2)) => RFloat::Infinite(s1 != s2),
            (RFloat::Infinite(sinf), RFloat::Real(s, _, c))
            | (RFloat::Real(s, _, c), RFloat::Infinite(sinf)) => {
                if c.is_zero() {
                    // Inf * 0 is undefined
                    RFloat::Nan
                } else {
                    // Inf * non-zero is just Inf
                    RFloat::Infinite(sinf != s)
                }
            }
            // finite values
            (RFloat::Real(s1, exp1, c1), RFloat::Real(s2, exp2, c2)) => {
                if c1.is_zero() || c2.is_zero() {
                    // finite * zero is zero
                    RFloat::zero()
                } else {
                    // non-zero * non-zero is non-zero
                    RFloat::Real(s1 != s2, exp1 + exp2, c1 * c2)
                }
            }
        }
    }
}

//
//  Convenient trait impls
//

impl Neg for RFloat {
    type Output = RFloat;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            RFloat::zero()
        } else {
            RealContext::new().neg(&self)
        }
    }
}

impl Add for RFloat {
    type Output = RFloat;

    fn add(self, rhs: Self) -> Self::Output {
        RealContext::new().add(&self, &rhs)
    }
}

impl Sub for RFloat {
    type Output = RFloat;

    fn sub(self, rhs: Self) -> Self::Output {
        RealContext::new().sub(&self, &rhs)
    }
}

impl Mul for RFloat {
    type Output = RFloat;

    fn mul(self, rhs: Self) -> Self::Output {
        RealContext::new().mul(&self, &rhs)
    }
}
