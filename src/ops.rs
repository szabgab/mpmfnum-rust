/*!
Traits for rounded mathematical operations.

Implementations of these traits operate on [`Real`] types,
rounding the result according to a given [`RoundingContext`].
*/

use crate::{Real, RoundingContext};

macro_rules! rounded_1ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output.
            Use the method prefixed by `mpmf_` if the input type differs."]
            fn $imp(&self, src: &Self::Rounded) -> Self::Rounded;

            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $mpmf<N: Real>(&self, src: &N) -> Self::Rounded;
        }
    };
}

// Traits for 1-ary operators
rounded_1ary!(RoundedNeg, neg, mpmf_neg, "-x");
rounded_1ary!(RoundedSqrt, sqrt, mpmf_sqrt, "sqrt(x)");
rounded_1ary!(RoundedCbrt, cbrt, mpmf_cbrt, "cbrt(x)");
rounded_1ary!(RoundedExp, exp, mpmf_exp, "exp(x)");
rounded_1ary!(RoundedExp2, exp2, mpmf_exp2, "2^x");
rounded_1ary!(RoundedLog, log, mpmf_log, "ln(x)");
rounded_1ary!(RoundedLog2, log2, mpmf_log2, "log2(x)");
rounded_1ary!(RoundedLog10, log10, mpmf_log10, "log10(x)");
rounded_1ary!(RoundedExpm1, expm1, mpmf_expm1, "expm1(x)");
rounded_1ary!(RoundedLog1p, log1p, mpmf_log1p, "log1p(x)");
rounded_1ary!(RoundedSin, sin, mpmf_sin, "sin(x)");
rounded_1ary!(RoundedCos, cos, mpmf_cos, "cos(x)");
rounded_1ary!(RoundedTan, tan, mpmf_tan, "tan(x)");
rounded_1ary!(RoundedAsin, asin, mpmf_asin, "arcsin(x)");
rounded_1ary!(RoundedAcos, acos, mpmf_acos, "arccos(x)");
rounded_1ary!(RoundedAtan, atan, mpmf_atan, "arctan(x)");
rounded_1ary!(RoundedSinh, sinh, mpmf_sinh, "sinh(x)");
rounded_1ary!(RoundedCosh, cosh, mpmf_cosh, "cosh(x)");
rounded_1ary!(RoundedTanh, tanh, mpmf_tanh, "tanh(x)");
rounded_1ary!(RoundedAsinh, asinh, mpmf_asinh, "arsinh(x)");
rounded_1ary!(RoundedAcosh, acosh, mpmf_acosh, "arcosh(x)");
rounded_1ary!(RoundedAtanh, atanh, mpmf_atanh, "artanh(x)");
rounded_1ary!(RoundedErf, erf, mpmf_erf, "erf(x)");
rounded_1ary!(RoundedErfc, erfc, mpmf_erfc, "erfc(x)");
rounded_1ary!(RoundedGamma, tgamma, mpmf_tgamma, "tgamma(x)");
rounded_1ary!(RoundedLgamma, lgamma, mpmf_lgamma, "lgamma(x)");

macro_rules! rounded_2ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output.
            Use the method prefixed by `mpmf_` if the input type differs."]
            fn $imp(&self, src1: &Self::Rounded, src2: &Self::Rounded) -> Self::Rounded;

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
rounded_2ary!(RoundedAdd, add, mpmf_add, "x + y");
rounded_2ary!(RoundedSub, sub, mpmf_sub, "x - y");
rounded_2ary!(RoundedMul, mul, mpmf_mul, "x * y");
rounded_2ary!(RoundedDiv, div, mpmf_div, "x / y");
rounded_2ary!(RoundedPow, pow, mpmf_pow, "x ^ y");
rounded_2ary!(RoundedHypot, hypot, mpmf_hypot, "sqrt(x^2 + y^2)");
rounded_2ary!(RoundedFmod, fmod, mpmf_fmod, "fmod(x, y)");
rounded_2ary!(
    RoundedRemainder,
    remainder,
    mpmf_remainder,
    "remainder(x, y)"
);
rounded_2ary!(RoundedAtan2, atan2, mpmf_atan2, "arctan(y / x)");

macro_rules! rounded_3ary {
    ($trait:ident, $imp:ident, $mpmf:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`. Argument is the same format as the output.
            Use the method prefixed by `mpmf_` if the input type differs."]
            fn $imp(
                &self,
                src1: &Self::Rounded,
                src2: &Self::Rounded,
                src3: &Self::Rounded,
            ) -> Self::Rounded;

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
rounded_3ary!(RoundedFMA, fma, mpmf_fma, "a*b + c");
