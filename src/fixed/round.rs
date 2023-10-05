use num_traits::Signed;
use rug::Integer;

use crate::fixed::{Exceptions, Fixed};
use crate::rational::{Rational, RationalContext};
use crate::{Number, RoundingContext, RoundingMode};

/// Fixed-point overflow behavior.
///
/// Should an unrounded number exceed the maximum number in the format,
/// the rounded value must be the next best result. In a hardware
/// implementation of fixed-point numbers, the number typically wraps,
/// preserving only the least significant bits of the implementation.
/// Alternatively, the value could be clamped to the largest representable
/// value in the representation, preserving the sign.
#[derive(Clone, Debug)]
pub enum Overflow {
    /// Values that overflow the format should be wrapped, the least
    /// significant bits preserved.
    Wrap,
    /// Clamp the representation to the largest representable value
    /// in the representation, preserving the sign.
    Saturate,
}

/// Rounding contexts for fixed-point numbers.
///
/// Fixed-point numbers are parameterized by `nbits` the total bitwidth
/// of the number and `scale` the position of the least-significant digit
/// in the format. Formats may either be signed or unsigned. The rounding
/// mode affects the rounding direction.
#[derive(Clone, Debug)]
pub struct FixedContext {
    pub(crate) signed: bool,
    pub(crate) scale: isize,
    pub(crate) nbits: usize,
    pub(crate) rm: RoundingMode,
    pub(crate) overflow: Overflow,
}

impl FixedContext {
    /// Constructs new rounding context.
    /// The default rounding mode is truncation
    /// (see [`ToZero`][crate::RoundingMode]). The default overflow
    /// behavior is saturation (see [`Saturate`][Overflow]).
    pub fn new(signed: bool, scale: isize, nbits: usize) -> Self {
        Self {
            signed,
            scale,
            nbits,
            rm: RoundingMode::ToZero,
            overflow: Overflow::Saturate,
        }
    }

    /// Sets the rounding mode of this [`FixedContext`].
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Sets the overflow behavior of this [`FixedContext`].
    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// The maximum value in format specified by this [`FixedContext`].
    /// If the format is unsigned, this is just `2^scale * 2^nbits - 1`.
    /// If the format is signed, this is just `2^scale * 2^(nbits-1) - 1`.
    pub fn maxval(&self) -> Fixed {
        if self.signed {
            let c = (Integer::from(1) << (self.nbits - 1)) - 1;
            Fixed {
                num: Rational::Real(false, self.scale, c),
                flags: Default::default(),
                ctx: self.clone(),
            }
        } else {
            let c = (Integer::from(1) << self.nbits) - 1;
            Fixed {
                num: Rational::Real(false, self.scale, c),
                flags: Default::default(),
                ctx: self.clone(),
            }
        }
    }

    /// The minimum value in a format specified by this [`FixedContext`].
    /// If the format is unsigned, this is just `0`.
    /// If the format is signed, this is just `2^scale * -2^(nbits-1)`.
    pub fn minval(&self) -> Fixed {
        if self.signed {
            Fixed {
                num: Rational::zero(),
                flags: Default::default(),
                ctx: self.clone(),
            }
        } else {
            let c = Integer::from(1) << (self.nbits - 1);
            Fixed {
                num: Rational::Real(true, self.scale, c),
                flags: Default::default(),
                ctx: self.clone(),
            }
        }
    }
}

impl FixedContext {
    fn round_wrap(&self, val: Rational) -> Rational {
        let offset = val.exp().unwrap() - self.scale;
        let div = Integer::from(1) << self.nbits;

        let c = val.c().unwrap() << offset;
        if self.signed {
            let m = if val.sign() { -c } else { c };
            let (_, wrapped) = m.div_rem(div);
            Rational::Real(wrapped.is_negative(), self.scale, wrapped.abs())
        } else {
            let (_, wrapped) = c.div_rem(div);
            Rational::Real(false, self.scale, wrapped)
        }
    }

    fn round_finite<T: Number>(&self, num: &T) -> Fixed {
        // step 1: rounding as a unbounded fixed-point number
        // so we need to compute the context parameters; we only set
        // `min_n` when rounding with a RationalContext, the first
        // digit we want to chop off.
        let (p, n) = RationalContext::new()
            .with_min_n(self.scale - 1)
            .round_params(num);

        // step 2: split the significand at binary digit `n`
        let split = RationalContext::round_prepare(num, n);
        let inexact = split.halfway_bit || split.sticky_bit;

        // step 3: finalize (fixed point)
        let rounded = RationalContext::round_finalize(split, p, self.rm);
        if !rounded.is_zero() {
            let exp = rounded.exp().unwrap();
            assert!(
                exp >= self.scale,
                "unexpected exponent, scale: {}, num: {:?}",
                self.scale,
                rounded
            );
        }

        // step 3: may need to round or saturate
        let maxval = self.maxval();
        let minval = self.minval();
        if rounded > maxval.num {
            // larger than the maxval
            Fixed {
                num: match self.overflow {
                    Overflow::Wrap => self.round_wrap(rounded),
                    Overflow::Saturate => maxval.num,
                },
                flags: Exceptions {
                    inexact,
                    overflow: true,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        } else if rounded < minval.num {
            // smaller than the minval
            Fixed {
                num: match self.overflow {
                    Overflow::Wrap => self.round_wrap(rounded),
                    Overflow::Saturate => minval.num,
                },
                flags: Exceptions {
                    inexact,
                    underflow: false,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        } else {
            Fixed {
                num: rounded,
                flags: Exceptions {
                    inexact,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        }
    }
}

impl RoundingContext for FixedContext {
    type Rounded = Fixed;

    fn round(&self, val: &Self::Rounded) -> Self::Rounded {
        self.mpmf_round(val)
    }

    fn mpmf_round<T: Number>(&self, val: &T) -> Self::Rounded {
        // case split by class
        if val.is_zero() {
            // zero is always representable
            Fixed {
                num: Rational::zero(),
                flags: Default::default(),
                ctx: self.clone(),
            }
        } else if val.is_infinite() {
            // +Inf goes to MAX
            // -Inf goes to MIN
            if val.sign() {
                self.minval()
            } else {
                self.maxval()
            }
        } else if val.is_nar() {
            // +/- NaN goes to 0
            Fixed {
                num: Rational::zero(),
                flags: Exceptions {
                    invalid: true,
                    ..Default::default()
                },
                ctx: self.clone(),
            }
        } else {
            self.round_finite(val)
        }
    }
}
