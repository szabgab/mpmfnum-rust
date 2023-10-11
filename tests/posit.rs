use mpmfnum::{posit::*, rfloat::RFloat, Real};
use rug::Integer;

fn bits_to_rfloat(ctx: &PositContext, i: usize) -> RFloat {
    RFloat::from(ctx.bits_to_number(Integer::from(i)))
}

#[test]
fn enumerate() {
    // posit<2, 6> format
    let ctx = PositContext::new(2, 6);
    let all_vals = [
        RFloat::zero(),
        RFloat::Real(false, -16, Integer::from(1)), // (false, -4, 0, 1)
        RFloat::Real(false, -12, Integer::from(1)), // (false, -3, 0, 1)
        RFloat::Real(false, -10, Integer::from(1)), // (false, -3, 2, 1)
        RFloat::Real(false, -8, Integer::from(1)),  // (false, -2, 0, 1)
        RFloat::Real(false, -7, Integer::from(1)),  // (false, -2, 1, 1)
        RFloat::Real(false, -6, Integer::from(1)),  // (false, -2, 2, 1)
        RFloat::Real(false, -5, Integer::from(1)),  // (false, -2, 3, 1)
        RFloat::Real(false, -5, Integer::from(2)),  // (false, -1, -1, 2)
        RFloat::Real(false, -5, Integer::from(3)),  // (false, -1, -1, 3)
        RFloat::Real(false, -4, Integer::from(2)),  // (false, -1, 0, 2)
        RFloat::Real(false, -4, Integer::from(3)),  // (false, -1, 0, 3)
        RFloat::Real(false, -3, Integer::from(2)),  // (false, -1, 1, 2)
        RFloat::Real(false, -3, Integer::from(3)),  // (false, -1, 1, 3)
        RFloat::Real(false, -2, Integer::from(2)),  // (false, -1, 2, 2)
        RFloat::Real(false, -2, Integer::from(3)),  // (false, -1, 2, 3)
        RFloat::Real(false, -1, Integer::from(2)),  // (false, 0, -1, 2)
        RFloat::Real(false, -1, Integer::from(3)),  // (false, 0, -1, 3)
        RFloat::Real(false, 0, Integer::from(2)),   // (false, 0, 0, 2)
        RFloat::Real(false, 0, Integer::from(3)),   // (false, 0, 0, 3)
        RFloat::Real(false, 1, Integer::from(2)),   // (false, 0, 1, 2)
        RFloat::Real(false, 1, Integer::from(3)),   // (false, 0, 1, 3)
        RFloat::Real(false, 2, Integer::from(2)),   // (false, 0, 2, 2)
        RFloat::Real(false, 2, Integer::from(3)),   // (false, 0, 2, 3)
        RFloat::Real(false, 4, Integer::from(1)),   // (false, 1, 0, 1)
        RFloat::Real(false, 5, Integer::from(1)),   // (false, 1, 1, 1)
        RFloat::Real(false, 6, Integer::from(1)),   // (false, 1, 2, 1)
        RFloat::Real(false, 7, Integer::from(1)),   // (false, 1, 3, 1)
        RFloat::Real(false, 8, Integer::from(1)),   // (false, 2, 0, 1)
        RFloat::Real(false, 10, Integer::from(1)),  // (false, 2, 2, 1)
        RFloat::Real(false, 12, Integer::from(1)),  // (false, 3, 0, 1)
        RFloat::Real(false, 16, Integer::from(1)),  // (false, 4, 0, 1)
        RFloat::Nan,
        RFloat::Real(true, -16, Integer::from(1)), // (true, -4, 0, 1)
        RFloat::Real(true, -12, Integer::from(1)), // (true, -3, 0, 1)
        RFloat::Real(true, -10, Integer::from(1)), // (true, -3, 2, 1)
        RFloat::Real(true, -8, Integer::from(1)),  // (true, -2, 0, 1)
        RFloat::Real(true, -7, Integer::from(1)),  // (true, -2, 1, 1)
        RFloat::Real(true, -6, Integer::from(1)),  // (true, -2, 2, 1)
        RFloat::Real(true, -5, Integer::from(1)),  // (true, -2, 3, 1)
        RFloat::Real(true, -5, Integer::from(2)),  // (true, -1, -1, 2)
        RFloat::Real(true, -5, Integer::from(3)),  // (true, -1, -1, 3)
        RFloat::Real(true, -4, Integer::from(2)),  // (true, -1, 0, 2)
        RFloat::Real(true, -4, Integer::from(3)),  // (true, -1, 0, 3)
        RFloat::Real(true, -3, Integer::from(2)),  // (true, -1, 1, 2)
        RFloat::Real(true, -3, Integer::from(3)),  // (true, -1, 1, 3)
        RFloat::Real(true, -2, Integer::from(2)),  // (true, -1, 2, 2)
        RFloat::Real(true, -2, Integer::from(3)),  // (true, -1, 2, 3)
        RFloat::Real(true, -1, Integer::from(2)),  // (true, 0, -1, 2)
        RFloat::Real(true, -1, Integer::from(3)),  // (true, 0, -1, 3)
        RFloat::Real(true, 0, Integer::from(2)),   // (true, 0, 0, 2)
        RFloat::Real(true, 0, Integer::from(3)),   // (true, 0, 0, 3)
        RFloat::Real(true, 1, Integer::from(2)),   // (true, 0, 1, 2)
        RFloat::Real(true, 1, Integer::from(3)),   // (true, 0, 1, 3)
        RFloat::Real(true, 2, Integer::from(2)),   // (true, 0, 2, 2)
        RFloat::Real(true, 2, Integer::from(3)),   // (true, 0, 2, 3)
        RFloat::Real(true, 4, Integer::from(1)),   // (true, 1, 0, 1)
        RFloat::Real(true, 5, Integer::from(1)),   // (true, 1, 1, 1)
        RFloat::Real(true, 6, Integer::from(1)),   // (true, 1, 2, 1)
        RFloat::Real(true, 7, Integer::from(1)),   // (true, 1, 3, 1)
        RFloat::Real(true, 8, Integer::from(1)),   // (true, 2, 0, 1)
        RFloat::Real(true, 10, Integer::from(1)),  // (true, 2, 2, 1)
        RFloat::Real(true, 12, Integer::from(1)),  // (true, 3, 0, 1)
        RFloat::Real(true, 16, Integer::from(1)),  // (true, 4, 0, 1)
    ];

    for (i, v) in all_vals.iter().enumerate() {
        let num = bits_to_rfloat(&ctx, i);
        if num.is_nar() {
            assert!(v.is_nar(), "failed conversion: i={}, v=NAR, e={:?}", i, v);
        } else {
            assert_eq!(
                num.clone(),
                v.clone(),
                "failed conversion: i={}, v={:?}, e={:?}",
                i,
                num,
                v
            );
        }

        let f = ctx.bits_to_number(Integer::from(i));
        let j = f.into_bits();
        assert_eq!(i, j, "failed round-trip: i={}, j={}, v={:?}", i, j, num);
    }
}

#[test]
fn bounds() {
    // posit<2, 8> format
    let ctx = PositContext::new(2, 8);
    assert_eq!(ctx.useed(), 16);
    assert_eq!(
        RFloat::from(ctx.maxval()),
        RFloat::Real(false, 24, Integer::from(1))
    );
    assert_eq!(
        RFloat::from(ctx.minval()),
        RFloat::Real(false, -24, Integer::from(1))
    );
}
