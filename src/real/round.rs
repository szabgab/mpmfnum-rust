use crate::rfloat::RFloat;
use crate::{Real, RoundingContext, Split};

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
    type Format = RFloat;

    fn round<T: Real>(&self, val: &T) -> Self::Format {
        if val.is_zero() {
            RFloat::zero()
        } else if val.is_finite() {
            let sign = val.sign().unwrap_or(false);
            RFloat::Real(sign, val.exp().unwrap(), val.c().unwrap())
        } else if val.is_infinite() {
            match val.sign() {
                Some(true) => RFloat::NegInfinity,
                _ => RFloat::PosInfinity,
            }
        } else {
            RFloat::Nan
        }
    }

    fn round_split(&self, split: Split) -> Self::Format {
        if split.is_zero() {
            RFloat::zero()
        } else {
            RFloat::Real(
                split.sign().unwrap(),
                split.exp().unwrap(),
                split.c().unwrap(),
            )
        }
    }
}
