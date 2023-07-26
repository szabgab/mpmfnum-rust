// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/mod.rs
//
// Top-level of the rational module.
// Exports public functions
//

use crate::ieee754::Context;
use crate::ops::*;
use crate::rational::Rational;
use crate::{Number, RoundingContext};

macro_rules! mpfr_2ary_impl {
    ($trait:ident, $impl:ident, $name:ident, $mpfr:ident) => {
        impl $trait for $impl {
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
    };
}

mpfr_2ary_impl!(RoundedAdd, Context, add, add_with_mpfr);
mpfr_2ary_impl!(RoundedSub, Context, sub, sub_with_mpfr);
mpfr_2ary_impl!(RoundedMul, Context, mul, mul_with_mpfr);
mpfr_2ary_impl!(RoundedDiv, Context, div, div_with_mpfr);
