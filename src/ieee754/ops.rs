use crate::ieee754::IEEE754Context;
use crate::math::*;
use crate::ops::*;
use crate::rfloat::RFloat;
use crate::{Real, RoundingContext};

macro_rules! rounded_1ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for IEEE754Context {
            fn $name(&self, src: &Self::Rounded) -> Self::Rounded {
                if src.is_nan() {
                    let mut result = self.round(src);
                    result.flags.invalid = true;
                    result
                } else {
                    // may need to interpret subnormals as 0
                    let mut result = if self.daz() && src.is_subnormal() {
                        self.$mpmf(&src.ctx.zero(src.sign()))
                    } else {
                        self.$mpmf(src)
                    };

                    // override NaNs
                    if result.is_nan() {
                        let canon_nan = self.qnan();
                        result.num = canon_nan.num;
                    }

                    // set flags and return
                    result.flags.denorm = src.is_subnormal();
                    result
                }
            }

            fn $mpmf<N: Real>(&self, src: &N) -> Self::Rounded {
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

rounded_1ary_impl!(RoundedNeg, format_neg, neg, mpfr_neg);
rounded_1ary_impl!(RoundedSqrt, format_sqrt, sqrt, mpfr_sqrt);
rounded_1ary_impl!(RoundedCbrt, format_cbrt, cbrt, mpfr_cbrt);
rounded_1ary_impl!(RoundedExp, format_exp, exp, mpfr_exp);
rounded_1ary_impl!(RoundedExp2, format_exp2, exp2, mpfr_exp2);
rounded_1ary_impl!(RoundedLog, format_log, log, mpfr_log);
rounded_1ary_impl!(RoundedLog2, format_log2, log2, mpfr_log2);
rounded_1ary_impl!(RoundedLog10, format_log10, log10, mpfr_log10);
rounded_1ary_impl!(RoundedExpm1, format_expm1, expm1, mpfr_expm1);
rounded_1ary_impl!(RoundedLog1p, format_log1p, log1p, mpfr_log1p);
rounded_1ary_impl!(RoundedSin, format_sin, sin, mpfr_sin);
rounded_1ary_impl!(RoundedCos, format_cos, cos, mpfr_cos);
rounded_1ary_impl!(RoundedTan, format_tan, tan, mpfr_tan);
rounded_1ary_impl!(RoundedAsin, format_asin, asin, mpfr_asin);
rounded_1ary_impl!(RoundedAcos, format_acos, acos, mpfr_acos);
rounded_1ary_impl!(RoundedAtan, format_atan, atan, mpfr_atan);
rounded_1ary_impl!(RoundedSinh, format_sinh, sinh, mpfr_sinh);
rounded_1ary_impl!(RoundedCosh, format_cosh, cosh, mpfr_cosh);
rounded_1ary_impl!(RoundedTanh, format_tanh, tanh, mpfr_tanh);
rounded_1ary_impl!(RoundedAsinh, format_asinh, asinh, mpfr_asinh);
rounded_1ary_impl!(RoundedAcosh, format_acosh, acosh, mpfr_acosh);
rounded_1ary_impl!(RoundedAtanh, format_atanh, atanh, mpfr_atanh);
rounded_1ary_impl!(RoundedErf, format_erf, erf, mpfr_erf);
rounded_1ary_impl!(RoundedErfc, format_erfc, erfc, mpfr_erfc);
rounded_1ary_impl!(RoundedGamma, format_tgamma, tgamma, mpfr_tgamma);
rounded_1ary_impl!(RoundedLgamma, format_lgamma, lgamma, mpfr_lgamma);

macro_rules! rounded_2ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for IEEE754Context {
            fn $name(&self, src1: &Self::Rounded, src2: &Self::Rounded) -> Self::Rounded {
                if src1.is_nan() {
                    let mut result = self.round(src1);
                    result.flags.invalid = true;
                    result
                } else if src2.is_nan() {
                    let mut result = self.round(src2);
                    result.flags.invalid = true;
                    result
                } else {
                    // may need to interpret subnormals as 0
                    let daz1 = self.daz() && src1.is_subnormal();
                    let daz2 = self.daz() && src2.is_subnormal();
                    let mut result = match (daz1, daz2) {
                        (false, false) => self.$mpmf(src1, src2),
                        (false, true) => self.$mpmf(src1, &src2.ctx.zero(src2.sign())),
                        (true, false) => self.$mpmf(&src1.ctx.zero(src1.sign()), src2),
                        (true, true) => {
                            self.$mpmf(&src1.ctx.zero(src1.sign()), &src2.ctx.zero(src2.sign()))
                        }
                    };

                    // override NaNs
                    if result.is_nan() {
                        let canon_nan = self.qnan();
                        result.num = canon_nan.num;
                    }

                    // set flags and return
                    result.flags.denorm = src1.is_subnormal() || src2.is_subnormal();
                    result
                }
            }

            fn $mpmf<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
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

rounded_2ary_impl!(RoundedAdd, format_add, add, mpfr_add);
rounded_2ary_impl!(RoundedSub, format_sub, sub, mpfr_sub);
rounded_2ary_impl!(RoundedMul, format_mul, mul, mpfr_mul);
rounded_2ary_impl!(RoundedDiv, format_div, div, mpfr_div);
rounded_2ary_impl!(RoundedPow, format_pow, pow, mpfr_pow);
rounded_2ary_impl!(RoundedHypot, format_hypot, hypot, mpfr_hypot);
rounded_2ary_impl!(RoundedFmod, format_fmod, fmod, mpfr_fmod);
rounded_2ary_impl!(
    RoundedRemainder,
    format_remainder,
    remainder,
    mpfr_remainder
);
rounded_2ary_impl!(RoundedAtan2, format_atan2, atan2, mpfr_atan2);

macro_rules! rounded_3ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for IEEE754Context {
            fn $name(
                &self,
                src1: &Self::Rounded,
                src2: &Self::Rounded,
                src3: &Self::Rounded,
            ) -> Self::Rounded {
                if src1.is_nan() {
                    let mut result = self.round(src1);
                    result.flags.invalid = true;
                    result
                } else if src2.is_nan() {
                    let mut result = self.round(src2);
                    result.flags.invalid = true;
                    result
                } else if src3.is_nan() {
                    let mut result = self.round(src3);
                    result.flags.invalid = true;
                    result
                } else {
                    // may need to interpret subnormals as 0
                    let daz1 = self.daz() && src1.is_subnormal();
                    let daz2 = self.daz() && src2.is_subnormal();
                    let daz3 = self.daz() && src3.is_subnormal();
                    let mut result = match (daz1, daz2, daz3) {
                        (false, false, false) => self.$mpmf(src1, src2, src3),
                        (false, false, true) => self.$mpmf(src1, src2, &src3.ctx.zero(src3.sign())),
                        (false, true, false) => self.$mpmf(src1, &src2.ctx.zero(src2.sign()), src3),
                        (false, true, true) => self.$mpmf(
                            src1,
                            &src2.ctx.zero(src2.sign()),
                            &src3.ctx.zero(src3.sign()),
                        ),
                        (true, false, false) => self.$mpmf(&src1.ctx.zero(src1.sign()), src2, src3),
                        (true, false, true) => self.$mpmf(
                            &src1.ctx.zero(src1.sign()),
                            src2,
                            &src3.ctx.zero(src3.sign()),
                        ),
                        (true, true, false) => self.$mpmf(
                            &src1.ctx.zero(src1.sign()),
                            &src2.ctx.zero(src2.sign()),
                            src3,
                        ),
                        (true, true, true) => self.$mpmf(
                            &src1.ctx.zero(src1.sign()),
                            &src2.ctx.zero(src2.sign()),
                            &src3.ctx.zero(src3.sign()),
                        ),
                    };

                    // override NaNs
                    if result.is_nan() {
                        let canon_nan = self.qnan();
                        result.num = canon_nan.num;
                    }

                    // set flags and return
                    result.flags.denorm =
                        src1.is_subnormal() || src2.is_subnormal() || src3.is_subnormal();
                    result
                }
            }

            fn $mpmf<N1, N2, N3>(&self, src1: &N1, src2: &N2, src3: &N3) -> Self::Rounded
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

rounded_3ary_impl!(RoundedFMA, format_fma, fma, mpfr_fma);
