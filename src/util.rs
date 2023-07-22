// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// util.rs
//
// Utility functions
//

use gmp::mpz::*;

/// Produces a bitmask (as an Mpz) encoding `(1 << n) - 1`
/// which can be used to extract the first `n` binary digits.
pub(crate) fn bitmask(n: usize) -> Mpz {
    (Mpz::one() << n) - Mpz::one()
}

/// Evenness check for rounding.
/// If the significand is less than two bits,
/// than the evenness is based on the exponent.
pub(crate) fn is_even(exp: isize, c: &Mpz) -> bool {
    if c.bit_length() > 1 {
        c.tstbit(0)
    } else {
        (exp % 2) == 0
    }
}
