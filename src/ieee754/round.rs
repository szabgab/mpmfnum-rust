use std::ops::{BitAnd, BitOr};

use num_traits::Zero;
use rug::Integer;

use crate::ieee754::{Exceptions, Float, IEEE754};
use crate::rational::{self, Rational};
use crate::round::RoundingDirection;
use crate::util::bitmask;
use crate::{Number, RoundingContext, RoundingMode};

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
    /// Implementation limit: maximum exponent size
    pub const ES_MAX: usize = 32;
    /// Implementation limit: minimum exponent size
    pub const ES_MIN: usize = 2;
    /// Implementation limit: minimum precision
    pub const PREC_MIN: usize = 3;

    /// Constructs a new rounding context with the given format parameters.
    /// The default rounding mode is [`RoundingMode::NearestTiesToEven`].
    /// Both fields specifying subnormal behavior are false by default.
    pub fn new(es: usize, nbits: usize) -> Self {
        assert!(
            es >= Self::ES_MIN,
            "exponent width needs to be at least {} bits, given {} bits",
            Self::ES_MIN,
            es
        );
        assert!(
            es <= Self::ES_MAX,
            "exponent width needs to be at most {} bits, given {} bits",
            Self::ES_MAX,
            es
        );
        assert!(
            nbits >= es + 3,
            "total bitwidth needs to be at least {} bits, given {} bits",
            es + 3,
            nbits
        );
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

    /// Sets the subnormal argument behavior.
    /// If enabled, any subnormal argument will be flushed to zero.
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
    /// is an integer. The result is just `self.emin() - self.max_m()`.
    pub fn expmin(&self) -> isize {
        self.emin() - (self.max_m() as isize)
    }

    /// The exponent "bias" used when converting a valid exponent range
    /// `[emin, emax]` to unsigned integers for bitpacking. Specifically,
    /// the final range is `[1, 2*emax]` The result is just `self.emax()`.
    pub fn bias(&self) -> isize {
        self.emax()
    }

    /// Returns the rounding mode of this context.
    pub fn rm(&self) -> RoundingMode {
        self.rm
    }

    /// Returns the minimum representable value with a sign.
    pub fn min_float(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: Float::Subnormal(sign, Integer::from(1)),
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }

    /// Returns the maximum representable value with a sign.
    pub fn max_float(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: Float::Normal(sign, self.expmax(), bitmask(self.max_p())),
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs an infinity with a sign.
    pub fn inf(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: Float::Infinity(sign),
            flags: Default::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs a canonical, quiet NaN (unsigned, quiet bit, empty payload).
    pub fn qnan(&self) -> IEEE754 {
        IEEE754 {
            num: Float::Nan(false, true, Integer::from(0)),
            flags: Default::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs a canonical, signaling NaN (unsigned, signal bit, 1).
    pub fn snan(&self) -> IEEE754 {
        IEEE754 {
            num: Float::Nan(false, false, Integer::from(1)),
            flags: Default::default(),
            ctx: self.clone(),
        }
    }

    /// Converts an [`Integer`] representing an IEEE 754 bitpattern
    /// into an [`IEEE754`] type.
    pub fn bits_to_number(&self, b: Integer) -> IEEE754 {
        let p = self.nbits - self.es;
        let limit = Integer::from(1) << self.nbits;
        assert!(b < limit, "must be less than 1 << nbits");

        // decompose into bitfields
        let s = b.get_bit((self.nbits - 1) as u32);
        let e = (b.clone() >> (p - 1)).bitand(bitmask(self.es));
        let m = b.bitand(bitmask(p - 1));

        // case split by classification
        let e_norm = e - self.emax();
        let num = if e_norm < self.emin() {
            // subnormal or zero
            if m.is_zero() {
                // zero
                Float::Zero(s)
            } else {
                // subnormal
                Float::Subnormal(s, m)
            }
        } else if e_norm <= self.emax() {
            // normal
            let c = (Integer::from(1) << (p - 1)).bitor(m);
            let exp = e_norm.to_isize().unwrap() - (p as isize - 1);
            Float::Normal(s, exp, c)
        } else {
            // non-real
            if m.is_zero() {
                // infinity
                Float::Infinity(s)
            } else {
                // nan
                let quiet = m.get_bit((p - 2) as u32);
                let payload = m.bitand(bitmask(p - 2));
                Float::Nan(s, quiet, payload)
            }
        };

        IEEE754 {
            num,
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

    /// Rounding utility function: returns true if the result will be tiny
    /// after rounding. The result of [`rational::Context::round_prepare`]
    /// is sufficient for computing this condition. This condition is
    /// satisfied when the rounded result would have been smaller than
    /// MIN_NORM if the exponent were unbounded (but non-zero).
    fn round_tiny(&self, split: &rational::RoundPrepareResult) -> bool {
        let trunc = Rational::Real(split.sign, split.exp, split.c.clone());
        let halfway_bit = split.halfway_bit;
        let quarter_bit = split.quarter_bit;
        let sticky_bit = split.sticky_bit;
        let inexact = halfway_bit || quarter_bit || sticky_bit;

        if trunc.is_zero() && inexact {
            // exact zero
            return false;
        }

        let e_trunc = trunc.e().unwrap();
        if e_trunc + 1 < self.emin() {
            // far below the subnormal boundary
            false
        } else if e_trunc + 1 > self.emin() {
            // far above the subnormal boundary
            true
        } else {
            // near the subnormal boundary

            // only care if we are between TINY_VAL and MIN_NORM
            // input has mantissa    `xx...xx|xx`
            // TINY_VAL has mantissa `01...11|10`
            let tiny_val = bitmask(self.max_p()) << 1;
            if trunc.c().unwrap() < tiny_val {
                // below TINY_VAL, so definitely tiny
                true
            } else {
                // need to check the rounding bits to resolve
                let low_bits = quarter_bit || sticky_bit;

                // case split on rounding mode
                match self.rm.to_direction(trunc.sign()) {
                    (true, _) => {
                        // nearest modes:
                        // tie will always round up to MIN_NORM
                        !halfway_bit || !quarter_bit
                    }
                    (_, RoundingDirection::ToZero) => {
                        // rounding always goes to MAX_SUB rather
                        // than TINY_VAL
                        true
                    }
                    (_, RoundingDirection::AwayZero) => {
                        // exactly halfway would round to TINY_VAL rather
                        // than MAX_NORM
                        !halfway_bit || !low_bits
                    }
                    (_, RoundingDirection::ToEven) => {
                        // MIN_NORM and MAX_SUB have even lsbs:
                        // all values except TINY_VAL round to either
                        // MIN_NORM or MAX_SUB
                        !halfway_bit || !low_bits
                    }
                    (_, RoundingDirection::ToOdd) => {
                        // MIN_NORM and MAX_SUB have even lsbs:
                        // all values except MAX_SUB round to TINY_VAL
                        true
                    }
                }
            }
        }
    }

    /// Rounding utility function: finishes the rounding procedure by
    /// checking for overflow. If overflow occurs, the rounding context
    /// decides the final numerical result. Exception flags are also
    /// set in this function.
    fn round_finalize(
        &self,
        unbounded: Rational,
        tiny_pre: bool,
        tiny_post: bool,
        inexact: bool,
    ) -> IEEE754 {
        // rounded result is zero
        if unbounded.is_zero() {
            return IEEE754 {
                num: Float::Zero(unbounded.sign()),
                flags: Exceptions {
                    underflow_pre: tiny_pre && inexact,
                    underflow_post: tiny_post && inexact,
                    inexact,
                    tiny_pre,
                    tiny_post,
                    ..Default::default()
                },
                ctx: self.clone(),
            };
        }

        // check for overflow
        let e = unbounded.e().unwrap();
        if e > self.emax() {
            let sign = unbounded.sign();
            if Context::overflow_to_infinity(sign, self.rm) {
                return IEEE754 {
                    num: Float::Infinity(sign),
                    flags: Exceptions {
                        overflow: true,
                        inexact: true,
                        ..Default::default()
                    },
                    ctx: self.clone(),
                };
            } else {
                let mut maxfloat = self.max_float(sign);
                maxfloat.flags.overflow = true;
                maxfloat.flags.inexact = true;
                return maxfloat;
            }
        }

        let carry = false;

        // check if we need flush subnormals
        if self.ftz && tiny_post {
            // flush to zero
            return IEEE754 {
                num: Float::Zero(unbounded.sign()),
                flags: Exceptions {
                    underflow_pre: true,
                    underflow_post: true,
                    inexact: true,
                    tiny_pre: true,
                    tiny_post: true,
                    ..Default::default()
                },
                ctx: self.clone(),
            };
        }

        // normal or subnormal result
        if e < self.emin() {
            // subnormal result
            let sign = unbounded.sign();
            let c = unbounded.c().unwrap();
            IEEE754 {
                num: Float::Subnormal(sign, c),
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
            let sign = unbounded.sign();
            let exp = unbounded.exp().unwrap();
            let c = unbounded.c().unwrap();
            IEEE754 {
                num: Float::Normal(sign, exp, c),
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

    /// Rounds a finite (non-zero) number.
    fn round_finite<T: Number>(&self, num: &T) -> IEEE754 {
        // step 1: rounding as a fixed-precision rational number first,
        // so we need to compute the context parameters; IEEE 754 numbers
        // support subnormalization so we need to set both `max_p` and
        // `min_n` when rounding using the rational number rounding context.
        let (p, n) = rational::Context::new()
            .with_rounding_mode(self.rm)
            .with_max_precision(self.nbits - self.es)
            .with_min_n(self.expmin() - 1)
            .round_params(num);

        // step 2: split the significand at binary digit `n`
        let split = rational::Context::round_prepare(num, n);

        // step 3: compute certain exception flags
        let inexact = split.halfway_bit || split.quarter_bit || split.sticky_bit;
        let (tiny_pre, tiny_post) = if num.is_zero() {
            // exact zero result means no tininess
            (false, false)
        } else {
            // need to actually compute the flags
            let e_trunc = num.e().unwrap();
            let tiny_pre = e_trunc < self.emin();
            let tiny_post = self.round_tiny(&split);
            (tiny_pre, tiny_post)
        };

        // step 4: finalize the rounding (unbounded exponent)
        let unbounded = rational::Context::round_finalize(split, p, self.rm);

        // step 5: finalize the rounded (bounded exponent)
        self.round_finalize(unbounded, tiny_pre, tiny_post, inexact)
    }
}

impl RoundingContext for Context {
    type Rounded = IEEE754;

    /// Rounds an [`IEEE754`] value into the format specified by
    /// this rounding context. See [`RoundingContext::round`] for the more
    /// general implementation of rounding from formats other than the
    /// output format.
    fn round(&self, val: &Self::Rounded) -> Self::Rounded {
        match &val.num {
            Float::Zero(s) => {
                // +/-0 is preserved
                IEEE754 {
                    num: Float::Zero(*s),
                    flags: Default::default(),
                    ctx: self.clone(),
                }
            }
            Float::Infinity(s) => {
                // +/-Inf is preserved
                IEEE754 {
                    num: Float::Infinity(*s),
                    flags: Default::default(),
                    ctx: self.clone(),
                }
            }
            Float::Nan(s, _, payload) => {
                // NaN
                // rounding truncates the payload
                // always quiets the result
                let offset = self.max_p() as isize - val.ctx.max_p() as isize;
                let payload = match offset.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        // truncation: chop off the lower bits
                        Integer::from(payload >> -offset)
                    }
                    std::cmp::Ordering::Greater => {
                        // padding
                        Integer::from(payload << offset)
                    }
                    std::cmp::Ordering::Equal => {
                        // payload is preserved exactly
                        payload.clone()
                    }
                };

                IEEE754 {
                    num: Float::Nan(*s, true, payload),
                    flags: Default::default(),
                    ctx: self.clone(),
                }
            }
            _ => {
                // finite, non-zero
                self.round_finite(val)
            }
        }
    }

    fn mpmf_round<T: Number>(&self, num: &T) -> Self::Rounded {
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
                num: Float::Nan(num.sign(), true, Integer::from(0)),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            self.round_finite(num)
        }
    }
}
