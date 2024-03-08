use std::cmp::Ordering;

use rug::Integer;

use crate::fixed::FixedContext;
use crate::{rfloat::RFloat, Real};

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
/// The associated [`RoundingContext`][crate::RoundingContext]
/// implementation is [`FixedContext`].
/// See [`FixedContext`] for more details on numerical properties
/// of the [`Fixed`] type.
///
/// A [`Fixed`] value also has an [`Exceptions`] instance to indicate
/// exceptional events that occured during its construction.
#[derive(Clone, Debug)]
pub struct Fixed {
    pub(crate) num: RFloat,
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

impl Real for Fixed {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> Option<bool> {
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

    fn prec(&self) -> Option<usize> {
        self.num.prec()
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

impl From<Fixed> for RFloat {
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
