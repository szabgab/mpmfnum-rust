use num_traits::Zero;
use rug::Integer;

use crate::{util::*, RoundingContext};
use crate::rfloat::RFloat;
use crate::Real;

/// Result of splitting a [`Real`] at binary digit `n`.
#[derive(Clone, Debug)]
pub struct Split {
    high: RFloat,
    low: RFloat,
    max_p: Option<usize>,
    n: isize,
}

impl Split {
    /// Splits a [`Real`] at binary digit `n`, returning two [`RFloat`] values:
    ///
    ///  - all significant digits above position `n`
    ///  - all significant digits at or below position `n`
    ///
    /// The sum of the resulting values will be exactly the input number,
    /// that is, it "splits" a number.
    fn split<T: Real>(num: &T, n: isize) -> (RFloat, RFloat) {
        let s = num.sign().unwrap();
        if num.is_zero() {
            let high = RFloat::Real(s, 0, Integer::zero());
            let low = RFloat::Real(s, 0, Integer::zero());
            (high, low)
        } else {
            // number components
            let e = num.e().unwrap();
            let exp = num.exp().unwrap();
            let c = num.c().unwrap();

            // case split by split point offset
            if n >= e {
                // split point is above the significant digits
                let high = RFloat::Real(s, 0, Integer::zero());
                let low = RFloat::Real(s, exp, c);
                (high, low)
            } else if n < exp {
                // split point is below the significant digits
                let high = RFloat::Real(s, exp, c);
                let low = RFloat::Real(s, 0, Integer::zero());
                (high, low)
            } else {
                // split point is within the significant digits
                let offset = n - (exp - 1);
                let mask = bitmask(offset as usize);
                let high = RFloat::Real(s, n + 1, c.to_owned() >> offset);
                let low = RFloat::Real(s, exp, c & mask);
                (high, low)
            }
        }
    }

    /// Splits a [`Real`] at binary digit `n` into two [`RFloat`] values:
    ///
    ///  - all significant digits above position `n`
    ///  - all significant digits at or below position `n`
    ///
    /// The sum of the resulting values will be exactly the input number,
    /// that is, it "splits" a number.
    pub fn new<T: Real>(num: &T, max_p: Option<usize>, n: isize) -> Self {
        assert!(!num.is_nar(), "must be real {:?}", num);
        let (high, low) = Self::split(num, n);
        Self { high, low, max_p, n }
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

    /// The absolute digit of the split.
    pub fn n(&self) -> isize {
        self.n
    }

    /// Extracts the round, guard, and sticky bit ("RGS") from lost digits.
    pub fn rgs(&self) -> (bool, bool, bool) {
        let (half, lower) = Self::split(&self.low, self.n - 1);
        let (quarter, lower) = Self::split(&lower, self.n - 2);
        (!half.is_zero(), !quarter.is_zero(), !lower.is_zero())
    }

    /// Extracts the round and sticky bit ("RGS") from lost digits.
    pub fn rs(&self) -> (bool, bool) {
        let (half, lower) = Self::split(&self.low, self.n - 1);
        (!half.is_zero(), !lower.is_zero())
    }

    /// Rounds this [`Split`] according to a [`RoundingContext`].
    pub fn round<Ctx: RoundingContext>(&self, ctx: &Ctx)  -> Ctx::Format {  
        ctx.round_split(self.to_owned())
    }
}
