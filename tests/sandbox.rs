// mpmfnum: a numbers library in Rust
use mpmfnum::RoundingContext;
use rug::Integer;

use mpmfnum::ieee754;
use mpmfnum::ops::*;

#[test]
fn sandbox() {
    let ctx = ieee754::Context::new(4, 8);
    let x = ctx.qnan();
    let y = ctx.snan();
    let z = ctx.add(&x, &y);
    println!("{:?}", z);

    let ctx2 = ieee754::Context::new(3, 6);
    let z = ctx.bits_to_number(Integer::from(0xFF));
    let w = ctx2.round(&z);
    println!("{:?} {:?}", z, w);
}
