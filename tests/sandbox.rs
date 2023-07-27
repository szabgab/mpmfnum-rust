// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// sandbox.rs
//
// Sandboxing

use mpmfnum::ops::*;
use mpmfnum::ieee754;

#[test]
fn sandbox() {
    let ctx = ieee754::Context::new(4, 8);
    let x = ctx.qnan();
    let y = ctx.snan();
    let z = ctx.add(&x, &y);

    println!("{:?}", z);
}
