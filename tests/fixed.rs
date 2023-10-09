use mpmfnum::rfloat::RFloat;
use mpmfnum::{fixed, RoundingContext};
use rug::Integer;

fn assert_round_small(signed: bool, scale: isize, nbits: usize, input: &RFloat, output: &RFloat) {
    let ctx = fixed::FixedContext::new(signed, scale, nbits);
    let rounded = ctx.round(input);

    assert_eq!(RFloat::from(rounded.clone()), *output, "mismatched result",);
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
