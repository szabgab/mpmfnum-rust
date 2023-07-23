// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/types.rs
//
// The IEEE 754 floating-point type

use gmp::mpz::Mpz;

use crate::ieee754::Context;
use crate::{rational::Rational, Number};

/// Exception flags to signal certain properties of the rounded result.
///
/// Besides returning a (possibly) numerical result, any computation with
/// floating-point numbers may also raise exceptions depending on certain conditions.
/// The IEEE 754 standard defines five such exceptions:
///
/// - _invalid operation_: no useful definable result;
/// - _division by zero_: an infinite result for finite arguments;
/// - _overflow_: result exceeded in magnitude what would have been the rounded result
///     had the exponent range been unbounded;
/// - _underflow_: non-zero result that either (a) would lie strictly between
///     `-b^emin` and `+b^emin` had the exponent range been unbounded,
///     or (b) would lie strictly between `-b^emin` and `+b^emin`
///     had the exponent range and precision been unbounded; this flag
///     may only be raised if the result is also inexact;
/// - _inexact_: result would be different had both the exponent range
///     and precision been unbounded.
///
/// These flags are contains in each Exceptions instance with similar names.
/// Note that both definitions of underflow are provided with (a) corresponding
/// to `underflow_post` and (b) corresponding to `underflow_pre`.
///
/// The [`Exceptions`] type defines additional exceptions:
///
/// - _carry_: the exponent of the rounded result when in the form
///     `(-1)^s * c * b^exp` is different than that of the truncated result.
///     In particular, it was incremented by 1 by the rounding operation.
///     This flag will not be raised if the final result is subnormal.
/// - _denorm_: at least one argument to a particular operation was
///     subnormal (see Section 4.9.1.2 of the Intel(R) 64 and IA-32
///     Architectures Developer's Manual: Vol. 1, June 2023).
/// - _tiny_pre_: similar to the `underflow_pre` flag except this flag
///     will be raised regardless of the state of the `inexact` flag,
///     i.e., `underflow_pre = tiny_pre && inexact`.
/// - _tiny_post_: similar to the `underflow_post` flag except this flag
///     will be raised regardless of the state of the `inexact` flag
///     i.e., `underflow_post = tiny_post && inexact`.
///
#[derive(Clone, Debug, Default)]
pub struct Exceptions {
    // defined in the IEEE 754 standard
    pub invalid: bool,
    pub divzero: bool,
    pub overflow: bool,
    pub underflow_pre: bool,
    pub underflow_post: bool,
    pub inexact: bool,
    // non-standard flags
    pub carry: bool,
    pub denorm: bool,
    pub tiny_pre: bool,
    pub tiny_post: bool,
}

/// IEEE 754 floating-point bitwise encoding viewed as an enumeration.
/// Unlike [`IEEE754`], [`Float`] contains only the numerical data
/// required to bitvector that encodes a binary floating-point number
/// as described by the IEEE 754 standard.
#[derive(Clone, Debug)]
pub enum Float {
    // zero (+/-)
    // => (sign)
    Zero(bool),
    // subnormal numbers
    // => (sign, significand)
    Subnormal(bool, Mpz),
    // signed zero or finite number
    // => (sign, exponent, significand)
    Normal(bool, isize, Mpz),
    // infinity (+/-)
    // => (sign)
    Infinity(bool),
    // not-a-number
    // => (sign, quiet, payload)
    Nan(bool, bool, Mpz),
}

/// The IEEE 754 floating-point type.
///
/// Parameterized by `es`, the bitwidth of the exponent field, and `nbits`,
/// the total bitwidth of the floating-point encoding. In addition to
/// numerical data, each [`IEEE754`] value has an [`Exceptions`] instance
/// as well as a rounding context that are set when the floating-point
/// number is created.
#[derive(Clone, Debug)]
pub struct IEEE754 {
    pub num: Float,
    pub flags: Exceptions,
    pub ctx: Context,
}

impl IEEE754 {
    /// Returns the flags set during the creation of this number
    pub fn flags(&self) -> &Exceptions {
        &self.flags
    }

    /// Returns the rounding context used to create this number.
    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
}

impl Number for IEEE754 {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        match &self.num {
            Float::Zero(s) => *s,
            Float::Subnormal(s, _) => *s,
            Float::Normal(s, _, _) => *s,
            Float::Infinity(s) => *s,
            Float::Nan(s, _, _) => *s,
        }
    }

    fn exp(&self) -> Option<isize> {
        match &self.num {
            Float::Zero(_) => None,
            Float::Subnormal(_, _) => Some(self.ctx().expmin()),
            Float::Normal(_, exp, _) => Some(*exp),
            Float::Infinity(_) => None,
            Float::Nan(_, _, _) => None,
        }
    }

    fn e(&self) -> Option<isize> {
        match &self.num {
            Float::Zero(_) => None,
            Float::Subnormal(_, c) => Some((self.ctx().expmin() - 1) + (c.bit_length() as isize)),
            Float::Normal(_, exp, c) => Some((*exp - 1) + (c.bit_length() as isize)),
            Float::Infinity(_) => None,
            Float::Nan(_, _, _) => None,
        }
    }

    fn n(&self) -> Option<isize> {
        match &self.num {
            Float::Zero(_) => None,
            Float::Subnormal(_, _) => Some(self.ctx().expmin() - 1),
            Float::Normal(_, exp, _) => Some(exp - 1),
            Float::Infinity(_) => None,
            Float::Nan(_, _, _) => None,
        }
    }

    fn c(&self) -> Option<Mpz> {
        match &self.num {
            Float::Zero(_) => Some(Mpz::zero()),
            Float::Subnormal(_, c) => Some(c.clone()),
            Float::Normal(_, _, c) => Some(c.clone()),
            Float::Infinity(_) => None,
            Float::Nan(_, _, _) => None,
        }
    }

    fn m(&self) -> Option<Mpz> {
        self.c().map(|c| if self.sign() { -c } else { c })
    }

    fn p(&self) -> usize {
        match &self.num {
            Float::Zero(_) => 0,
            Float::Subnormal(_, c) => c.bit_length(),
            Float::Normal(_, _, c) => c.bit_length(),
            Float::Infinity(_) => 0,
            Float::Nan(_, _, _) => 0,
        }
    }

    fn is_nar(&self) -> bool {
        matches!(&self.num, Float::Infinity(_) | Float::Nan(_, _, _))
    }

    fn is_finite(&self) -> bool {
        matches!(
            &self.num,
            Float::Zero(_) | Float::Subnormal(_, _) | Float::Normal(_, _, _)
        )
    }

    fn is_infinite(&self) -> bool {
        matches!(&self.num, Float::Infinity(_))
    }

    fn is_zero(&self) -> bool {
        matches!(&self.num, Float::Zero(_))
    }

    fn is_negative(&self) -> Option<bool> {
        match &self.num {
            Float::Zero(s) => Some(*s),
            Float::Subnormal(s, _) => Some(*s),
            Float::Normal(s, _, _) => Some(*s),
            Float::Infinity(s) => Some(*s),
            Float::Nan(_, _, _) => None,
        }
    }

    fn is_numerical(&self) -> bool {
        !matches!(&self.num, Float::Nan(_, _, _))
    }
}

impl From<IEEE754> for Rational {
    fn from(val: IEEE754) -> Self {
        match &val.num {
            Float::Zero(_) => Rational::zero(),
            Float::Subnormal(_, _) => todo!(),
            Float::Normal(_, _, _) => todo!(),
            Float::Infinity(_) => todo!(),
            Float::Nan(_, _, _) => todo!(),
        }
    }
}
