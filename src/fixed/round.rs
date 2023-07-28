use crate::fixed::Fixed;
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
    scale: usize,
    nbits: usize,
    rm: RoundingMode,
    overflow: Overflow,
}

impl Context {
    /// Constructs new rounding context.
    /// The default rounding mode is truncation
    /// (see [`ToZero`][crate::RoundingMode]). The default overflow
    /// behavior is saturation (see [`Saturate`][Overflow]).
    pub fn new(signed: bool, scale: usize, nbits: usize) -> Self {
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
