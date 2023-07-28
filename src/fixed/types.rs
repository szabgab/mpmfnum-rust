use crate::{rational::Rational, Number};

/// The classic fixed-point format.
///
/// Fixed-point numbers are parameterized by `nbits` the total bitwidth
/// of the representation, `scale` the position of the least-significant
/// digit in the representation, and if it is signed.
pub struct Fixed {
    num: Rational,
}

impl Number for Fixed {
    fn radix() -> usize {
        todo!()
    }

    fn sign(&self) -> bool {
        todo!()
    }

    fn exp(&self) -> Option<isize> {
        todo!()
    }

    fn e(&self) -> Option<isize> {
        todo!()
    }

    fn n(&self) -> Option<isize> {
        todo!()
    }

    fn c(&self) -> Option<rug::Integer> {
        todo!()
    }

    fn m(&self) -> Option<rug::Integer> {
        todo!()
    }

    fn p(&self) -> usize {
        todo!()
    }

    fn is_nar(&self) -> bool {
        todo!()
    }

    fn is_finite(&self) -> bool {
        todo!()
    }

    fn is_infinite(&self) -> bool {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> Option<bool> {
        todo!()
    }

    fn is_numerical(&self) -> bool {
        todo!()
    }
}
