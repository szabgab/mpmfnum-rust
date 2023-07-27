// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ops.rs
//
// Traits for mathematical operations

use crate::{Number, RoundingContext};

macro_rules! rounded_1ary {
    ($trait:ident, $name:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded "]
            #[doc = $descr]
            #[doc = "."]
            fn $name<N: Number>(&self, src: &N) -> Self::Rounded;
        }
    };
}

// Traits for 1-ary operators
rounded_1ary!(RoundedNeg, neg, "`-x`");
rounded_1ary!(RoundedSqrt, sqrt, "`sqrt(x)`");
rounded_1ary!(RoundedCbrt, cbrt, "`cbrt(x)`");
rounded_1ary!(RoundedExp, exp, "`exp(x)`");
rounded_1ary!(RoundedExp2, exp2, "`2^x`");
rounded_1ary!(RoundedLog, log, "`ln(x)`");
rounded_1ary!(RoundedLog2, log2, "`log2(x)`");
rounded_1ary!(RoundedLog10, log10, "`log10(x)`");
rounded_1ary!(RoundedExpm1, expm1, "`expm1(x)`");
rounded_1ary!(RoundedLog1p, log1p, "`log1p(x)`");
rounded_1ary!(RoundedSin, sin, "`sin(x)`");
rounded_1ary!(RoundedCos, cos, "`cos(x)`");
rounded_1ary!(RoundedTan, tan, "`tan(x)`");
rounded_1ary!(RoundedAsin, asin, "`arcsin(x)`");
rounded_1ary!(RoundedAcos, acos, "`arccos(x)`");
rounded_1ary!(RoundedAtan, atan, "`arctan(x)`");
rounded_1ary!(RoundedSinh, sinh, "`sinh(x)`");
rounded_1ary!(RoundedCosh, cosh, "`cosh(x)`");
rounded_1ary!(RoundedTanh, tanh, "`tanh(x)`");
rounded_1ary!(RoundedAsinh, asinh, "`arsinh(x)`");
rounded_1ary!(RoundedAcosh, acosh, "`arcosh(x)`");
rounded_1ary!(RoundedAtanh, atanh, "`artanh(x)`");
rounded_1ary!(RoundedErf, erf, "`erf(x)`");
rounded_1ary!(RoundedErfc, erfc, "`erfc(x)`");
rounded_1ary!(RoundedGamma, tgamma, "`tgamma(x)`");
rounded_1ary!(RoundedLgamma, lgamma, "`lgamma(x)`");

macro_rules! rounded_2ary {
    ($trait:ident, $name:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded "]
            #[doc = $descr]
            #[doc = "."]
            fn $name<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded;
        }
    };
}

// Traits for 2-ary operators
rounded_2ary!(RoundedAdd, add, "`x + y`");
rounded_2ary!(RoundedSub, sub, "`x - y`");
rounded_2ary!(RoundedMul, mul, "`x * y`");
rounded_2ary!(RoundedDiv, div, "`x / y`");
rounded_2ary!(RoundedPow, pow, "`x ^ y`");
rounded_2ary!(RoundedHypot, hypot, "`sqrt(x^2 + y^2)`");
rounded_2ary!(RoundedFmod, fmod, "`fmod(x, y)`");
rounded_2ary!(RoundedRemainder, remainder, "`remainder(x, y)`");
rounded_2ary!(RoundedAtan2, atan2, "`arctan(y / x)`");

macro_rules! rounded_3ary {
    ($trait:ident, $name:ident, $descr:expr) => {
        #[doc = "Rounded "]
        #[doc = $descr]
        #[doc = " for rounding contexts."]
        pub trait $trait: RoundingContext {
            #[doc = "Performs rounded "]
            #[doc = $descr]
            #[doc = "."]
            fn $name<N1: Number, N2: Number, N3: Number>(
                &self,
                src1: &N1,
                src2: &N2,
                src3: &N3,
            ) -> Self::Rounded;
        }
    };
}

// Traits for 3-ary operators
rounded_3ary!(RoundedFMA, fma, "`a*b + c`");
