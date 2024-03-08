use std::{
    cmp::min,
    ops::{Add, Mul, Neg, Sub},
};

use crate::{
    ops::{RoundedAbs, RoundedAdd, RoundedMul, RoundedNeg, RoundedSub},
    rfloat::RFloat,
    Real, RoundingContext,
};

use super::RealContext;

impl RoundedNeg for RealContext {
    fn neg<N: Real>(&self, src: &N) -> Self::Format {
        let src = self.round(src); // convert (exactly) to RFloat
        match src {
            RFloat::Real(s, exp, c) => RFloat::Real(!s, exp, c),
            RFloat::PosInfinity => RFloat::NegInfinity,
            RFloat::NegInfinity => RFloat::PosInfinity,
            RFloat::Nan => RFloat::Nan,
        }
    }
}

impl RoundedAbs for RealContext {
    fn abs<N: Real>(&self, src: &N) -> Self::Format {
        let src = self.round(src); // convert (exactly) to RFloat
        match src {
            RFloat::Real(_, exp, c) => RFloat::Real(false, exp, c),
            RFloat::PosInfinity | RFloat::NegInfinity => RFloat::PosInfinity,
            RFloat::Nan => RFloat::Nan,
        }
    }
}

impl RoundedAdd for RealContext {
    fn add<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Format
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
            (RFloat::PosInfinity, RFloat::PosInfinity) => RFloat::PosInfinity,
            (RFloat::NegInfinity, RFloat::NegInfinity) => RFloat::NegInfinity,
            (RFloat::PosInfinity, RFloat::NegInfinity)
            | (RFloat::NegInfinity, RFloat::PosInfinity) => RFloat::Nan,
            (RFloat::PosInfinity, _) | (_, RFloat::PosInfinity) => RFloat::PosInfinity,
            (RFloat::NegInfinity, _) | (_, RFloat::NegInfinity) => RFloat::NegInfinity,
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
    fn sub<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Format
    where
        N1: Real,
        N2: Real,
    {
        self.add(src1, &self.neg(src2))
    }
}

impl RoundedMul for RealContext {
    fn mul<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Format
    where
        N1: Real,
        N2: Real,
    {
        let src1 = self.round(src1); // convert (exactly) to RFloat
        let src2 = self.round(src2); // convert (exactly) to RFloat

        // case split by class
        // match statements are somehow worse
        if src1.is_nan() || src2.is_nan() {
            // undefined
            RFloat::Nan
        } else if src1.is_infinite() {
            if src2.is_zero() {
                // Inf * 0 is undefined
                RFloat::Nan
            } else if src1.sign().unwrap() == src2.sign().unwrap() {
                // Inf * non-zero (same signs)
                RFloat::PosInfinity
            } else {
                // Inf * non-zero (opposite signs)
                RFloat::NegInfinity
            }
        } else if src2.is_infinite() {
            if src1.is_zero() {
                // 0 * Inf is undefined
                RFloat::Nan
            } else if src1.sign().unwrap() == src2.sign().unwrap() {
                // non-zero * Inf (same signs)
                RFloat::PosInfinity
            } else {
                // non-zero * Inf (opposite signs)
                RFloat::NegInfinity
            }
        } else if src1.is_zero() || src2.is_zero() {
            // 0 * finite is 0
            RFloat::zero()
        } else {
            // finite, non-zero * finite, non-zero
            let s1 = src1.sign().unwrap();
            let exp1 = src1.exp().unwrap();
            let c1 = src1.c().unwrap();

            let s2 = src2.sign().unwrap();
            let exp2 = src2.exp().unwrap();
            let c2 = src2.c().unwrap();

            RFloat::Real(s1 != s2, exp1 + exp2, c1 * c2)
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
