use crate::rfloat::RFloat;
use crate::{Real, RoundingContext};

/// Rounding contexts for exact arithmetic.
///
/// The rounding operation is just the identity function.
#[derive(Clone, Debug, Default)]
pub struct RealContext {}

impl RealContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RoundingContext for RealContext {
    type Rounded = RFloat;

    fn round<T: Real>(&self, val: &T) -> Self::Rounded {
        if val.is_zero() {
            RFloat::zero()
        } else if val.is_finite() {
            RFloat::Real(val.sign(), val.exp().unwrap(), val.c().unwrap())
        } else if val.is_infinite() {
            RFloat::Infinite(val.sign())
        } else {
            RFloat::Nan
        }
    }
}
