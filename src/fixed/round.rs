use rug::Integer;

use crate::fixed::Fixed;
use crate::rational::Rational;
use crate::{RoundingContext, RoundingMode};

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
pub struct Context {
    signed: bool,
    scale: isize,
    nbits: usize,
    rm: RoundingMode,
    overflow: Overflow,
}

impl Context {
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

    /// Sets the rounding mode of this [`Context`].
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Sets the overflow behavior of this [`Context`].
    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// The maximum value in format specified by this [`Context`].
    /// If the format is unsigned, this is just `2^scale * 2^nbits - 1`.
    /// If the format is signed, this is just `2^scale * 2^(nbits-1) - 1`.
    pub fn maxval(&self) -> Fixed {
        Fixed {
            num: Rational::Real(
                false,
                self.scale,
                if self.signed {
                    (Integer::from(1) << (self.nbits - 1)) - 1
                } else {
                    (Integer::from(1) << self.nbits) - 1
                },
            ),
        }
    }

    /// The minimum value in a format specified by this [`Context`].
    /// If the format is unsigned, this is just `0`.
    /// If the format is signed, this is just `2^scale * -2^(nbits-1)`.
    pub fn minval(&self) -> Fixed {
        if self.signed {
            Fixed {
                num: Rational::zero(),
            }
        } else {
            let c = Integer::from(1) << (self.nbits - 1);
            Fixed {
                num: Rational::Real(true, self.scale, c),
            }
        }
    }
}

impl RoundingContext for Context {
    type Rounded = Fixed;

    fn round(&self, val: &Self::Rounded) -> Self::Rounded {
        todo!()
    }

    fn mpmf_round<T: crate::Number>(&self, val: &T) -> Self::Rounded {
        todo!()
    }
}
