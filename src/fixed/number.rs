use std::cmp::Ordering;

use rug::Integer;

use crate::fixed::FixedContext;
use crate::{rational::Rational, Number};

/// Exception flags to signal certain properties of the rounded result.
///
/// Besides returning a (possibly) numerical result, any computation
/// with fixed-point numbers may also raise exceptions depending on
/// certain conditions. This module implements two exceptions:
///
/// - _invalid operation_: no useful definable result;
/// - _overflow_: the rounded result with unbounded range
///     was larger than the maximum representable value;
/// - _underflow_: the rounded result with unbounded range
///     was smaller than the minimum representable value;
/// - _inexact_: the result would be different had both the exponent
///     range and precision been unbounded.
///
#[derive(Clone, Debug, Default)]
pub struct Exceptions {
    // defined in the IEEE 754 standard
    pub invalid: bool,
    pub overflow: bool,
    pub underflow: bool,
    pub inexact: bool,
}

/// The classic fixed-point format.
///
/// Fixed-point numbers are parameterized by `nbits` the total bitwidth
/// of the representation, `scale` the position of the least-significant
/// digit in the representation, and if it is signed.
///
/// In addition to numerical data, each [`Fixed`] value has
/// an [`Exceptions`] instance as well as a rounding context that are
/// set when the fixed-point number is created.
#[derive(Clone, Debug)]
pub struct Fixed {
    pub(crate) num: Rational,
    pub(crate) flags: Exceptions,
    pub(crate) ctx: FixedContext,
}

impl Fixed {
    /// Returns the flags set during the creation of this number
    pub fn flags(&self) -> &Exceptions {
        &self.flags
    }

    /// Returns the rounding context used to create this number.
    pub fn ctx(&self) -> &FixedContext {
        &self.ctx
    }
}

impl Number for Fixed {
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

    fn c(&self) -> Option<Integer> {
        self.num.c()
    }

    fn m(&self) -> Option<Integer> {
        self.num.m()
    }

    fn p(&self) -> usize {
        self.num.p()
    }

    fn is_nar(&self) -> bool {
        false
    }

    fn is_finite(&self) -> bool {
        true
    }

    fn is_infinite(&self) -> bool {
        false
    }

    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    fn is_negative(&self) -> Option<bool> {
        self.num.is_negative()
    }

    fn is_numerical(&self) -> bool {
        true
    }
}

impl From<Fixed> for Rational {
    fn from(val: Fixed) -> Self {
        val.num
    }
}

impl PartialEq for Fixed {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl PartialOrd for Fixed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}
