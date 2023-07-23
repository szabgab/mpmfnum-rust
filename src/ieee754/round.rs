// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/round.rs
//
// IEEE 754 floating-point rounding

use std::cmp::max;

use gmp::mpz::Mpz;

use crate::ieee754::{Exceptions, Float, IEEE754};
use crate::rational::{self, RoundingDirection, RoundingMode};
use crate::util::bitmask;
use crate::{Number, RoundingContext};

/// Rounding contexts for IEEE 754 floating-point numbers.
/// Must define format parameters `es` and `nbits` (see [`IEEE754`]
/// for a description of these fields). The rounding mode
/// affects the rounding direction. The `dtz` and `ftz` fields
/// specify subnormal handling specifically before an operation
/// `dtz` and after rounding `ftz`.
#[derive(Clone, Debug)]
pub struct Context {
    es: usize,
    nbits: usize,
    rm: RoundingMode,
    dtz: bool,
    ftz: bool,
}

impl Context {
    /// Constructs a new rounding context with the given format parameters.
    /// The default rounding mode is [`RoundingMode::NearestTiesToEven`].
    /// Both fields specifying subnormal behavior are false by default.
    pub fn new(es: usize, nbits: usize) -> Self {
        Self {
            es,
            nbits,
            rm: RoundingMode::NearestTiesToEven,
            dtz: false,
            ftz: false,
        }
    }

    /// Sets the rounding mode.
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Sets the denormal argument behavior.
    /// If enabled, a denormal argument will be interpreted as 0.
    pub fn with_dtz(mut self, enable: bool) -> Self {
        self.dtz = enable;
        self
    }

    /// Sets the subnormal result behavior.
    /// If enabled, any subnormal result will be flushed to zero.
    pub fn with_ftz(mut self, enable: bool) -> Self {
        self.ftz = enable;
        self
    }

    /// Returns the exponent bitwidth of the format produced by
    /// this context (when viewed as a bitvector). This is guaranteed
    /// to satisfy `2 <= self.es() <= self.nbits() - 2. Exponent
    /// overflowing will likely occur past 60 bits, but MPFR generally
    /// has a limit at 31 bits.
    pub fn es(&self) -> usize {
        self.es
    }

    /// Returns the total bitwidth of the format produced by this context
    /// (when viewed as a bitvector). This is guaranteed to satisfy
    /// `self.es() + 2 <= self.nbits()`.
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Returns the maximum precision allowed by this format.
    /// The result is always `self.nbits() - self.es()`.
    pub fn max_p(&self) -> usize {
        self.nbits - self.es
    }

    /// Returns the maximum significand width allowed by this format
    /// (when viewed as a bitvector) The result is always `self.max_p() - 1`.
    pub fn max_m(&self) -> usize {
        self.nbits - self.es - 1
    }

    /// Exponent of the largst finite floating-point value representable
    /// in this format when viewed as `(-1)^s * m * b^e` where `m`
    /// is a fraction between 1 and 2.
    pub fn emax(&self) -> isize {
        (1 << (self.es - 1)) - 1
    }

    /// Exponent of the smallest normal floating-point value representable
    /// in this format when viewed as `(-1)^s * m * b^e` where `m`
    /// is a fraction between 1 and 2. The result is just `self.emax() - 1`.
    pub fn emin(&self) -> isize {
        1 - self.emax()
    }

    /// Exponent of the largst finite floating-point value representable
    /// in this format when viewed as `(-1)^s * c * b^e` where `c`
    /// is an integer. The result is just `self.emax() - self.max_m()`
    pub fn expmax(&self) -> isize {
        self.emax() - (self.max_m() as isize)
    }

    /// Exponent of the smallest normal floating-point value representable
    /// in this format when viewed as `(-1)^s * c * b^e` where `c`
    /// is an integer. The result is just `self.emin() - self.max_m()`
    /// `self.emax() - 1`.
    pub fn expmin(&self) -> isize {
        self.emin() - (self.max_m() as isize)
    }

    /// The exponent "bias" used when converting a valid exponent range
    /// `[emin, emax]` to unsigned integers for bitpacking. Specifically,
    /// the final range is `[1, 2*emax]` The result is just `self.emax()`.
    pub fn bias(&self) -> isize {
        self.emax()
    }

    /// Returns the maximum representable value.
    pub fn max_float(&self) -> IEEE754 {
        IEEE754 {
            num: Float::Normal(false, self.expmax(), bitmask(self.max_p() + 1)),
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }
}

// Rounding utility functions.
impl Context {
    /// Given a sign and rounding mode, returns true if a overflow
    /// exception means the result is rounded to infinity rather
    /// than MAX_FLOAT.
    fn overflow_to_infinity(sign: bool, rm: RoundingMode) -> bool {
        // case split on rounding mode
        match rm.to_direction(sign) {
            (true, _) => true,
            (_, RoundingDirection::ToZero) => false, // always truncate
            (_, RoundingDirection::AwayZero) => true, // always away
            (_, RoundingDirection::ToEven) => true,  // MAX_FLOAT has odd LSB
            (_, RoundingDirection::ToOdd) => false,  // MAX_FLOAT has odd LSB
        }
    }

