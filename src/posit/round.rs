use rug::Integer;

use crate::{Real, RoundingContext};

use super::{Posit, PositVal};

/// Rounding contexts for posit numbers.
///
/// The associated storage type is [`Posit`].
///
/// Values rounded under this context are posit numbers as described
/// by the Posit standard: base 2 scientific numbers:
/// `(-1)^s * c * 2^e * (2^2^es)^r` where `c` is an unsigned integer,
/// `r` and `e` are integers. The key property of posit numbers
/// is that the precision of `c` and `e` change based on the value
/// of `r`. In general, `c` and `e` are large when `r` is near 0
/// and small (or zero) when `r` is large or small. In posit terminology,
/// the value `2^2^es` is called `useed`.
///
/// A [`PositContext`] is parameterized by
///
///  - maximum bitwidth of the exponent field
///  - total bitwidth of the encoding
///
/// For values in between the largest and smallest magnitude,
/// [`NearestTiesToEven`][RoundingDirection::NearestTiesToEven].
/// Otherwise, the values are flushed to `NAR`.
#[derive(Clone, Debug)]
pub struct PositContext {
    es: usize,
    nbits: usize,
}

impl PositContext {
    /// Implementation limit: maximum exponent size
    pub const ES_MAX: usize = 32;
    /// Implementation limit: minimum additional bitwidth,
    pub const PAD_MIN: usize = 3;

    pub fn new(es: usize, nbits: usize) -> Self {
        assert!(
            es <= Self::ES_MAX,
            "exponent width needs to be at most {} bits, given {} bits",
            Self::ES_MAX,
            es
        );
        assert!(
            nbits >= es + Self::PAD_MIN,
            "total bitwidth needs to be at least {} bits, given {} bits",
            es + Self::PAD_MIN,
            nbits
        );

        Self { es, nbits }
    }

    /// Returns the regime bitwidth of the format produced by
    /// this context (when viewed as a bitvector). This is guaranteed
    /// to satisfy `2 <= self.es() < self.nbits() - 2.
    pub fn es(&self) -> usize {
        self.es
    }

    /// Returns the total bitwidth of the format produced by this context
    /// (when viewed as a bitvector). This is guaranteed to satisfy
    /// `self.es() + 2 < self.nbits()`.
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Returns the maximum precision allowed by this format.
    pub fn max_p(&self) -> usize {
        self.nbits - self.es - 3
    }

    /// Posit terminology for `2^2^es`
    pub fn useed(&self) -> isize {
        (1_usize << (1 << self.es)) as isize
    }

    /// The exponent scale `2^es`
    pub fn rscale(&self) -> isize {
        (1 << self.es) as isize
    }

    /// Largest representable regime
    pub fn rmax(&self) -> isize {
        let max_r = (self.nbits - 1) as isize;
        max_r - 1
    }

    /// Largest representable regime
    pub fn rmin(&self) -> isize {
        let max_r = (self.nbits - 1) as isize;
        1 - max_r
    }

    /// Largest representable (normalized) exponent
    pub fn emax(&self) -> isize {
        // format only contains regime bits
        self.rscale() * self.rmax()
    }

    /// Smallest representable (unnormalized) exponent
    pub fn expmax(&self) -> isize {
        // format only contains regime bits
        self.emax()
    }

    /// Smallest representable (normalized) exponent
    pub fn emin(&self) -> isize {
        // format only contains regime bits
        self.rscale() * self.rmin()
    }

    /// Largest representable (unnormalized) exponent
    pub fn expmin(&self) -> isize {
        self.emin() // precision is 1 bit
    }

    /// Maximum representable value
    pub fn maxval(&self) -> Posit {
        Posit {
            num: PositVal::NonZero(false, self.rmax(), 0, Integer::from(1)),
            ctx: self.clone(),
        }
    }

    /// Minimum representable value
    pub fn minval(&self) -> Posit {
        Posit {
            num: PositVal::NonZero(false, self.rmin(), 0, Integer::from(1)),
            ctx: self.clone(),
        }
    }

    /// Constructs zero in this format.
    pub fn zero(&self) -> Posit {
        Posit {
            num: PositVal::Zero,
            ctx: self.clone(),
        }
    }
}

impl RoundingContext for PositContext {
    type Rounded = Posit;

    fn round<T: Real>(&self, val: &T) -> Self::Rounded {
        todo!()
    }
}
