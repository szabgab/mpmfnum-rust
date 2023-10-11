use rug::Integer;

use crate::{rfloat::RFloatContext, util::bitmask, Real, RoundingContext, RoundingMode};

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
        self.rscale() * -self.rmax()
    }

    /// Largest representable (unnormalized) exponent
    pub fn expmin(&self) -> isize {
        self.emin() // precision is 1 bit
    }

    /// Maximum representable value.
    pub fn maxval(&self, sign: bool) -> Posit {
        Posit {
            num: PositVal::NonZero(sign, self.rmax(), 0, Integer::from(1)),
            ctx: self.clone(),
        }
    }

    /// Minimum representable value.
    pub fn minval(&self, sign: bool) -> Posit {
        Posit {
            num: PositVal::NonZero(sign, -self.rmax(), 0, Integer::from(1)),
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

    /// Constructs `NAR` in this format.
    pub fn nar(&self) -> Posit {
        Posit {
            num: PositVal::Nar,
            ctx: self.clone(),
        }
    }

    /// Converts an [`Integer`] representing a posit bitpattern into
    /// a [`Posit`] value under this [`PositContext`].
    pub fn bits_to_number(&self, b: Integer) -> Posit {
        let limit = Integer::from(1) << self.nbits;
        assert!(b < limit, "must be less than 1 << nbits");

        // decompose into sign and magnitude
        let s = b.get_bit((self.nbits - 1) as u32);
        let ns = b & bitmask(self.nbits - 1);

        if ns == 0 {
            // either 0 or NAR
            Posit {
                num: if s { PositVal::Nar } else { PositVal::Zero },
                ctx: self.clone(),
            }
        } else {
            // scan for LSB of the regime field
            let r0 = ns.get_bit((self.nbits - 2) as u32);
            let mut r0_pos = self.nbits - 2;
            while r0_pos > 0 && ns.get_bit((r0_pos - 1) as u32) == r0 {
                r0_pos -= 1;
            }

            if r0_pos == 0 {
                // special case: we shifted out looking for the LSB
                // of the regime, so we must be the maximum value
                Posit {
                    num: PositVal::NonZero(s, self.rmax(), 0, Integer::from(1)),
                    ctx: self.clone(),
                }
            } else {
                // exponent and mantissa fields are dynamic and start
                // below `r0` with mantissa being shifted off first
                let embits = r0_pos - 1;
                let rbits = self.nbits - embits - 1;
                let (ebits, mbits) = if embits <= self.es {
                    (embits, 0)
                } else {
                    (self.es, embits - self.es)
                };

                // extract bits
                let efield = (ns.clone() >> mbits) & bitmask(ebits);
                let mfield = ns & bitmask(mbits);

                // convert regime
                let kbits = rbits - 1;
                let regime = if r0 {
                    kbits as isize - 1
                } else {
                    -(kbits as isize)
                };

                // convert exponent
                let e = if ebits < self.es {
                    efield.to_isize().unwrap() << (self.es - ebits)
                } else {
                    efield.to_isize().unwrap()
                };

                // convert significand
                let c = mfield | (1 << mbits);

                // compose result
                Posit {
                    num: PositVal::NonZero(s, regime, e - mbits as isize, c),
                    ctx: self.clone(),
                }
            }
        }
    }
}

// Rounding utility functions.
impl PositContext {
    fn round_finite<T: Real>(&self, val: &T) -> Posit {
        // extract fields
        let s = val.sign();
        let e = val.e().unwrap();
        if e >= self.emax() {
            // |val| >= MAXVAL
            self.maxval(s)
        } else if e <= self.emin() {
            // |val| <= MINVAL
            self.minval(s)
        } else {
            // within representable range

            // step 1: compute size of the mantissa since it is dynamic,
            // it is a function of the size of the regime field
            let useed = self.useed();
            let r = e / useed;
            let kbits = if r < 0 { -r } else { r + 1 } as usize;
            let embits = self.nbits - (kbits + 2);
            let mbits = if embits <= self.es {
                0
            } else {
                embits - self.es
            };

            // step 2: rounding as an unbounded, fixed-precision floating-point,
            // so we need to compute the context parameters: we use
            // precision `mbits + 1` using `NearestTiesToEven`
            let (p, n) = RFloatContext::new().with_max_p(mbits + 1).round_params(val);

            // step 3: split the significand at binary digit `n`
            let split = RFloatContext::round_prepare(val, n);

            // step 4: finalize the rounding
            let rounded = RFloatContext::round_finalize(split, p, RoundingMode::NearestTiesToEven);

            // recompute exponent
            let e = rounded.e().unwrap();
            let r = e / useed;
            let e = e % useed;

            // unnormalized exponent and significand
            let c = rounded.c().unwrap();
            let exp = (e + 1) - (c.significant_bits() as isize);

            // compose result
            Posit {
                num: PositVal::NonZero(s, r, exp, c),
                ctx: self.clone(),
            }
        }
    }
}

impl RoundingContext for PositContext {
    type Rounded = Posit;

    fn round<T: Real>(&self, val: &T) -> Self::Rounded {
        if val.is_zero() {
            self.zero()
        } else if val.is_nar() {
            self.nar()
        } else {
            self.round_finite(val)
        }
    }
}
