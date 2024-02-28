/*!
Round-to-odd arithmetic using MPFR.

The mathematical core of this crate.
Round-to-odd is a special rounding mode that allows safe
re-rounding at slightly lower precision using the standard
rounding modes.
MPFR does not support round-to-odd natively, but we can emulate it.
These functions are exported for convenience.

All computation is done using [`RFloat`] values.
*/

use gmp_mpfr_sys::mpfr;
use rug::Float;

use crate::rfloat::RFloat;
use crate::util::{mpfr_flags, MPFRFlags};

/// Result type of all mathematical functions in this crate.
#[derive(Clone, Debug)]
pub struct MPFRResult {
    num: RFloat,
    prec: usize,
    flags: MPFRFlags,
}

impl MPFRResult {
    /// Constructs an [`RTOResult`] from an MPFR result.
    pub fn new(val: Float, t: i32, flags: MPFRFlags, prec: usize) -> Self {
        let num = RFloat::from(val).with_ternary(t);
        Self { num, prec, flags }
    }

    /// The numerical result of an operation.
    pub fn num(&self) -> &RFloat {
        &self.num
    }

    /// The precision of the result.
    pub fn prec(&self) -> usize {
        self.prec
    }

    /// The MPFR flags raised by the computation.
    pub fn flags(&self) -> &MPFRFlags {
        &self.flags
    }
}

impl RFloat {
    /// Applies a correction to a [`RFloat`] type from an MPFR ternary
    /// value to translate a rounded result of precision `p - 1` obtained
    /// with round-to-zero to a rounded result of precision `p` obtained
    /// with round-to-odd.
    pub(crate) fn with_ternary(mut self, t: i32) -> Self {
        // correction only required for non-zero real values
        if let RFloat::Real(_, exp, c) = &mut self {
            if !c.is_zero() {
                // LSB is 1 iff ternary value is non-zero; else 0
                *c <<= 1;
                *exp -= 1;
                if t != 0 {
                    *c += 1;
                }
            }
        }

        self
    }
}

/// Unary RTO operations.
macro_rules! mpfr_1ary {
    ($name:ident, $mpfr:ident, $cname:expr) => {
        #[doc = "Computes `"]
        #[doc = $cname]
        #[doc = "` to `p` binary digits of precision, rounding to odd."]
        pub fn $name(src: RFloat, p: usize) -> MPFRResult {
            assert!(
                p as i64 > mpfr::PREC_MIN && p as i64 <= mpfr::PREC_MAX,
                "precision must be between {} and {}",
                mpfr::PREC_MIN + 1,
                mpfr::PREC_MAX
            );

            // compute with `p - 1` bits
            let mut dst = Float::new((p - 1) as u32);
            let src = Float::from(src);
            let (t, flags) = unsafe {
                mpfr::clear_flags();
                let t = mpfr::$mpfr(dst.as_raw_mut(), src.as_raw(), mpfr::rnd_t::RNDZ);
                (t, mpfr_flags())
            };

            // compose result
            MPFRResult::new(dst, t, flags, p)
        }
    };
}

/// Binary RTO operations.
macro_rules! mpfr_2ary {
    ($name:ident, $mpfr:ident, $cname:expr) => {
        #[doc = "Computes `"]
        #[doc = $cname]
        #[doc = "` to `p` binary digits of precision, rounding to odd."]
        pub fn $name(src1: RFloat, src2: RFloat, p: usize) -> MPFRResult {
            assert!(
                p as i64 > mpfr::PREC_MIN && p as i64 <= mpfr::PREC_MAX,
                "precision must be between {} and {}",
                mpfr::PREC_MIN + 1,
                mpfr::PREC_MAX
            );

            // compute with `p - 1` bits
            let mut dst = Float::new((p - 1) as u32);
            let src1 = Float::from(src1);
            let src2 = Float::from(src2);
            let (t, flags) = unsafe {
                mpfr::clear_flags();
                let t = mpfr::$mpfr(
                    dst.as_raw_mut(),
                    src1.as_raw(),
                    src2.as_raw(),
                    mpfr::rnd_t::RNDZ,
                );
                (t, mpfr_flags())
            };

            // compose result
            MPFRResult::new(dst, t, flags, p)
        }
    };
}

/// Ternary RTO operations.
macro_rules! mpfr_3ary {
    ($name:ident, $mpfr:ident, $cname:expr) => {
        #[doc = "Computes `"]
        #[doc = $cname]
        #[doc = "` to `p` binary digits of precision, rounding to odd."]
        pub fn $name(src1: RFloat, src2: RFloat, src3: RFloat, p: usize) -> MPFRResult {
            assert!(
                p as i64 > mpfr::PREC_MIN && p as i64 <= mpfr::PREC_MAX,
                "precision must be between {} and {}",
                mpfr::PREC_MIN + 1,
                mpfr::PREC_MAX
            );

            // compute with `p - 1` bits
            let mut dst = Float::new((p - 1) as u32);
            let src1 = Float::from(src1);
            let src2 = Float::from(src2);
            let src3 = Float::from(src3);
            let (t, flags) = unsafe {
                mpfr::clear_flags();
                let t = mpfr::$mpfr(
                    dst.as_raw_mut(),
                    src1.as_raw(),
                    src2.as_raw(),
                    src3.as_raw(),
                    mpfr::rnd_t::RNDZ,
                );
                (t, mpfr_flags())
            };

            // compose result
            MPFRResult::new(dst, t, flags, p)
        }
    };
}

