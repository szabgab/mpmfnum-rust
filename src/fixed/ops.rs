use crate::fixed::FixedContext;
use crate::math::*;
use crate::ops::*;
use crate::rational::Rational;
use crate::{Real, RoundingContext};

macro_rules! rounded_1ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for FixedContext {
            fn $name(&self, src: &Self::Rounded) -> Self::Rounded {
                self.$mpmf(src)
            }

            fn $mpmf<N: Real>(&self, src: &N) -> Self::Rounded {
                // compute approximately, rounding-to-odd,
                // with 2 rounding bits
                let p = self.nbits + 2;
                let r = Rational::from_number(src);
                let result = $mpfr(r, p);
                let mut rounded = self.mpmf_round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded
            }
        }
    };
}

rounded_1ary_impl!(RoundedNeg, neg, mpmf_neg, mpfr_neg);
rounded_1ary_impl!(RoundedSqrt, sqrt, mpmf_sqrt, mpfr_sqrt);
rounded_1ary_impl!(RoundedCbrt, cbrt, mpmf_cbrt, mpfr_cbrt);
rounded_1ary_impl!(RoundedExp, exp, mpmf_exp, mpfr_exp);
rounded_1ary_impl!(RoundedExp2, exp2, mpmf_exp2, mpfr_exp2);
rounded_1ary_impl!(RoundedLog, log, mpmf_log, mpfr_log);
rounded_1ary_impl!(RoundedLog2, log2, mpmf_log2, mpfr_log2);
rounded_1ary_impl!(RoundedLog10, log10, mpmf_log10, mpfr_log10);
rounded_1ary_impl!(RoundedExpm1, expm1, mpmf_expm1, mpfr_expm1);
rounded_1ary_impl!(RoundedLog1p, log1p, mpmf_log1p, mpfr_log1p);
rounded_1ary_impl!(RoundedSin, sin, mpmf_sin, mpfr_sin);
rounded_1ary_impl!(RoundedCos, cos, mpmf_cos, mpfr_cos);
rounded_1ary_impl!(RoundedTan, tan, mpmf_tan, mpfr_tan);
rounded_1ary_impl!(RoundedAsin, asin, mpmf_asin, mpfr_asin);
rounded_1ary_impl!(RoundedAcos, acos, mpmf_acos, mpfr_acos);
rounded_1ary_impl!(RoundedAtan, atan, mpmf_atan, mpfr_atan);
rounded_1ary_impl!(RoundedSinh, sinh, mpmf_sinh, mpfr_sinh);
rounded_1ary_impl!(RoundedCosh, cosh, mpmf_cosh, mpfr_cosh);
rounded_1ary_impl!(RoundedTanh, tanh, mpmf_tanh, mpfr_tanh);
rounded_1ary_impl!(RoundedAsinh, asinh, mpmf_asinh, mpfr_asinh);
rounded_1ary_impl!(RoundedAcosh, acosh, mpmf_acosh, mpfr_acosh);
rounded_1ary_impl!(RoundedAtanh, atanh, mpmf_atanh, mpfr_atanh);
rounded_1ary_impl!(RoundedErf, erf, mpmf_erf, mpfr_erf);
rounded_1ary_impl!(RoundedErfc, erfc, mpmf_erfc, mpfr_erfc);
rounded_1ary_impl!(RoundedGamma, tgamma, mpmf_tgamma, mpfr_tgamma);
rounded_1ary_impl!(RoundedLgamma, lgamma, mpmf_lgamma, mpfr_lgamma);

macro_rules! rounded_2ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for FixedContext {
            fn $name(&self, src1: &Self::Rounded, src2: &Self::Rounded) -> Self::Rounded {
                self.$mpmf(src1, src2)
            }

            fn $mpmf<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
            where
                N1: Real,
                N2: Real,
            {
                // compute approximately, rounding-to-odd,
                // with 2 rounding bits
                let p = self.nbits + 2;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let result = $mpfr(r1, r2, p);
                let mut rounded = self.mpmf_round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded
            }
        }
    };
}

rounded_2ary_impl!(RoundedAdd, add, mpmf_add, mpfr_add);
rounded_2ary_impl!(RoundedSub, sub, mpmf_sub, mpfr_sub);
rounded_2ary_impl!(RoundedMul, mul, mpmf_mul, mpfr_mul);
rounded_2ary_impl!(RoundedDiv, div, mpmf_div, mpfr_div);
rounded_2ary_impl!(RoundedPow, pow, mpmf_pow, mpfr_pow);
rounded_2ary_impl!(RoundedHypot, hypot, mpmf_hypot, mpfr_hypot);
rounded_2ary_impl!(RoundedFmod, fmod, mpmf_fmod, mpfr_fmod);
rounded_2ary_impl!(RoundedRemainder, remainder, mpmf_remainder, mpfr_remainder);
rounded_2ary_impl!(RoundedAtan2, atan2, mpmf_atan2, mpfr_atan2);

macro_rules! rounded_3ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for FixedContext {
            fn $name(
                &self,
                src1: &Self::Rounded,
                src2: &Self::Rounded,
                src3: &Self::Rounded,
            ) -> Self::Rounded {
                self.$mpmf(src1, src2, src3)
            }

            fn $mpmf<N1, N2, N3>(&self, src1: &N1, src2: &N2, src3: &N3) -> Self::Rounded
            where
                N1: Real,
                N2: Real,
                N3: Real,
            {
                // compute approximately, rounding-to-odd,
                // with 2 rounding bits
                let p = self.nbits + 2;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let r3 = Rational::from_number(src3);
                let result = $mpfr(r1, r2, r3, p);
                let mut rounded = self.mpmf_round(result.num());
                rounded.flags.invalid = result.flags().invalid;
                rounded
            }
        }
    };
}

rounded_3ary_impl!(RoundedFMA, fma, mpmf_fma, mpfr_fma);
