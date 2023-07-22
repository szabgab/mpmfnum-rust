// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational/types.rs
//
// The rational number type

use gmp::mpz::*;

use crate::number::Number;

/// The rational number format.
///
/// This is not a traditional rational number `p/q` where `p` and `q`
/// are integers (canonically, `p` is signed). Instead, this type defines
/// a _fixed-width_ rational number `(-1)^s * c * 2^e` where `c` is a
/// binary-encoded integer with a maximum bitwidth. Like rational numbers,
/// `e` is theoretically unbounded and may be as large or small as needed.
/// Rational numbers may encode a non-real number (see [`NAR`]) which is
/// interpreted as a NaN (neither finite nor infinite). All operations
/// canonicalize -0 to +0 (no sign bit).
#[derive(Debug, Clone)]
pub enum Rational {
    /// A finite (real) number specified by the canonical triple
    /// of sign, exponent, significand.
    Real(bool, Mpz, Mpz),
    /// An infinite number (signed to indicate direction).
    Infinite(bool),
    /// Not a real number; either an undefined or infinte result.
    Nan,
}

/// An instatiation of [`Rational::Nan`].
pub const NAN: Rational = Rational::Nan;

/// An instantiation of [`Rational::Infinite`] with positive sign.
pub const POS_INF: Rational = Rational::Infinite(true);

/// An instantiation of [`Rational::Infinite`] with negative sign.
pub const NEG_INF: Rational = Rational::Infinite(false);

// Implements the `Number` trait for `Rational`.
// See `Rational` for a description of the trait and its members.
impl Number for Rational {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        match self {
            Rational::Real(s, _, _) => *s,
            Rational::Infinite(s) => *s,
            Rational::Nan => false,
        }
    }

    fn exp(&self) -> Option<Mpz> {
        match self {
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(exp.clone())
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn e(&self) -> Option<Mpz> {
        match self {
            // (exp - 1) + len(c)
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    let n = exp - Mpz::from(1);
                    Some(n + Mpz::from(c.bit_length() as u64))
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn n(&self) -> Option<Mpz> {
        match self {
            // exp - 1
            Rational::Real(_, exp, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(exp - Mpz::from(1))
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn c(&self) -> Option<Mpz> {
        match self {
            Rational::Real(_, _, c) => Some(c.clone()),
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn m(&self) -> Option<Mpz> {
        match self {
            Rational::Real(s, _, c) => {
                if *s {
                    Some(-c.clone())
                } else {
                    Some(c.clone())
                }
            }
            Rational::Infinite(_) => None,
            Rational::Nan => None,
        }
    }

    fn p(&self) -> usize {
        match self {
            Rational::Real(_, _, c) => {
                if c.is_zero() {
                    0
                } else {
                    c.bit_length()
                }
            }
            Rational::Infinite(_) => 0,
            Rational::Nan => 0,
        }
    }

    fn is_nar(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => false,
            Rational::Infinite(_) => true,
            Rational::Nan => true,
        }
    }

    fn is_finite(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => true,
            Rational::Infinite(_) => false,
            Rational::Nan => false,
        }
    }

    fn is_infinite(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => false,
            Rational::Infinite(_) => true,
            Rational::Nan => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Rational::Real(_, _, c) => c.is_zero(),
            Rational::Infinite(_) => false,
            Rational::Nan => false,
        }
    }

    fn is_negative(&self) -> Option<bool> {
        match self {
            Rational::Real(s, _, c) => {
                if c.is_zero() {
                    None
                } else {
                    Some(*s)
                }
            }
            Rational::Infinite(s) => Some(*s),
            Rational::Nan => None,
        }
    }

    fn is_numerical(&self) -> bool {
        match self {
            Rational::Real(_, _, _) => true,
            Rational::Infinite(_) => true,
            Rational::Nan => false,
        }
    }
}
