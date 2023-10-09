use mpmfnum::{posit::*, rfloat::RFloat};
use rug::Integer;

#[test]
fn enumerate() {
    // "Posit8" format
    let ctx = PositContext::new(2, 8);
    assert_eq!(ctx.useed(), 16);
    assert_eq!(RFloat::from(ctx.maxval()), RFloat::Real(false, 24, Integer::from(1)));
    assert_eq!(RFloat::from(ctx.minval()), RFloat::Real(false, -24, Integer::from(1)));
}
