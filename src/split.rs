use rug::Integer;

use crate::rfloat::RFloat;
use crate::{Real, RoundingContext};

/// An exact sum of two [`Real`] values split at an absolute binary digit.
#[derive(Clone, Debug)]
pub struct Split {
    high: RFloat,
    low: RFloat,
    max_p: Option<usize>,
    split_pos: isize,
}

impl Split {
    /// Splits a [`Real`] at binary digit `n` into two [`RFloat`] values:
    ///
    ///  - all significant digits above position `n`
    ///  - all significant digits at or below position `n`
    ///
    /// The sum of the resulting values will be exactly the input number,
    /// that is, it "splits" a number.
    pub fn new<T: Real>(num: &T, max_p: Option<usize>, n: isize) -> Self {
        assert!(!num.is_nar(), "must be real {:?}", num);
        let (high, low) = num.split(n);
        Self {
            high,
            low,
            max_p,
            split_pos: n,
        }
    }

    /// Extracts the upper value of the split.
    pub fn num(&self) -> &RFloat {
        &self.high
    }

    /// Extracts the lower value of the split.
    pub fn lost(&self) -> &RFloat {
        &self.low
    }

    /// The precision of the upper value of the split.
    pub fn max_p(&self) -> Option<usize> {
        self.max_p
    }

    /// The position of the first digit lost in the split.
    pub fn split_pos(&self) -> isize {
        self.split_pos
    }

    /// Extracts the round, guard, and sticky bit ("RGS") from lost digits.
    pub fn rgs(&self) -> (bool, bool, bool) {
        let (half, lower) = self.low.split(self.split_pos - 1);
        let (quarter, lower) = lower.split(self.split_pos - 2);
        (!half.is_zero(), !quarter.is_zero(), !lower.is_zero())
    }

    /// Extracts the round and sticky bit ("RGS") from lost digits.
    pub fn rs(&self) -> (bool, bool) {
        let (half, lower) = self.low.split(self.split_pos - 1);
        (!half.is_zero(), !lower.is_zero())
    }

    /// Returns `true` if the lost digits are all zero.
    pub fn is_exact(&self) -> bool {
        self.low.is_zero()
    }

    /// Rounds this [`Split`] according to a [`RoundingContext`].
    pub fn round<Ctx: RoundingContext>(&self, ctx: &Ctx) -> Ctx::Format {
        ctx.round_split(self.to_owned())
    }
}

impl Real for Split {
    fn radix() -> usize {
        2
    }

    fn sign(&self) -> Option<bool> {
        self.high.sign()
    }

    fn exp(&self) -> Option<isize> {
        match self.low.exp() {
            Some(exp) => Some(exp),
            None => self.high.exp(),
        }
    }

    fn e(&self) -> Option<isize> {
        match self.high.e() {
            Some(e) => Some(e),
            None => self.low.e(),
        }
    }

    fn n(&self) -> Option<isize> {
        match self.low.n() {
            Some(n) => Some(n),
            None => self.high.n(),
        }
    }

    fn c(&self) -> Option<Integer> {
        match (self.high.c(), self.low.c()) {
            (None, None) => None,
            (Some(c), None) | (None, Some(c)) => Some(c),
            (Some(c1), Some(c2)) => {
                let offset = self.high.exp().unwrap() - self.low.exp().unwrap();
                Some((c1 << offset) + c2)
            }
        }
    }

    fn m(&self) -> Option<Integer> {
        self.c().map(|c| if self.sign().unwrap() { -c } else { c })
    }

    fn prec(&self) -> Option<usize> {
        match (self.e(), self.n()) {
            (None, None) => None,
            (Some(e), Some(n)) => Some((e - n) as usize),
            (_, _) => panic!("unreachable"),
        }
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
        self.high.is_zero() && self.low.is_zero()
    }

    fn is_negative(&self) -> Option<bool> {
        match self.high.is_negative() {
            Some(b) => Some(b),
            None => self.low.is_negative(),
        }
    }

    fn is_numerical(&self) -> bool {
        true
    }
}
