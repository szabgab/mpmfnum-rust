use crate::float::FloatContext;
use crate::mpfr::*;
use crate::ops::*;
use crate::rfloat::RFloat;
use crate::{Real, RoundingContext};

macro_rules! rounded_1ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for FloatContext {
            fn $name<N: Real>(&self, src: &N) -> Self::Format {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r = RFloat::from_number(src);
                let result = $mpfr(r, p);
                let mut rounded = self.round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded.flags.divzero = result.flags().divzero;
                rounded
            }
        }
    };
}

rounded_1ary_impl!(RoundedNeg, neg, mpfr_neg);
rounded_1ary_impl!(RoundedAbs, abs, mpfr_abs);
rounded_1ary_impl!(RoundedSqrt, sqrt, mpfr_sqrt);
rounded_1ary_impl!(RoundedCbrt, cbrt, mpfr_cbrt);
rounded_1ary_impl!(RoundedRecip, recip, mpfr_recip);
rounded_1ary_impl!(RoundedRecipSqrt, recip_sqrt, mpfr_recip_sqrt);
rounded_1ary_impl!(RoundedExp, exp, mpfr_exp);
rounded_1ary_impl!(RoundedExp2, exp2, mpfr_exp2);
rounded_1ary_impl!(RoundedLog, log, mpfr_log);
rounded_1ary_impl!(RoundedLog2, log2, mpfr_log2);
rounded_1ary_impl!(RoundedLog10, log10, mpfr_log10);
rounded_1ary_impl!(RoundedExpm1, expm1, mpfr_expm1);
rounded_1ary_impl!(RoundedExp2m1, exp2m1, mpfr_exp2m1);
rounded_1ary_impl!(RoundedExp10m1, exp10m1, mpfr_exp10m1);
rounded_1ary_impl!(RoundedLog1p, log1p, mpfr_log1p);
rounded_1ary_impl!(RoundedLog2p1, log2p1, mpfr_log2p1);
rounded_1ary_impl!(RoundedLog10p1, log10p1, mpfr_log10p1);
rounded_1ary_impl!(RoundedSin, sin, mpfr_sin);
rounded_1ary_impl!(RoundedCos, cos, mpfr_cos);
rounded_1ary_impl!(RoundedTan, tan, mpfr_tan);
rounded_1ary_impl!(RoundedSinPi, sin_pi, mpfr_sin_pi);
rounded_1ary_impl!(RoundedCosPi, cos_pi, mpfr_cos_pi);
rounded_1ary_impl!(RoundedTanPi, tan_pi, mpfr_tan_pi);
rounded_1ary_impl!(RoundedAsin, asin, mpfr_asin);
rounded_1ary_impl!(RoundedAcos, acos, mpfr_acos);
rounded_1ary_impl!(RoundedAtan, atan, mpfr_atan);
rounded_1ary_impl!(RoundedSinh, sinh, mpfr_sinh);
rounded_1ary_impl!(RoundedCosh, cosh, mpfr_cosh);
rounded_1ary_impl!(RoundedTanh, tanh, mpfr_tanh);
rounded_1ary_impl!(RoundedAsinh, asinh, mpfr_asinh);
rounded_1ary_impl!(RoundedAcosh, acosh, mpfr_acosh);
rounded_1ary_impl!(RoundedAtanh, atanh, mpfr_atanh);
rounded_1ary_impl!(RoundedErf, erf, mpfr_erf);
rounded_1ary_impl!(RoundedErfc, erfc, mpfr_erfc);
rounded_1ary_impl!(RoundedGamma, tgamma, mpfr_tgamma);
rounded_1ary_impl!(RoundedLgamma, lgamma, mpfr_lgamma);

macro_rules! rounded_2ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for FloatContext {
            fn $name<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Format
            where
                N1: Real,
                N2: Real,
            {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r1 = RFloat::from_number(src1);
                let r2 = RFloat::from_number(src2);
                let result = $mpfr(r1, r2, p);
                let mut rounded = self.round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded.flags.divzero = result.flags().divzero;
                rounded
            }
        }
    };
}

rounded_2ary_impl!(RoundedAdd, add, mpfr_add);
rounded_2ary_impl!(RoundedSub, sub, mpfr_sub);
rounded_2ary_impl!(RoundedMul, mul, mpfr_mul);
rounded_2ary_impl!(RoundedDiv, div, mpfr_div);
rounded_2ary_impl!(RoundedPow, pow, mpfr_pow);
rounded_2ary_impl!(RoundedHypot, hypot, mpfr_hypot);
rounded_2ary_impl!(RoundedFmod, fmod, mpfr_fmod);
rounded_2ary_impl!(RoundedRemainder, remainder, mpfr_remainder);
rounded_2ary_impl!(RoundedAtan2, atan2, mpfr_atan2);

macro_rules! rounded_3ary_impl {
    ($tname:ident, $name:ident, $mpfr:ident) => {
        impl $tname for FloatContext {
            fn $name<N1, N2, N3>(&self, src1: &N1, src2: &N2, src3: &N3) -> Self::Format
            where
                N1: Real,
                N2: Real,
                N3: Real,
            {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r1 = RFloat::from_number(src1);
                let r2 = RFloat::from_number(src2);
                let r3 = RFloat::from_number(src3);
                let result = $mpfr(r1, r2, r3, p);
                let mut rounded = self.round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded.flags.divzero = result.flags().divzero;
                rounded
            }
        }
    };
}

rounded_3ary_impl!(RoundedFMA, fma, mpfr_fma);