    /// Returns true if the result will be tiny after rounding.
    /// This condition is satisfied when the result is at least as small
    /// as the minimum normal value and the rounded result is different
    /// than if rounding were done with unbounded exponent. Assumes
    /// `c_trunc` has at most `p+2` binary digits, so the half bit and
    /// quarter bits are the least significand digits of `c_trunc`.
    fn round_tiny(&self, sign: bool, e_trunc: isize, c_trunc: &Mpz, lost: &Mpz) -> bool {
        if c_trunc.is_zero() && lost.is_zero() {
            // easy case: exact zero
            false
        } else if e_trunc + 1 < self.emin() {
            // truncated result is far below the subnormal boundary
            true
        } else if e_trunc + 1 == self.emin() {
            // c_trunc is of the form `xx...xx|xx`
            // TINY_VAL has mantissa `01...11|10`
            let tiny_val = bitmask(self.max_p()) << 1;
            if *c_trunc < tiny_val {
                // less than TINY_VAL, so below the subnormal boundary
                true
            } else {
                // rounding bits
                let half_bit = c_trunc.tstbit(1);
                let quarter_bit = c_trunc.tstbit(0);
                let sticky_bit = !lost.is_zero();
                let low_bits = quarter_bit || sticky_bit;

                // case split on rounding mode
                match self.rm.to_direction(sign) {
                    (true, _) => {
                        // nearest modes:
                        // tie will always round up to MIN_NORM
                        !half_bit || !quarter_bit
                    }
                    (_, RoundingDirection::ToZero) => {
                        // rounding always goes to MAX_SUB rather
                        // than TINY_VAL
                        true
                    }
                    (_, RoundingDirection::AwayZero) => {
                        // exactly halfway would round to TINY_VAL rather
                        // than MAX_NORM
                        !half_bit || !low_bits
                    }
                    (_, RoundingDirection::ToEven) => {
                        // MIN_NORM and MAX_SUB have even lsbs:
                        // all values except TINY_VAL round to either
                        // MIN_NORM or MAX_SUB
                        !half_bit || !low_bits
                    }
                    (_, RoundingDirection::ToOdd) => {
                        // MIN_NORM and MAX_SUB have even lsbs:
                        // all values except MAX_SUB round to TINY_VAL
                        true
                    }
                }
            }
        } else {
            // truncated result is at least MIN_NORM,
            // so subnormalization will not affect the result
            false
        }
    }

    /// Rounds a finite (non-zero) number.
    fn round_finite<T: Number>(&self, num: &T) -> IEEE754 {
        // step 1: rounding as a fixed-precision rational number
        // first, so we need to compute the context parameters.
        // IEEE 754 numbers support subnormalization so we need
        // to set both `max_p` and `min_n` when rounding using the
        // rational number rounding context.
        let max_p = self.nbits - self.es;
        let unbounded_n = num.exp().unwrap() - 1;
        let n = max(unbounded_n, self.expmin() - 1);

        // step 2: round and collect the lost bits
        let rctx = rational::Context::new()
            .with_rounding_mode(self.rm)
            .with_max_precision(max_p)
            .with_min_n(n);
        let (rounded, lost) = rctx.round_residual(num);

        // rounding components
        let sign = num.sign();
        let e = rounded.e().unwrap();
        let inexact = !lost.unwrap().is_zero();

        // step 3: check for overflow and possibly clamp exponent
        if e > self.emax() {
            let to_inf = Context::overflow_to_infinity(sign, self.rm);
            return IEEE754 {
                num: if to_inf {
                    // rounding says to overflow to +Inf
                    Float::Infinity(sign)
                } else {
                    Float::Zero(false)
                },
                flags: Exceptions {
                    overflow: true,
                    inexact: true,
                    carry: true,
                    ..Default::default()
                },
                ctx: self.clone(),
            };
        }

        // step 4: check for underflow after rounding
        // split again but with 2 more digits in the significant part:
        // the halfway and quarter bit are the least significant parts of `c_trunc`
        // and the lower rounding bits are contained in `lost`.
        let (exp_trunc, c_trunc, lost, _, _) = rational::Context::split(num, n - 2);
        let e_trunc = exp_trunc + c_trunc.bit_length() as isize - 1;

        let tiny_pre = e_trunc < self.emin();
        let tiny_post = self.round_tiny(sign, e_trunc, &c_trunc, &lost);
        let carry = e > e_trunc;

        // step 5: compose result
        if self.ftz && tiny_post {
            // flush to zero
            IEEE754 {
                num: Float::Zero(sign),
                flags: Exceptions {
                    underflow_pre: true,
                    underflow_post: true,
                    inexact: true,
                    tiny_pre: true,
                    tiny_post: true,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        } else if e < self.emin() {
            // subnormal result
            IEEE754 {
                num: Float::Subnormal(sign, rounded.c().unwrap()),
                flags: Exceptions {
                    underflow_pre: tiny_pre && inexact,
                    underflow_post: tiny_post && inexact,
                    inexact,
                    tiny_pre,
                    tiny_post,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        } else {
            // normal result
            IEEE754 {
                num: Float::Normal(sign, rounded.exp().unwrap(), rounded.c().unwrap()),
                flags: Exceptions {
                    underflow_pre: tiny_pre && inexact,
                    underflow_post: tiny_post && inexact,
                    inexact,
                    carry,
                    tiny_pre,
                    tiny_post,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        }
    }
}

impl RoundingContext for Context {
    type Rounded = IEEE754;

    fn round<T: Number>(&self, num: &T) -> IEEE754 {
        // case split by class
        if num.is_zero() {
            IEEE754 {
                num: Float::Zero(num.sign()),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if num.is_infinite() {
            IEEE754 {
                num: Float::Infinity(num.sign()),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if num.is_nar() {
            IEEE754 {
                num: Float::Nan(num.sign(), true, Mpz::from(0)),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            self.round_finite(num)
        }
    }
}
