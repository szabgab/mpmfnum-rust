use rug::Integer;

use crate::{rfloat::RFloat, Real};

use super::PositContext;

/// Posit number encoding viewed as an enumeration.
/// Unlike [`Posit`], [`PositVal`] represents only numerical data.
#[derive(Clone, Debug)]
pub enum PositVal {
    /// Exact zero
    Zero,
    /// Finite, non-zero value
    NonZero(bool, isize, isize, Integer),
    /// Non-real or undefined
    Nar,
}

/// Posit number format.
///
/// The associated [`RoundingContext`][crate::RoundingContext]
/// implementation is [`PositContext`].
/// See [`PositContext`] for more details on numerical properties
/// of the [`Posit`] type.
#[derive(Clone, Debug)]
pub struct Posit {
    pub(crate) num: PositVal,
    pub(crate) ctx: PositContext,
}

impl Posit {
    /// Returns the rounding context under which this number was created.
    pub fn ctx(&self) -> &PositContext {
        &self.ctx
    }
}

impl Real for Posit {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> bool {
        self.is_negative().unwrap_or(false)
    }

    fn exp(&self) -> Option<isize> {
        match &self.num {
            PositVal::Zero => None,
            PositVal::NonZero(_, r, exp, _) => Some((r * self.ctx.useed()) + exp),
            PositVal::Nar => None,
        }
    }

    fn e(&self) -> Option<isize> {
        match &self.num {
            PositVal::Zero => None,
            PositVal::NonZero(_, r, exp, c) => {
                Some(((r * self.ctx.useed()) + exp - 1) + (c.significant_bits() as isize))
            }
            PositVal::Nar => None,
        }
    }

    fn n(&self) -> Option<isize> {
        match &self.num {
            PositVal::Zero => None,
            PositVal::NonZero(_, r, exp, _) => Some((r * self.ctx.useed()) + exp - 1),
            PositVal::Nar => None,
        }
    }

    fn c(&self) -> Option<Integer> {
        match &self.num {
            PositVal::Zero => None,
            PositVal::NonZero(_, _, _, c) => Some(c.clone()),
            PositVal::Nar => None,
        }
    }

    fn m(&self) -> Option<Integer> {
        self.c().map(|c| if self.sign() { -c } else { c })
    }

    fn p(&self) -> usize {
        match &self.num {
            PositVal::Zero => 0,
            PositVal::NonZero(_, _, _, c) => c.significant_bits() as usize,
            PositVal::Nar => 0,
        }
    }

    fn is_nar(&self) -> bool {
        matches!(self.num, PositVal::Nar)
    }

    fn is_finite(&self) -> bool {
        !matches!(self.num, PositVal::Nar)
    }

    fn is_infinite(&self) -> bool {
        false
    }

    fn is_zero(&self) -> bool {
        matches!(self.num, PositVal::Zero)
    }

    fn is_negative(&self) -> Option<bool> {
        match &self.num {
            PositVal::Zero => None,
            PositVal::NonZero(s, _, _, _) => Some(*s),
            PositVal::Nar => None,
        }
    }

    fn is_numerical(&self) -> bool {
        !matches!(self.num, PositVal::Nar)
    }
}

impl From<Posit> for RFloat {
    fn from(value: Posit) -> Self {
        match value.num {
            PositVal::Zero => RFloat::zero(),
            PositVal::NonZero(s, r, exp, c) => RFloat::Real(s, value.ctx.rscale() * r + exp, c),
            PositVal::Nar => RFloat::Nan,
        }
    }
}
