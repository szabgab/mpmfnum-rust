use crate::{
    rfloat::{RFloat, RFloatContext},
    Real, RoundingContext, RoundingMode, Split,
};

use super::{Exceptions, Float};

/// Rounding contexts for fixed-precision, floating-point numbers
/// with unbounded exponent.
///
/// The associated storage type is [`Float`].
///
/// This is not IEEE 754 style rounding:
/// values rounded under this context are base-2 scientific numbers
/// `(-1)^s * c * 2^exp` where `c` is a fixed-precision unsigned integer
/// and `exp` is an unbounded signed integer.
///
/// A [`FloatContext`] is parameterized by
///
///  - maximum precision (see [`Real::p`]),
///  - rounding mode.
///
/// By default, the rounding mode is [`RoundingMode::NearestTiesToEven`].
/// This rounding context is similar to the one implemented by
/// [Rug](https://docs.rs/rug/latest/rug/) (MPFR).
///
#[derive(Clone, Debug)]
pub struct FloatContext {
    prec: usize,
    rm: RoundingMode,
}

impl FloatContext {
    /// Constructs a new rounding context.
    pub fn new(prec: usize) -> Self {
        Self {
            prec,
            rm: RoundingMode::NearestTiesToEven,
        }
    }

    /// Sets the precision of this context.
    pub fn with_max_p(mut self, prec: usize) -> Self {
        self.prec = prec;
        self
    }

    /// Sets the rounding mode of this context.
    pub fn with_rm(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Returns the maximum precision allowed by this format.
    pub fn max_p(&self) -> usize {
        self.prec
    }

    /// Returns the rounding mode of this context.
    pub fn rm(&self) -> RoundingMode {
        self.rm
    }
}

impl RoundingContext for FloatContext {
    type Format = Float;

    fn round<T: Real>(&self, val: &T) -> Self::Format {
        // case split by class
        if val.is_nar() {
            Float {
                num: RFloat::Nan,
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if val.is_infinite() {
            if val.sign().unwrap() {
                Float {
                    num: RFloat::NegInfinity,
                    flags: Exceptions::default(),
                    ctx: self.clone(),
                }
            } else {
                Float {
                    num: RFloat::PosInfinity,
                    flags: Exceptions::default(),
                    ctx: self.clone(),
                }
            }
        } else if val.is_zero() {
            Float {
                num: RFloat::zero(),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            // step 1: rounding as an unbounded, fixed-precision floating-point,
            // so we need to compute the context parameters; we only set
            // `max_p` when rounding with a RFloatContext.
            let (p, n) = RFloatContext::new()
                .with_max_p(self.max_p())
                .round_params(val);

            // step 2: split the significand at binary digit `n`
            let split = Split::new(val, p, n);

            // step 3...: finalize the rounding using the split
            self.round_split(split)
        }
    }

    fn round_split(&self, split: Split) -> Self::Format {
        // step 3: extract split parameters and inexactness flag
        let inexact = !split.is_exact();
        let unrounded_e = split.e();

        // step 4: finalize (unbounded exponent)
        let rounded = RFloatContext::round_finalize(split, self.rm);

        // step 5: carry flag
        let carry = match unrounded_e {
            Some(e) => rounded.e().unwrap() > e,
            None => false,
        };

        // step 6: compose result
        Float {
            num: rounded,
            flags: Exceptions {
                inexact,
                carry,
                ..Default::default()
            },
            ctx: self.clone(),
        }
    }
}
