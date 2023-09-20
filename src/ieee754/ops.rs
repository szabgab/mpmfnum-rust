use crate::ieee754::Context;
use crate::ops::*;
use crate::rational::Rational;
use crate::{Number, RoundingContext};

macro_rules! rounded_1ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for Context {
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

            fn $mpmf<N: Number>(&self, src: &N) -> Self::Rounded {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r = Rational::from_number(src);
                let (result, flags) = r.$mpfr(p);
                let mut rounded = self.mpmf_round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }
    };
}

rounded_1ary_impl!(RoundedNeg, neg, mpmf_neg, neg_with_mpfr);
rounded_1ary_impl!(RoundedSqrt, sqrt, mpmf_sqrt, sqrt_with_mpfr);
rounded_1ary_impl!(RoundedCbrt, cbrt, mpmf_cbrt, cbrt_with_mpfr);
rounded_1ary_impl!(RoundedExp, exp, mpmf_exp, exp_with_mpfr);
rounded_1ary_impl!(RoundedExp2, exp2, mpmf_exp2, exp2_with_mpfr);
rounded_1ary_impl!(RoundedLog, log, mpmf_log, log_with_mpfr);
rounded_1ary_impl!(RoundedLog2, log2, mpmf_log2, log2_with_mpfr);
rounded_1ary_impl!(RoundedLog10, log10, mpmf_log10, log10_with_mpfr);
rounded_1ary_impl!(RoundedExpm1, expm1, mpmf_expm1, expm1_with_mpfr);
rounded_1ary_impl!(RoundedLog1p, log1p, mpmf_log1p, log1p_with_mpfr);
rounded_1ary_impl!(RoundedSin, sin, mpmf_sin, sin_with_mpfr);
rounded_1ary_impl!(RoundedCos, cos, mpmf_cos, cos_with_mpfr);
rounded_1ary_impl!(RoundedTan, tan, mpmf_tan, tan_with_mpfr);
rounded_1ary_impl!(RoundedAsin, asin, mpmf_asin, asin_with_mpfr);
rounded_1ary_impl!(RoundedAcos, acos, mpmf_acos, acos_with_mpfr);
rounded_1ary_impl!(RoundedAtan, atan, mpmf_atan, atan_with_mpfr);
rounded_1ary_impl!(RoundedSinh, sinh, mpmf_sinh, sinh_with_mpfr);
rounded_1ary_impl!(RoundedCosh, cosh, mpmf_cosh, cosh_with_mpfr);
rounded_1ary_impl!(RoundedTanh, tanh, mpmf_tanh, tanh_with_mpfr);
rounded_1ary_impl!(RoundedAsinh, asinh, mpmf_asinh, asinh_with_mpfr);
rounded_1ary_impl!(RoundedAcosh, acosh, mpmf_acosh, acosh_with_mpfr);
rounded_1ary_impl!(RoundedAtanh, atanh, mpmf_atanh, atanh_with_mpfr);
rounded_1ary_impl!(RoundedErf, erf, mpmf_erf, erf_with_mpfr);
rounded_1ary_impl!(RoundedErfc, erfc, mpmf_erfc, erfc_with_mpfr);
rounded_1ary_impl!(RoundedGamma, tgamma, mpmf_tgamma, tgamma_with_mpfr);
rounded_1ary_impl!(RoundedLgamma, lgamma, mpmf_lgamma, lgamma_with_mpfr);

macro_rules! rounded_2ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for Context {
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
                N1: Number,
                N2: Number,
            {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let (result, flags) = r1.$mpfr(&r2, p);
                let mut rounded = self.mpmf_round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }
    };
}

rounded_2ary_impl!(RoundedAdd, add, mpmf_add, add_with_mpfr);
rounded_2ary_impl!(RoundedSub, sub, mpmf_sub, sub_with_mpfr);
rounded_2ary_impl!(RoundedMul, mul, mpmf_mul, mul_with_mpfr);
rounded_2ary_impl!(RoundedDiv, div, mpmf_div, div_with_mpfr);
rounded_2ary_impl!(RoundedPow, pow, mpmf_pow, pow_with_mpfr);
rounded_2ary_impl!(RoundedHypot, hypot, mpmf_hypot, hypot_with_mpfr);
rounded_2ary_impl!(RoundedFmod, fmod, mpmf_fmod, fmod_with_mpfr);
rounded_2ary_impl!(
    RoundedRemainder,
    remainder,
    mpmf_remainder,
    remainder_with_mpfr
);
rounded_2ary_impl!(RoundedAtan2, atan2, mpmf_atan2, atan2_with_mpfr);

macro_rules! rounded_3ary_impl {
    ($tname:ident, $name:ident, $mpmf:ident, $mpfr:ident) => {
        impl $tname for Context {
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
                N1: Number,
                N2: Number,
                N3: Number,
            {
                // compute with 2 additional bits, rounding-to-odd
                let p = self.max_p() + 2;
                let r1 = Rational::from_number(src1);
                let r2 = Rational::from_number(src2);
                let r3 = Rational::from_number(src3);
                let (result, flags) = r1.$mpfr(&r2, &r3, p);
                let mut rounded = self.mpmf_round(&result);
                rounded.flags.invalid = flags.invalid;
                rounded.flags.divzero = flags.divzero;
                rounded
            }
        }
    };
}

rounded_3ary_impl!(RoundedFMA, fma, mpmf_fma, fma_with_mpfr);
