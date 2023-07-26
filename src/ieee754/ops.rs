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

impl RoundedAdd for Context {
    fn add<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded {
        // compute approximately, rounding-to-odd, with 3 rounding bits
        let p = self.max_p() + 3;
        let r1 = Rational::from_number(src1);
        let r2 = Rational::from_number(src2);
        let (result, flags) = r1.add_with_mpfr(&r2, p);
        let mut rounded = self.round(&result);
        rounded.flags.invalid = flags.invalid;
        rounded
    }
}

impl RoundedMul for Context {
    fn mul<N1: Number, N2: Number>(&self, src1: &N1, src2: &N2) -> Self::Rounded {
        // compute approximately, rounding-to-odd, with 3 rounding bits
        let p = self.max_p() + 3;
        let r1 = Rational::from_number(src1);
        let r2 = Rational::from_number(src2);
        let (result, flags) = r1.mul_with_mpfr(&r2, p);
        let mut rounded = self.round(&result);
        rounded.flags.invalid = flags.invalid;
        rounded
    }
}
