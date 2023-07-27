// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/mod.rs
//
// Top-level of the rational module.
// Exports public functions
//

use crate::ieee754::Context;
use crate::ieee754::IEEE754;
use crate::ops::*;
use crate::rational::Rational;
use crate::{Number, RoundingContext};

macro_rules! rounded_1ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for Context {
            fn $name<N: Number>(&self, src: &N) -> Self::Rounded {
                // compute approximately, rounding-to-odd, with 3 rounding bits
                let p = self.max_p() + 3;
                let r = Rational::from_number(src);
                let (result, flags) = r.$mpfr(p);
                let mut rounded = self.round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }

        impl Context {
            pub fn $name(&self, src: &IEEE754) -> IEEE754 {
                if src.is_nan() {
                    self.round(src)
                } else {
                    <Self as $tname>::$name(self, src)
                }
            }
        }
    };
}

rounded_1ary_impl!(RoundedNeg, neg, neg_with_mpfr);
rounded_1ary_impl!(RoundedSqrt, sqrt, sqrt_with_mpfr);
rounded_1ary_impl!(RoundedCbrt, cbrt, cbrt_with_mpfr);
rounded_1ary_impl!(RoundedExp, exp, exp_with_mpfr);
rounded_1ary_impl!(RoundedExp2, exp2, exp2_with_mpfr);
rounded_1ary_impl!(RoundedLog, log, log_with_mpfr);
rounded_1ary_impl!(RoundedLog2, log2, log2_with_mpfr);
rounded_1ary_impl!(RoundedLog10, log10, log10_with_mpfr);
rounded_1ary_impl!(RoundedExpm1, expm1, expm1_with_mpfr);
rounded_1ary_impl!(RoundedLog1p, log1p, log1p_with_mpfr);
rounded_1ary_impl!(RoundedSin, sin, sin_with_mpfr);
rounded_1ary_impl!(RoundedCos, cos, cos_with_mpfr);
rounded_1ary_impl!(RoundedTan, tan, tan_with_mpfr);
rounded_1ary_impl!(RoundedAsin, asin, asin_with_mpfr);
rounded_1ary_impl!(RoundedAcos, acos, acos_with_mpfr);
rounded_1ary_impl!(RoundedAtan, atan, atan_with_mpfr);
rounded_1ary_impl!(RoundedSinh, sinh, sinh_with_mpfr);
rounded_1ary_impl!(RoundedCosh, cosh, cosh_with_mpfr);
rounded_1ary_impl!(RoundedTanh, tanh, tanh_with_mpfr);
rounded_1ary_impl!(RoundedAsinh, asinh, asinh_with_mpfr);
rounded_1ary_impl!(RoundedAcosh, acosh, acosh_with_mpfr);
rounded_1ary_impl!(RoundedAtanh, atanh, atanh_with_mpfr);
rounded_1ary_impl!(RoundedErf, erf, erf_with_mpfr);
rounded_1ary_impl!(RoundedErfc, erfc, erfc_with_mpfr);
rounded_1ary_impl!(RoundedGamma, tgamma, tgamma_with_mpfr);
rounded_1ary_impl!(RoundedLgamma, lgamma, lgamma_with_mpfr);

macro_rules! rounded_2ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for Context {
            fn $name<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded {
                // compute approximately, rounding-to-odd, with 3 rounding bits
                let p = self.max_p() + 3;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let (result, flags) = r1.$mpfr(&r2, p);
                let mut rounded = self.round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }

        impl Context {
            pub fn $name(&self, src1: &IEEE754, src2: &IEEE754) -> IEEE754 {
                if src1.is_nan() || src2.is_nan() {
                    self.round(src1)
                } else {
                    <Self as $tname>::$name(self, src1, src2)
                }
            }
        }
    };
}

rounded_2ary_impl!(RoundedAdd, add, add_with_mpfr);
rounded_2ary_impl!(RoundedSub, sub, sub_with_mpfr);
rounded_2ary_impl!(RoundedMul, mul, mul_with_mpfr);
rounded_2ary_impl!(RoundedDiv, div, div_with_mpfr);
rounded_2ary_impl!(RoundedPow, pow, pow_with_mpfr);
rounded_2ary_impl!(RoundedHypot, hypot, hypot_with_mpfr);
rounded_2ary_impl!(RoundedFmod, fmod, fmod_with_mpfr);
rounded_2ary_impl!(RoundedRemainder, remainder, remainder_with_mpfr);
rounded_2ary_impl!(RoundedAtan2, atan2, atan2_with_mpfr);

macro_rules! rounded_3ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for Context {
            fn $name<N1: Number, N2: Number, N3: Number>(
                &self,
                src1: &N1,
                src2: &N2,
                src3: &N3,
            ) -> Self::Rounded {
                // compute approximately, rounding-to-odd, with 3 rounding bits
                let p = self.max_p() + 3;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let r3 = Rational::from_number(src3);
                let (result, flags) = r1.$mpfr(&r2, &r3, p);
                let mut rounded = self.round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }

        impl Context {
            pub fn $name(&self, src1: &IEEE754, src2: &IEEE754, src3: &IEEE754) -> IEEE754 {
                if src1.is_nan() || src2.is_nan() || src3.is_nan() {
                    self.round(src1)
                } else {
                    <Self as $tname>::$name(self, src1, src2, src3)
                }
            }
        }
    };
}

rounded_3ary_impl!(RoundedFMA, fma, fma_with_mpfr);
