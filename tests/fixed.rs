use mpmfnum::fixed::{FixedContext, Overflow};
use mpmfnum::ops::RoundedAdd;
use mpmfnum::rfloat::RFloat;
use mpmfnum::{fixed, RoundingContext};
use rug::Integer;

fn assert_round_small(signed: bool, scale: isize, nbits: usize, input: &RFloat, output: &RFloat) {
    let ctx = fixed::FixedContext::new(signed, scale, nbits);
    let rounded = ctx.round(input);

    assert_eq!(RFloat::from(rounded.clone()), *output, "mismatched result",);
}

#[test]
fn bounds() {
    // 8-bit unsigned integer
    let ctx = FixedContext::new(false, 0, 8);
    let zero = ctx.zero();
    let minval = ctx.minval();
    let maxval = ctx.maxval();

    assert_eq!(RFloat::from(zero), RFloat::zero());
    assert_eq!(RFloat::from(minval), RFloat::zero());
    assert_eq!(
        RFloat::from(maxval),
        RFloat::Real(false, 0, Integer::from(255))
    );

    // 8-bit signed integer
    let ctx = FixedContext::new(true, 0, 8);
    let zero = ctx.zero();
    let minval = ctx.minval();
    let maxval = ctx.maxval();

    assert_eq!(RFloat::from(zero), RFloat::zero());
    assert_eq!(
        RFloat::from(minval),
        RFloat::Real(true, 0, Integer::from(128))
    );
    assert_eq!(
        RFloat::from(maxval),
        RFloat::Real(false, 0, Integer::from(127))
    );

    // 8-bit unsigned, scale -4
    let ctx = FixedContext::new(false, -4, 8);
    let zero = ctx.zero();
    let minval = ctx.minval();
    let maxval = ctx.maxval();

    assert_eq!(RFloat::from(zero), RFloat::zero());
    assert_eq!(RFloat::from(minval), RFloat::zero());
    assert_eq!(
        RFloat::from(maxval),
        RFloat::Real(false, -4, Integer::from(255))
    );

    // 8-bit signed integer, scale -4
    let ctx = FixedContext::new(true, -4, 8);
    let zero = ctx.zero();
    let minval = ctx.minval();
    let maxval = ctx.maxval();

    assert_eq!(RFloat::from(zero), RFloat::zero());
    assert_eq!(
        RFloat::from(minval),
        RFloat::Real(true, -4, Integer::from(128))
    );
    assert_eq!(
        RFloat::from(maxval),
        RFloat::Real(false, -4, Integer::from(127))
    );
}

#[test]
fn round_small() {
    let pos_1 = RFloat::Real(false, 0, Integer::from(1));
    let pos_7_8 = RFloat::Real(false, -3, Integer::from(7));
    let pos_3_4 = RFloat::Real(false, -2, Integer::from(3));
    let pos_1_2 = RFloat::Real(false, -1, Integer::from(1));
    let zero = RFloat::zero();

    assert_round_small(false, 1, 4, &pos_1, &zero);
    assert_round_small(false, 0, 4, &pos_1, &pos_1);
    assert_round_small(false, -1, 4, &pos_1, &pos_1);

    assert_round_small(false, 0, 4, &pos_7_8, &zero);
    assert_round_small(false, -1, 4, &pos_7_8, &pos_1_2);
    assert_round_small(false, -2, 4, &pos_7_8, &pos_3_4);
}

#[test]
fn overflow() {
    // 3-bit, unsigned, wrapping
    let ctx = FixedContext::new(false, 0, 3);
    let zero = ctx.zero();
    let delta = ctx.quantum();
    let maxval = ctx.maxval();
    assert_eq!(zero, ctx.add(&maxval, &delta), "should have wrapped");

    // 3-bit, signed, wrapping
    let ctx = FixedContext::new(true, 0, 3);
    let delta = ctx.quantum();
    let minval = ctx.minval();
    let maxval = ctx.maxval();
    assert_eq!(minval, ctx.add(&maxval, &delta), "should have wrapped");

    // 3-bit, unsigned, saturating
    let ctx = FixedContext::new(false, 0, 3).with_overflow(Overflow::Saturate);
    let delta = ctx.quantum();
    let maxval = ctx.maxval();
    assert_eq!(maxval, ctx.add(&maxval, &delta), "should have wrapped");

    // 3-bit, signed, saturating
    let ctx = FixedContext::new(true, 0, 3).with_overflow(Overflow::Saturate);
    let delta = ctx.quantum();
    let maxval = ctx.maxval();
    assert_eq!(maxval, ctx.add(&maxval, &delta), "should have wrapped");
}
