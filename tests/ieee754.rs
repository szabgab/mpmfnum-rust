// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754.rs
//
// Tests for the IEEE 754 module

use gmp::mpz::Mpz;
use mpmfnum::ieee754;
use mpmfnum::rational::Rational;
use mpmfnum::{Number, RoundingContext};

#[test]
fn sandbox() {
    let ctx = ieee754::Context::new(2, 4);
    let num = Rational::Real(false, -3, Mpz::from(7));
    let f = ctx.round(&num);
}
