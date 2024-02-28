use num_traits::Zero;
use rug::Integer;
use std::ops::{BitAnd, BitOr};

use crate::ieee754::{Exceptions, IEEE754Val, IEEE754};
use crate::rfloat::{RFloat, RFloatContext};
use crate::util::bitmask;
use crate::{Real, RoundingContext, RoundingDirection, RoundingMode, Split};

/// Rounding contexts for IEEE 754 floating-point numbers.
///
/// The associated storage type is [`IEEE754`].
///
/// Values rounded under this context are floating-point numbers
/// as described in the IEEE 754 standard: base 2 scientific numbers
/// `(-1)^s * c * 2^exp` where `c` is a fixed-precision unsigned integer
/// and `exp` is a signed integer with format-specific bounds.
///
/// An [`IEEE754Context`] is parameterized by
///
///  - bitwidth of the exponent field,
///  - total bitwidth of the encoding,
///  - rounding mode,
///  - optional subnormal flushing
///
/// By default, the rounding mode is [`RoundingMode::NearestTiesToEven`],
/// and subnormals are not flushed during rounding nor interpreted
/// as zero during an operation.
///
#[derive(Clone, Debug)]
pub struct IEEE754Context {
    es: usize,
    nbits: usize,
    rm: RoundingMode,
    daz: bool,
    ftz: bool,
}

impl IEEE754Context {
    /// Implementation limit: maximum exponent size
    pub const ES_MAX: usize = 32;
    /// Implementation limit: minimum exponent size
    pub const ES_MIN: usize = 2;
    /// Implementation limit: minimum precision
    pub const PREC_MIN: usize = 3;

