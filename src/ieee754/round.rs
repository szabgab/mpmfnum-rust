// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754/round.rs
//
// IEEE 754 floating-point rounding

use std::cmp::min;

use crate::ieee754::IEEE754;
use crate::rational::{self, RoundingMode};
use crate::{Number, RoundingContext};

/// Rounding contexts for IEEE 754 floating-point numbers.
/// Must define format parameters `es` and `nbits` (see [`IEEE754`]
/// for a description of these fields). The rounding mode
/// affects the rounding direction. The `dtz` and `ftz` fields
/// specify subnormal handling specifically before an operation
/// `dtz` and after rounding `ftz`.
#[derive(Clone, Debug)]
pub struct Context {
    es: usize,
    nbits: usize,
    rm: RoundingMode,
    dtz: bool,
    ftz: bool,
}

impl Context {
    /// Constructs a new rounding context with the given format parameters.
    /// The default rounding mode is [`RoundingMode::NearestTiesToEven`].
    /// Both fields specifying subnormal behavior are false by default.
    pub fn new(es: usize, nbits: usize) -> Self {
        Self {
            es,
            nbits,
            rm: RoundingMode::NearestTiesToEven,
            dtz: false,
            ftz: false,
        }
    }

    /// Sets the rounding mode.
    pub fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rm = rm;
        self
    }

    /// Sets the denormal argument behavior.
    /// If enabled, a denormal argument will be interpreted as 0.
    pub fn with_dtz(mut self, enable: bool) -> Self {
        self.dtz = enable;
        self
    }

    /// Sets the subnormal result behavior.
    /// If enabled, any subnormal result will be flushed to zero.
    pub fn with_ftz(mut self, enable: bool) -> Self {
        self.ftz = enable;
        self
    }

    /// Returns the exponent bitwidth of the format produced by
    /// this context (when viewed as a bitvector). This is guaranteed
    /// to satisfy `2 <= self.es() <= self.nbits() - 2. Exponent
    /// overflowing will likely occur past 60 bits, but MPFR generally
    /// has a limit at 31 bits.
    pub fn es(&self) -> usize {
        self.es
    }

    /// Returns the total bitwidth of the format produced by this context
    /// (when viewed as a bitvector). This is guaranteed to satisfy
    /// `self.es() + 2 <= self.nbits()`.
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Returns the maximum precision allowed by this format.
    /// The result is always `self.nbits() - self.es()`.
    pub fn max_p(&self) -> usize {
        self.nbits - self.es
    }

    /// Returns the maximum significand width allowed by this format
    /// (when viewed as a bitvector) The result is always `self.max_p() - 1`.
    pub fn max_m(&self) -> usize {
        self.nbits - self.es - 1
    }

    /// Exponent of the largst finite floating-point value representable
    /// in this format when viewed as `(-1)^s * m * b^e` where `m`
    /// is a fraction between 1 and 2.
    pub fn emax(&self) -> isize {
        (1 << (self.es - 1)) - 1
    }

    /// Exponent of the smallest normal floating-point value representable
    /// in this format when viewed as `(-1)^s * m * b^e` where `m`
    /// is a fraction between 1 and 2. The result is just `self.emax() - 1`.
    pub fn emin(&self) -> isize {
        1 - self.emax()
    }

    /// Exponent of the largst finite floating-point value representable
    /// in this format when viewed as `(-1)^s * c * b^e` where `c`
    /// is an integer. The result is just `self.emax() - self.max_m()`
    pub fn expmax(&self) -> isize {
        self.emax() - (self.max_m() as isize)
    }

    /// Exponent of the smallest normal floating-point value representable
    /// in this format when viewed as `(-1)^s * c * b^e` where `c`
    /// is an integer. The result is just `self.emin() - self.max_m()`
    /// `self.emax() - 1`.
    pub fn expmin(&self) -> isize {
        self.emin() - (self.max_m() as isize)
    }

    /// The exponent "bias" used when converting a valid exponent range
    /// `[emin, emax]` to unsigned integers for bitpacking. Specifically,
    /// the final range is `[1, 2*emax]` The result is just `self.emax()`.
    pub fn bias(&self) -> isize {
        self.emax()
    }
}

impl RoundingContext for Context {
    type Rounded = IEEE754;

    fn round<T: Number>(&self, num: &T) -> IEEE754 {
        // step 1: rounding as a fixed-precision rational number
        // first, so we need to compute the context parameters.
        // IEEE 754 numbers support subnormalization so we need
        // to set both `max_p` and `min_n` when rounding using the
        // rational number rounding context.
        let max_p = self.nbits - self.es;
        let min_n = if let Some(exp) = num.exp() {
            min(exp, self.expmin()) - 1
        } else {
            self.expmin() - 1
        };

        // step 2: round and collect the lost bits
        let rctx = rational::Context::new()
            .with_max_precision(max_p)
            .with_min_n(min_n)
            .with_rounding_mode(self.rm);
        let (rounded, lost) = rctx.round_residual(num);


        todo!()
    }
}
