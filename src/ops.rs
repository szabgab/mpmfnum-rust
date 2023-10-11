//! Rounded mathematical operations.
//!
//! Implementations of these traits operate on [`Real`] types,
//! rounding the result according to a given [`RoundingContext`].
//!

use crate::{Real, RoundingContext};

macro_rules! rounded_1ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output."]
            #[doc = $mpmf]
            #[doc = "."]
            fn $imp(&self, src: &Self::Rounded) -> Self::Rounded {
                self.$mpmf(src)
            }

            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $mpmf<N: Real>(&self, src: &N) -> Self::Rounded;
        }
    };
}

// Traits for 1-ary operators
rounded_1ary!(RoundedNeg, format_neg, neg, "-x");
rounded_1ary!(RoundedAbs, format_abs, abs, "|x|");
rounded_1ary!(RoundedSqrt, format_sqrt, sqrt, "sqrt(x)");
rounded_1ary!(RoundedCbrt, format_cbrt, cbrt, "cbrt(x)");
rounded_1ary!(RoundedRecip, format_recip, recip, "1/x");
rounded_1ary!(RoundedRecipSqrt, format_recip_sqrt, recip_sqrt, "1/sqrt(x)");
rounded_1ary!(RoundedExp, format_exp, exp, "exp(x)");
rounded_1ary!(RoundedExp2, format_exp2, exp2, "2^x");
rounded_1ary!(RoundedLog, format_log, log, "ln(x)");
rounded_1ary!(RoundedLog2, format_log2, log2, "log2(x)");
rounded_1ary!(RoundedLog10, format_log10, log10, "log10(x)");
rounded_1ary!(RoundedExpm1, format_expm1, expm1, "e^x - 1");
rounded_1ary!(RoundedExp2m1, format_exp2m1, exp2m1, "2^x - 1");
rounded_1ary!(RoundedExp10m1, format_exp10m1, exp10m1, "10^x - 1");
rounded_1ary!(RoundedLog1p, format_log1p, log1p, "log(x + 1)");
rounded_1ary!(RoundedLog2p1, format_log2p1, log2p1, "log2(x + 1)");
rounded_1ary!(RoundedLog10p1, format_log10p1, log10p1, "log10(x + 1)");
rounded_1ary!(RoundedSin, format_sin, sin, "sin(x)");
rounded_1ary!(RoundedCos, format_cos, cos, "cos(x)");
rounded_1ary!(RoundedTan, format_tan, tan, "tan(x)");
rounded_1ary!(RoundedSinPi, format_sin_pi, sin_pi, "sin(pi * x)");
rounded_1ary!(RoundedCosPi, format_cos_pi, cos_pi, "cos(pi * x)");
rounded_1ary!(RoundedTanPi, format_tan_pi, tan_pi, "tan(pi * x)");
rounded_1ary!(RoundedAsin, format_asin, asin, "arcsin(x)");
rounded_1ary!(RoundedAcos, format_acos, acos, "arccos(x)");
rounded_1ary!(RoundedAtan, format_atan, atan, "arctan(x)");
rounded_1ary!(RoundedSinh, format_sinh, sinh, "sinh(x)");
rounded_1ary!(RoundedCosh, format_cosh, cosh, "cosh(x)");
rounded_1ary!(RoundedTanh, format_tanh, tanh, "tanh(x)");
rounded_1ary!(RoundedAsinh, format_asinh, asinh, "arsinh(x)");
rounded_1ary!(RoundedAcosh, format_acosh, acosh, "arcosh(x)");
rounded_1ary!(RoundedAtanh, format_atanh, atanh, "artanh(x)");
rounded_1ary!(RoundedErf, format_erf, erf, "erf(x)");
rounded_1ary!(RoundedErfc, format_erfc, erfc, "erfc(x)");
rounded_1ary!(RoundedGamma, format_tgamma, tgamma, "tgamma(x)");
rounded_1ary!(RoundedLgamma, format_lgamma, lgamma, "lgamma(x)");

macro_rules! rounded_2ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output."]
            fn $imp(&self, src1: &Self::Rounded, src2: &Self::Rounded) -> Self::Rounded {
                self.$mpmf(src1, src2)
            }

            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $mpmf<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Rounded
            where
                N1: Real,
                N2: Real;
        }
    };
}

// Traits for 2-ary operators
rounded_2ary!(RoundedAdd, format_add, add, "x + y");
rounded_2ary!(RoundedSub, format_sub, sub, "x - y");
rounded_2ary!(RoundedMul, format_mul, mul, "x * y");
rounded_2ary!(RoundedDiv, format_div, div, "x / y");
rounded_2ary!(RoundedPow, format_pow, pow, "x ^ y");
rounded_2ary!(RoundedHypot, format_hypot, hypot, "sqrt(x^2 + y^2)");
rounded_2ary!(RoundedFmod, format_fmod, fmod, "fmod(x, y)");
rounded_2ary!(
    RoundedRemainder,
    format_remainder,
    remainder,
    "remainder(x, y)"
);
rounded_2ary!(RoundedAtan2, format_atan2, atan2, "arctan(y / x)");

macro_rules! rounded_3ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output."]
            fn $imp(
                &self,
                src1: &Self::Rounded,
                src2: &Self::Rounded,
                src3: &Self::Rounded,
            ) -> Self::Rounded {
                self.$mpmf(src1, src2, src3)
            }

            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $mpmf<N1, N2, N3>(&self, src1: &N1, src2: &N2, src3: &N3) -> Self::Rounded
            where
                N1: Real,
                N2: Real,
                N3: Real;
        }
    };
}

// Traits for 3-ary operators
rounded_3ary!(RoundedFMA, format_fma, fma, "a*b + c");