    /// Constructs a new rounding context with the given format parameters.
    /// The default rounding mode is [`NearestTiesToEven`][RoundingMode].
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
            nbits >= es + Self::PREC_MIN,
            "total bitwidth needs to be at least {} bits, given {} bits",
            es + Self::PREC_MIN,
            nbits
        );

        Self {
            es,
            nbits,
            rm: RoundingMode::NearestTiesToEven,
            daz: false,
            ftz: false,
        }
    }

    /// Sets the rounding mode.
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Sets the subnormal argument behavior.
    /// If enabled, any subnormal argument will be interpreted as zero.
    pub fn with_daz(mut self, enable: bool) -> Self {
        self.daz = enable;
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
    /// to satisfy `2 <= self.es() < self.nbits() - 2.
    pub fn es(&self) -> usize {
        self.es
    }

    /// Returns the rounding mode of this context.
    pub fn rm(&self) -> RoundingMode {
        self.rm
    }

    /// Returns the daz (denormals-are-zero) field.
    pub fn daz(&self) -> bool {
        self.daz
    }

    /// Returns the ftz (flush-to-zero) field.
    pub fn ftz(&self) -> bool {
        self.ftz
    }

    /// Returns the total bitwidth of the format produced by this context
    /// (when viewed as a bitvector). This is guaranteed to satisfy
    /// `self.es() + 2 < self.nbits()`.
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

    /// Returns a signed zero.
    pub fn zero(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: if sign {
                IEEE754Val::NegZero
            } else {
                IEEE754Val::PosZero
            },
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }

    /// Returns the minimum representable value with a sign.
    pub fn min_float(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: IEEE754Val::Subnormal(sign, Integer::from(1)),
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }

    /// Returns the maximum representable value with a sign.
    pub fn max_float(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: IEEE754Val::Normal(sign, self.expmax(), bitmask(self.max_p())),
            flags: Exceptions::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs an infinity with a sign.
    pub fn inf(&self, sign: bool) -> IEEE754 {
        IEEE754 {
            num: if sign {
                IEEE754Val::NegInfinity
            } else {
                IEEE754Val::PosInfinity
            },
            flags: Default::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs a canonical, quiet NaN (unsigned, quiet bit, empty payload).
    pub fn qnan(&self) -> IEEE754 {
        IEEE754 {
            num: IEEE754Val::Nan(false, true, Integer::from(0)),
            flags: Default::default(),
            ctx: self.clone(),
        }
    }

    /// Constructs a canonical, signaling NaN (unsigned, signal bit, 1).
    pub fn snan(&self) -> IEEE754 {
        IEEE754 {
            num: IEEE754Val::Nan(false, false, Integer::from(1)),
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
        let e_norm = e.to_isize().unwrap() - self.emax();
        let num = if e_norm < self.emin() {
            // subnormal or zero
            if m.is_zero() {
                // zero
                if s {
                    IEEE754Val::NegZero
                } else {
                    IEEE754Val::PosZero
                }
            } else {
                // subnormal
                IEEE754Val::Subnormal(s, m)
            }
        } else if e_norm <= self.emax() {
            // normal
            let c = (Integer::from(1) << (p - 1)).bitor(m);
            let exp = e_norm - (p as isize - 1);
            IEEE754Val::Normal(s, exp, c)
        } else {
            // non-real
            if m.is_zero() {
                // infinity
                if s {
                    IEEE754Val::NegInfinity
                } else {
                    IEEE754Val::PosInfinity
                }
            } else {
                // nan
                let quiet = m.get_bit((p - 2) as u32);
                let payload = m.bitand(bitmask(p - 2));
                IEEE754Val::Nan(s, quiet, payload)
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
impl IEEE754Context {
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
    /// after rounding. The result of [`round_prepare`][crate::float::FloatIEEE754Context::round_prepare]
    /// is sufficient for computing this condition. This condition is
    /// satisfied when the rounded result would have been smaller than
    /// MIN_NORM if the exponent were unbounded (but non-zero).
    fn round_tiny<T: Real>(&self, num: &T) -> bool {
        // easy case: exact zero
        if num.is_zero() {
            // tininess requires result be non-zero
            return false;
        }

        let e_trunc = num.e().unwrap();
        match e_trunc.cmp(&(self.emin() - 1)) {
            std::cmp::Ordering::Less => {
                // far below the subnormal boundary
                true
            }
            std::cmp::Ordering::Greater => {
                // far above the subnormal boundary
                false
            }
            std::cmp::Ordering::Equal => {
                // near the subnormal boundary
                // follow the IEEE specification and round with unbounded exponent
                let unbounded_ctx = RFloatContext::new()
                    .with_rounding_mode(self.rm)
                    .with_max_p(self.max_p());
                let unbounded = unbounded_ctx.round(num);

                // tiny if below MIN_NORM
                unbounded.e().unwrap() < self.emin()
            }
        }
    }

    /// Rounding utility function: finishes the rounding procedure by
    /// checking for overflow. If overflow occurs, the rounding context
    /// decides the final numerical result. Exception flags are also
    /// set in this function.
    fn round_finalize(
        &self,
        unbounded: RFloat,
        tiny_pre: bool,
        tiny_post: bool,
        inexact: bool,
        carry: bool,
    ) -> IEEE754 {
        // all outcomes require a sign
        let sign = unbounded.sign().unwrap();

        // rounded result is zero
        if unbounded.is_zero() {
            return IEEE754 {
                num: if sign {
                    IEEE754Val::NegZero
                } else {
                    IEEE754Val::PosZero
                },
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
            if IEEE754Context::overflow_to_infinity(sign, self.rm) {
                return IEEE754 {
                    num: if sign {
                        IEEE754Val::NegInfinity
                    } else {
                        IEEE754Val::PosInfinity
                    },
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

        // check if we need flush subnormals
        if self.ftz && tiny_post {
            // flush to zero
            return IEEE754 {
                num: if sign {
                    IEEE754Val::NegZero
                } else {
                    IEEE754Val::PosZero
                },
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
        let c = unbounded.c().unwrap();
        if e < self.emin() {
            // subnormal result
            IEEE754 {
                num: IEEE754Val::Subnormal(sign, c),
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
            let exp = unbounded.exp().unwrap();
            IEEE754 {
                num: IEEE754Val::Normal(sign, exp, c),
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

impl RoundingContext for IEEE754Context {
    type Format = IEEE754;

    fn round<T: Real>(&self, num: &T) -> Self::Format {
        // case split by class
        if num.is_zero() {
            IEEE754 {
                num: match num.sign() {
                    Some(true) => IEEE754Val::NegZero,
                    _ => IEEE754Val::PosZero,
                },
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if num.is_infinite() {
            IEEE754 {
                num: match num.sign() {
                    Some(true) => IEEE754Val::NegInfinity,
                    _ => IEEE754Val::PosInfinity,
                },
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if num.is_nar() {
            let sign = num.sign().unwrap_or(false);
            IEEE754 {
                num: IEEE754Val::Nan(sign, true, Integer::zero()),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            // step 1: rounding as an unbounded, fixed-precision floating-point,
            // so we need to compute the context parameters; IEEE 754 numbers
            // support subnormalization so we need to set both `max_p` and
            // `min_n` when rounding with a RFloatContext.
            let (p, n) = RFloatContext::new()
                .with_max_p(self.max_p())
                .with_min_n(self.expmin() - 1)
                .round_params(num);

            // step 2: split the significand at binary digit `n`
            let split = Split::new(num, p, n);

            // step 3: extract split parameters and compute some exception flags
            let inexact = !split.is_exact();
            let unrounded_e = split.e();
            let (tiny_pre, tiny_post) = match unrounded_e {
                None => (false, false), // exact zero result means no tininess
                Some(e) => {
                    // need to actually compute the flags
                    let tiny_pre = e < self.emin();
                    let tiny_post = self.round_tiny(&split);
                    (tiny_pre, tiny_post)
                }
            };

            // step 4: finalize the rounding (unbounded exponent)
            let unbounded = RFloatContext::round_finalize(split, self.rm);

            // step 5: carry flag
            let carry = match (unrounded_e, unbounded.e()) {
                (Some(e1), Some(e2)) => e2 > e1,
                (_, _) => false,
            };

            // step 6: finalize the rounding (bounded exponent)
            self.round_finalize(unbounded, tiny_pre, tiny_post, inexact, carry)
        }
    }

    // fn format_round(&self, val: &Self::Format) -> Self::Format {
    //     match &val.num {
    //         IEEE754Val::Zero(s) => {
    //             // +/-0 is preserved
    //             IEEE754 {
    //                 num: IEEE754Val::Zero(*s),
    //                 flags: Default::default(),
    //                 ctx: self.clone(),
    //             }
    //         }
    //         IEEE754Val::Infinity(s) => {
    //             // +/-Inf is preserved
    //             IEEE754 {
    //                 num: IEEE754Val::Infinity(*s),
    //                 flags: Default::default(),
    //                 ctx: self.clone(),
    //             }
    //         }
    //         IEEE754Val::Nan(s, _, payload) => {
    //             // NaN
    //             // rounding truncates the payload
    //             // always quiets the result
    //             let offset = self.max_p() as isize - val.ctx.max_p() as isize;
    //             let payload = match offset.cmp(&0) {
    //                 std::cmp::Ordering::Less => {
    //                     // truncation: chop off the lower bits
    //                     Integer::from(payload >> -offset)
    //                 }
    //                 std::cmp::Ordering::Greater => {
    //                     // padding
    //                     Integer::from(payload << offset)
    //                 }
    //                 std::cmp::Ordering::Equal => {
    //                     // payload is preserved exactly
    //                     payload.clone()
    //                 }
    //             };

    //             IEEE754 {
    //                 num: IEEE754Val::Nan(*s, true, payload),
    //                 flags: Default::default(),
    //                 ctx: self.clone(),
    //             }
    //         }
    //         _ => {
    //             // finite, non-zero
    //             self.round_finite(val)
    //         }
    //     }
    // }
}
