use mpmfnum::{posit::*, rfloat::RFloat, Real, RoundingContext};
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
    }
}

#[test]
fn round_trip() {
    // posit<2, 6> format
    let ctx = PositContext::new(2, 6);
    for i in 0..(1 << ctx.nbits()) {
        let num = ctx.bits_to_number(Integer::from(i));
        let j = num.clone().into_bits();
        assert_eq!(i, j, "round trip failed: i={}, j={}, num={:?}", i, j, num);
    }

    // posit<2, 8> format
    let ctx = PositContext::new(2, 8);
    for i in 0..(1 << ctx.nbits()) {
        let num = ctx.bits_to_number(Integer::from(i));
        let j = num.clone().into_bits();
        assert_eq!(i, j, "round trip failed: i={}, j={}, num={:?}", i, j, num);
    }

    // posit<3, 12> format
    let ctx = PositContext::new(3, 12);
    for i in 0..(1 << ctx.nbits()) {
        let num = ctx.bits_to_number(Integer::from(i));
        let j = num.clone().into_bits();
        assert_eq!(i, j, "round trip failed: i={}, j={}, num={:?}", i, j, num);
    }
}

#[test]
fn bounds() {
    // posit<2, 8> format
    let ctx = PositContext::new(2, 8);
    assert_eq!(ctx.useed(), 16);
    assert_eq!(
        RFloat::from(ctx.maxval(false)),
        RFloat::Real(false, 24, Integer::from(1))
    );
    assert_eq!(
        RFloat::from(ctx.minval(false)),
        RFloat::Real(false, -24, Integer::from(1))
    );

    // posit<3, 8> format
    let ctx = PositContext::new(3, 8);
    assert_eq!(ctx.useed(), 256);
    assert_eq!(
        RFloat::from(ctx.maxval(false)),
        RFloat::Real(false, 48, Integer::from(1))
    );
    assert_eq!(
        RFloat::from(ctx.minval(false)),
        RFloat::Real(false, -48, Integer::from(1))
    );
}

#[test]
fn round_small() {
    let ctx = PositContext::new(2, 8);

    // rounding NaN
    let nan = RFloat::Nan;
    let rounded_nan = ctx.round(&nan);
    assert!(rounded_nan.is_nar(), "round(NaN) = NaR");

    // rounding +Inf
    let inf = RFloat::Infinite(true);
    let rounded_inf = ctx.round(&inf);
    assert!(rounded_inf.is_nar(), "round(+Inf) = NaR");

    // rounding +Inf
    let inf = RFloat::Infinite(false);
    let rounded_inf = ctx.round(&inf);
    assert!(rounded_inf.is_nar(), "round(+Inf) = NaR");

    // rounding 0
    let zero = RFloat::zero();
    let rounded_zero = ctx.round(&zero);
    assert!(rounded_zero.is_zero(), "round(+0) = +0");

    // rounding MAXVAL + 1
    let maxp1 = RFloat::from(ctx.maxval(false)) + RFloat::one();
    let rounded_maxp1 = ctx.round(&maxp1);
    assert_eq!(rounded_maxp1, ctx.maxval(false), "round(MAXVAL+1) = MAXVAL");

    // rounding MINVAL + 1
    let minval = RFloat::from(ctx.minval(false));
    let tiny = RFloat::Real(
        minval.sign(),
        minval.exp().unwrap() - 1,
        minval.c().unwrap(),
    );
    let rounded_tiny = ctx.round(&tiny);
    assert_eq!(
        rounded_tiny,
        ctx.minval(false),
        "rouned(MINVAL * 2^-1, MINVAL)"
    );

    // rounding +1
    let one = RFloat::one();
    let rounded_one = ctx.round(&one);
    assert_eq!(RFloat::from(rounded_one), one, "round(+1) = +1");

    // rounding +1.0625
    let one_1_16 = RFloat::Real(false, -4, Integer::from(17));
    let rounded = ctx.round(&one_1_16);
    assert_eq!(RFloat::from(rounded), one, "round(+1.0625) = +1");

    // rounding +1.1875
    let one_3_16 = RFloat::Real(false, -4, Integer::from(19));
    let rounded = ctx.round(&one_3_16);
    assert_eq!(
        RFloat::from(rounded),
        RFloat::Real(false, -4, Integer::from(20)),
        "round(+1.1875) = +1.25"
    );
}
