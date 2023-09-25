use crate::{
    rational::{Rational, RationalContext},
    Number, RoundingContext, RoundingMode,
};

use super::{Exceptions, Float};

/// Rounding contexts for the [`Float`] format.
///
/// Parameterized by `p`, the maximum precision for the format.
/// A rounding mode may be optionally specified (by default,
/// [`RoundingMode::NearestTiesToEven`]).
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
    fn round_finite<T: Number>(&self, num: &T) -> Float {
        // step 1: rounding as an unbounded, fixed-precision floating-point,
        // so we need to compute the context parameters; we only set
        // `max_p` when rounding with a RationalContext.
        let (p, n) = RationalContext::new()
            .with_max_precision(self.max_p())
            .round_params(num);

        // step 2: split the significand at binary digit `n`
        let split = RationalContext::round_prepare(num, n);
        let inexact = split.halfway_bit || split.sticky_bit;

        // step 3: finalize (unbounded exponent)
        let rounded = RationalContext::round_finalize(split, p, self.rm);
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

    fn round(&self, val: &Self::Rounded) -> Self::Rounded {
        self.mpmf_round(val)
    }

    fn mpmf_round<T: crate::Number>(&self, val: &T) -> Self::Rounded {
        // case split by class
        if val.is_zero() {
            Float {
                num: Rational::zero(),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if val.is_infinite() {
            Float {
                num: Rational::Infinite(val.sign()),
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else if val.is_nar() {
            Float {
                num: Rational::Nan,
                flags: Exceptions::default(),
                ctx: self.clone(),
            }
        } else {
            self.round_finite(val)
        }
    }
}