// Unary operators
mpfr_1ary!(mpfr_neg, neg, "(- x)");
mpfr_1ary!(mpfr_abs, abs, "|x|");
mpfr_1ary!(mpfr_sqrt, sqrt, "sqrt(x)");
mpfr_1ary!(mpfr_cbrt, cbrt, "cbrt(x)");
mpfr_1ary!(mpfr_recip_sqrt, rec_sqrt, "1/sqrt(x)");
mpfr_1ary!(mpfr_exp, exp, "exp(x)");
mpfr_1ary!(mpfr_exp2, exp2, "2^x");
mpfr_1ary!(mpfr_exp10, exp10, "exp10(x)");
mpfr_1ary!(mpfr_log, log, "ln(x)");
mpfr_1ary!(mpfr_log2, log2, "log2(x)");
mpfr_1ary!(mpfr_log10, log10, "log10(x)");
mpfr_1ary!(mpfr_expm1, expm1, "e^x - 1");
mpfr_1ary!(mpfr_exp2m1, exp2m1, "2^x - 1");
mpfr_1ary!(mpfr_exp10m1, exp10m1, "10^x - 1");
mpfr_1ary!(mpfr_log1p, log1p, "ln(x + 1)");
mpfr_1ary!(mpfr_log2p1, log2p1, "log2(x + 1)");
mpfr_1ary!(mpfr_log10p1, log10p1, "log10(x + 1)");
mpfr_1ary!(mpfr_sin, sin, "sin(x)");
mpfr_1ary!(mpfr_cos, cos, "cos(x)");
mpfr_1ary!(mpfr_tan, tan, "tan(x)");
mpfr_1ary!(mpfr_sin_pi, sinpi, "sin(pi * x)");
mpfr_1ary!(mpfr_cos_pi, cospi, "cos(pi * x)");
mpfr_1ary!(mpfr_tan_pi, tanpi, "tan(pi * x)");
mpfr_1ary!(mpfr_asin, asin, "arcsin(x)");
mpfr_1ary!(mpfr_acos, acos, "arccos(x)");
mpfr_1ary!(mpfr_atan, atan, "arctan(x)");
mpfr_1ary!(mpfr_sinh, sinh, "sinh(x)");
mpfr_1ary!(mpfr_cosh, cosh, "cosh(x)");
mpfr_1ary!(mpfr_tanh, tanh, "tanh(x)");
mpfr_1ary!(mpfr_asinh, asinh, "arsinh(x)");
mpfr_1ary!(mpfr_acosh, acosh, "arcosh(x)");
mpfr_1ary!(mpfr_atanh, atanh, "artanh(x)");
mpfr_1ary!(mpfr_erf, erf, "erf(x)");
mpfr_1ary!(mpfr_erfc, erfc, "erfc(x)");
mpfr_1ary!(mpfr_tgamma, gamma, "tgamma(x)");
mpfr_1ary!(mpfr_lgamma, lngamma, "lgamma(x)");

// Binary operators
mpfr_2ary!(mpfr_add, add, "x + y");
mpfr_2ary!(mpfr_sub, sub, "x - y");
mpfr_2ary!(mpfr_mul, mul, "x * y");
mpfr_2ary!(mpfr_div, div, "x / y");
mpfr_2ary!(mpfr_pow, pow, "x ^ y");
mpfr_2ary!(mpfr_hypot, hypot, "sqrt(x^2 + y^2)");
mpfr_2ary!(mpfr_fmod, fmod, "fmod(x, y)");
mpfr_2ary!(mpfr_remainder, remainder, "remainder(x, y)");
mpfr_2ary!(mpfr_atan2, atan2, "arctan(y / x)");

// Ternary operators
mpfr_3ary!(mpfr_fma, fma, "a * b + c");

// Special operators

/// Computes `1/x` to `p` binary digits of precision, rounding to odd.
pub fn mpfr_recip(src: RFloat, p: usize) -> MPFRResult {
    assert!(
        p as i64 > mpfr::PREC_MIN && p as i64 <= mpfr::PREC_MAX,
        "precision must be between {} and {}",
        mpfr::PREC_MIN + 1,
        mpfr::PREC_MAX
    );

    // compute with `p - 1` bits
    let mut dst = Float::new((p - 1) as u32);
    let src = Float::from(src);
    let (t, flags) = unsafe {
        mpfr::clear_flags();
        let t = mpfr::ui_div(dst.as_raw_mut(), 1, src.as_raw(), mpfr::rnd_t::RNDZ);
        (t, mpfr_flags())
    };

    // apply correction to get the last bit and compose
    MPFRResult {
        num: RFloat::from(dst).with_ternary(t),
        prec: p,
        flags,
    }
}
