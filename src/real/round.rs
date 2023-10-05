use crate::{rational::Rational, Real, RoundingContext};

/// Rounding contexts for exact arithmetic.
///
/// Just the identity function.
#[derive(Clone, Debug, Default)]
pub struct RealContext {}

impl RealContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RoundingContext for RealContext {
    type Rounded = Rational;

    fn round<T: Real>(&self, val: &T) -> Self::Rounded {
        if val.is_zero() {
            Rational::zero()
        } else if val.is_finite() {
            Rational::Real(val.sign(), val.exp().unwrap(), val.c().unwrap())
        } else if val.is_infinite() {
            Rational::Infinite(val.sign())
        } else {
            Rational::Nan
        }
    }
}
