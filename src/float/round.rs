use crate::{
    rfloat::{RFloat, RFloatContext},
    Real, RoundingContext, RoundingMode,
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
    pub fn with_prec(mut self, prec: usize) -> Self {
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

impl FloatContext {
    fn round_finite<T: Real>(&self, num: &T) -> Float {
        // step 1: rounding as an unbounded, fixed-precision floating-point,
        // so we need to compute the context parameters; we only set
        // `max_p` when rounding with a RFloatContext.
        let (p, n) = RFloatContext::new()
            .with_max_p(self.max_p())
            .round_params(num);

        // step 2: split the significand at binary digit `n`
        let split = RFloatContext::round_prepare(num, n);
        let inexact = split.halfway_bit || split.sticky_bit;

        // step 3: finalize (unbounded exponent)
        let rounded = RFloatContext::round_finalize(split, p, self.rm);
        let carry = matches!((num.e(), rounded.e()), (Some(e1), Some(e2)) if e2 > e1);

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

impl RoundingContext for FloatContext {
    type Rounded = Float;

    fn round<T: Real>(&self, val: &T) -> Self::Rounded {
        // case split by class
        if val.is_zero() {
            Float {
                num: RFloat::zero(),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if val.is_infinite() {
            Float {
                num: RFloat::Infinite(val.sign()),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if val.is_nar() {
            Float {
                num: RFloat::Nan,
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            self.round_finite(val)
        }
    }
}
