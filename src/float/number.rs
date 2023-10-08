use std::cmp::Ordering;

use crate::{rfloat::RFloat, Real};

use super::FloatContext;

/// Exception flags to signal properties of a rounded result.
///
/// Similar to IEEE 754 style exceptions, except we have
/// no constraints on exponent so we only have four exceptions:
///
/// - _invalid operation_: no useful definable result;
/// - _division by zero_: an infinite result for finite arguments;
/// - _inexact_: result would be different had both the exponent range
///     and precision been unbounded.
/// - _carry_: the exponent of the rounded result when in the form
///     `(-1)^s * c * b^exp` is different than that of the truncated result.
///     In particular, it was incremented by 1 by the rounding operation.
///
#[derive(Clone, Debug, Default)]
pub struct Exceptions {
    pub invalid: bool,
    pub divzero: bool,
    pub inexact: bool,
    pub carry: bool,
}

impl Exceptions {
    /// Constructs a new set of exceptions.
    /// All flags are set to false.
    pub fn new() -> Self {
        Self {
            invalid: false,
            divzero: false,
            inexact: false,
            carry: false,
        }
    }
}

/// The floating-point number format.
///
/// This is not an IEEE 754 style floating-point number.
/// This type defines a base-2 scientific number `(-1)^s * c * 2^e`
/// where `c` is a fixed-precision unsigned-integer and
/// `e` is theoretically unbounded  any integer
/// (In practice, this is an [`isize`] value).
///
/// Any [`Float`] value may encode a non-real number.
#[derive(Debug, Clone)]
pub struct Float {
    pub(crate) num: RFloat,
    pub(crate) flags: Exceptions,
    pub(crate) ctx: FloatContext,
}

impl Float {
    /// Return the flags set when this number was created.
    pub fn flags(&self) -> &Exceptions {
        &self.flags
    }

    /// Returns the rounding context under which this number was created.
    pub fn ctx(&self) -> &FloatContext {
        &self.ctx
    }
}

impl Real for Float {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        self.num.sign()
    }

    fn exp(&self) -> Option<isize> {
        self.num.exp()
    }

    fn e(&self) -> Option<isize> {
        self.num.e()
    }

    fn n(&self) -> Option<isize> {
        self.num.n()
    }

    fn c(&self) -> Option<rug::Integer> {
        self.num.c()
    }

    fn m(&self) -> Option<rug::Integer> {
        self.num.m()
    }

    fn p(&self) -> usize {
        self.num.p()
    }

    fn is_nar(&self) -> bool {
        self.num.is_nar()
    }

    fn is_finite(&self) -> bool {
        self.num.is_finite()
    }

    fn is_infinite(&self) -> bool {
        self.num.is_finite()
    }

    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    fn is_negative(&self) -> Option<bool> {
        self.num.is_negative()
    }

    fn is_numerical(&self) -> bool {
        self.num.is_numerical()
    }
}

impl From<Float> for RFloat {
    fn from(value: Float) -> Self {
        value.num
    }
}

impl From<Float> for rug::Float {
    fn from(value: Float) -> Self {
        rug::Float::from(value.num)
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}
