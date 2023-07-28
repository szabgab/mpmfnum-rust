use rug::Integer;

use crate::{rational::Rational, Number};

/// The classic fixed-point format.
///
/// Fixed-point numbers are parameterized by `nbits` the total bitwidth
/// of the representation, `scale` the position of the least-significant
/// digit in the representation, and if it is signed.
pub struct Fixed {
    pub(crate) num: Rational,
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
