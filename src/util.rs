use gmp_mpfr_sys::mpfr;
use rug::Integer;

/// Produces a bitmask (as an [`Integer`]) encoding `(1 << n) - 1`
/// which can be used to extract the first `n` binary digits.
pub(crate) fn bitmask(n: usize) -> Integer {
    (Integer::from(1) << n) - 1
}

#[derive(Clone, Debug)]
pub struct MPFRFlags {
    pub invalid: bool,
    pub divzero: bool,
    pub overflow: bool,
    pub underflow: bool,
    pub inexact: bool,
}

pub fn mpfr_flags() -> MPFRFlags {
    unsafe {
        let invalid = mpfr::nanflag_p() != 0;
        let divzero = mpfr::divby0_p() != 0;
        let overflow = mpfr::overflow_p() != 0;
        let inexact = mpfr::inexflag_p() != 0;
        let underflow = inexact && mpfr::underflow_p() != 0;

        MPFRFlags {
            invalid,
            divzero,
            overflow,
            underflow,
            inexact,
        }
    }
}
