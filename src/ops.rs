//! Rounded mathematical operations.
//!
//! Implementations of these traits operate on [`Real`] types,
//! rounding the result according to a given [`RoundingContext`].
//!

use crate::{Real, RoundingContext};

macro_rules! rounded_1ary {
    ($trait:ident, $impl:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $impl<N: Real>(&self, src: &N) -> Self::Format;
        }

        #[doc = "Computes `"]
        #[doc = $descr]
        #[doc = "` and rounds according to the [`RoundingContext`] ctx."]
        pub fn $impl<Ctx, N>(ctx: &Ctx, src: &N) -> Ctx::Format
        where
            Ctx: $trait,
            N: Real,
        {
            ctx.$impl(src)
        }
    };
}

// Traits for 1-ary operators
rounded_1ary!(RoundedNeg, neg, "-x");
rounded_1ary!(RoundedAbs, abs, "|x|");
rounded_1ary!(RoundedSqrt, sqrt, "sqrt(x)");
rounded_1ary!(RoundedCbrt, cbrt, "cbrt(x)");
rounded_1ary!(RoundedRecip, recip, "1/x");
rounded_1ary!(RoundedRecipSqrt, recip_sqrt, "1/sqrt(x)");
rounded_1ary!(RoundedExp, exp, "exp(x)");
rounded_1ary!(RoundedExp2, exp2, "2^x");
rounded_1ary!(RoundedLog, log, "ln(x)");
rounded_1ary!(RoundedLog2, log2, "log2(x)");
rounded_1ary!(RoundedLog10, log10, "log10(x)");
rounded_1ary!(RoundedExpm1, expm1, "e^x - 1");
rounded_1ary!(RoundedExp2m1, exp2m1, "2^x - 1");
rounded_1ary!(RoundedExp10m1, exp10m1, "10^x - 1");
rounded_1ary!(RoundedLog1p, log1p, "log(x + 1)");
rounded_1ary!(RoundedLog2p1, log2p1, "log2(x + 1)");
rounded_1ary!(RoundedLog10p1, log10p1, "log10(x + 1)");
rounded_1ary!(RoundedSin, sin, "sin(x)");
rounded_1ary!(RoundedCos, cos, "cos(x)");
rounded_1ary!(RoundedTan, tan, "tan(x)");
rounded_1ary!(RoundedSinPi, sin_pi, "sin(pi * x)");
rounded_1ary!(RoundedCosPi, cos_pi, "cos(pi * x)");
rounded_1ary!(RoundedTanPi, tan_pi, "tan(pi * x)");
rounded_1ary!(RoundedAsin, asin, "arcsin(x)");
rounded_1ary!(RoundedAcos, acos, "arccos(x)");
rounded_1ary!(RoundedAtan, atan, "arctan(x)");
rounded_1ary!(RoundedSinh, sinh, "sinh(x)");
rounded_1ary!(RoundedCosh, cosh, "cosh(x)");
rounded_1ary!(RoundedTanh, tanh, "tanh(x)");
rounded_1ary!(RoundedAsinh, asinh, "arsinh(x)");
rounded_1ary!(RoundedAcosh, acosh, "arcosh(x)");
rounded_1ary!(RoundedAtanh, atanh, "artanh(x)");
rounded_1ary!(RoundedErf, erf, "erf(x)");
rounded_1ary!(RoundedErfc, erfc, "erfc(x)");
rounded_1ary!(RoundedGamma, tgamma, "tgamma(x)");
rounded_1ary!(RoundedLgamma, lgamma, "lgamma(x)");

macro_rules! rounded_2ary {
    ($trait:ident, $impl:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $impl<N1, N2>(&self, src1: &N1, src2: &N2) -> Self::Format
            where
                N1: Real,
                N2: Real;
        }

        #[doc = "Computes `"]
        #[doc = $descr]
        #[doc = "` and rounds according to the [`RoundingContext`] ctx."]
        pub fn $impl<Ctx, N1, N2>(ctx: &Ctx, src1: &N1, src2: &N2) -> Ctx::Format
        where
            Ctx: $trait,
            N1: Real,
            N2: Real,
        {
            ctx.$impl(src1, src2)
        }
    };
}

// Traits for 2-ary operators
rounded_2ary!(RoundedAdd, add, "x + y");
rounded_2ary!(RoundedSub, sub, "x - y");
rounded_2ary!(RoundedMul, mul, "x * y");
rounded_2ary!(RoundedDiv, div, "x / y");
rounded_2ary!(RoundedPow, pow, "x ^ y");
rounded_2ary!(RoundedHypot, hypot, "sqrt(x^2 + y^2)");
rounded_2ary!(RoundedFmod, fmod, "fmod(x, y)");
rounded_2ary!(RoundedRemainder, remainder, "remainder(x, y)");
rounded_2ary!(RoundedAtan2, atan2, "arctan(y / x)");

macro_rules! rounded_3ary {
    ($trait:ident, $impl:ident, $descr:expr) => {
        #[doc = "Rounded `"]
        #[doc = $descr]
        #[doc = "` for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded `"]
            #[doc = $descr]
            #[doc = "`."]
            fn $impl<N1, N2, N3>(&self, src1: &N1, src2: &N2, src3: &N3) -> Self::Format
            where
                N1: Real,
                N2: Real,
                N3: Real;
        }

        #[doc = "Computes `"]
        #[doc = $descr]
        #[doc = "` and rounds according to the [`RoundingContext`] ctx."]
        pub fn $impl<Ctx, N1, N2, N3>(ctx: &Ctx, src1: &N1, src2: &N2, src3: &N3) -> Ctx::Format
        where
            Ctx: $trait,
            N1: Real,
            N2: Real,
            N3: Real,
        {
            ctx.$impl(src1, src2, src3)
        }
    };
}

// Traits for 3-ary operators
rounded_3ary!(RoundedFMA, fma, "a*b + c");
