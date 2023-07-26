// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// util.rs
//
// Utility functions
//

use rug::Integer;
use gmp_mpfr_sys::mpfr;

/// Produces a bitmask (as an Mpz) encoding `(1 << n) - 1`
/// which can be used to extract the first `n` binary digits.
pub(crate) fn bitmask(n: usize) -> Integer {
    (Integer::from(1) << n) - 1
}

/// Evenness check for rounding.
/// If the significand is less than two bits, than the evenness
/// is based on the exponent.
pub(crate) fn is_even(exp: isize, c: &Integer) -> bool {
    if c.significant_bits() > 1 {
        !c.get_bit(0)
    } else {
        (exp % 2) == 0
    }
}

pub struct MPFRFlags {
    pub invalid: bool,
    pub divzero: bool,
    pub overflow: bool,
    pub underflow: bool,
    pub inexact: bool
}

pub fn mpfr_flags() -> MPFRFlags {
    unsafe {
        let invalid = mpfr::nanflag_p() != 0;
        let divzero = mpfr::divby0_p() != 0;
        let overflow = mpfr::overflow_p() != 0;
        let inexact = mpfr::inexflag_p() != 0;
        let underflow = inexact && mpfr::underflow_p() != 0;

        MPFRFlags { invalid, divzero, overflow, underflow, inexact }
    }
}
