// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754.rs
//
// Tests for the IEEE 754 module

use mpmfnum::ieee754;
use mpmfnum::rational::{Rational, RoundingMode};
use mpmfnum::RoundingContext;
use rug::Integer;

fn assert_round_small(
    input: &Rational,
    rm: RoundingMode,
    output: &Rational,
    overflow: bool,
    underflow_pre: bool,
    underflow_post: bool,
    inexact: bool,
    tiny_pre: bool,
    tiny_post: bool,
    carry: bool,
) {
    let ctx = ieee754::Context::new(2, 5).with_rounding_mode(rm);
    let rounded = ctx.round(input);

    assert_eq!(
        Rational::from(rounded.clone()),
        *output,
        "mismatched result",
    );
    assert_eq!(
        rounded.flags().overflow,
        overflow,
        "mismatched overflow flag"
    );
    assert_eq!(
        rounded.flags().underflow_pre,
        underflow_pre,
        "mismatched underflow flag (before rounding)"
    );
    assert_eq!(
        rounded.flags().underflow_post,
        underflow_post,
        "mismatched underflow flag (after rounding)"
    );
    assert_eq!(rounded.flags().inexact, inexact, "mismatched inexact flag");
    assert_eq!(
        rounded.flags().tiny_pre,
        tiny_pre,
        "mismatched tiny flag (before rounding)"
    );
    assert_eq!(
        rounded.flags().tiny_post,
        tiny_post,
        "mismatched tiny flag (after rounding)"
    );
    assert_eq!(rounded.flags().carry, carry, "mismatched carry flag");
}

#[test]
fn round_small() {
    use RoundingMode::*;

    // test values
    let pos_1 = Rational::Real(false, 0, Integer::from(1));
    let pos_15_16 = Rational::Real(false, -4, Integer::from(15));
    let pos_7_8 = Rational::Real(false, -3, Integer::from(7));
    let pos_3_4 = Rational::Real(false, -2, Integer::from(3));

    let neg_1 = -pos_1.clone();
    let neg_15_16 = -pos_15_16.clone();
    let neg_7_8 = -pos_7_8.clone();
    let neg_3_4 = -pos_3_4.clone();

    // 1
    assert_round_small(
        &pos_1,
        NearestTiesToEven,
        &pos_1,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &pos_1,
        NearestTiesAwayZero,
        &pos_1,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &pos_1, ToPositive, &pos_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &pos_1, ToNegative, &pos_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &pos_1, ToZero, &pos_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &pos_1, AwayZero, &pos_1, false, false, false, false, false, false, false,
    );

    // 15/16
    assert_round_small(
        &pos_15_16,
        NearestTiesToEven,
        &pos_1,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &pos_15_16,
        NearestTiesAwayZero,
        &pos_1,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &pos_15_16, ToPositive, &pos_1, false, true, false, true, true, false, true,
    );
    assert_round_small(
        &pos_15_16, ToNegative, &pos_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &pos_15_16, ToZero, &pos_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &pos_15_16, AwayZero, &pos_1, false, true, false, true, true, false, true,
    );

    // 7/8
    assert_round_small(
        &pos_7_8,
        NearestTiesToEven,
        &pos_1,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &pos_7_8,
        NearestTiesAwayZero,
        &pos_1,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &pos_7_8, ToPositive, &pos_1, false, true, true, true, true, true, true,
    );
    assert_round_small(
        &pos_7_8, ToNegative, &pos_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &pos_7_8, ToZero, &pos_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &pos_7_8, AwayZero, &pos_1, false, true, true, true, true, true, true,
    );

    // 3/4
    assert_round_small(
        &pos_3_4,
        NearestTiesToEven,
        &pos_3_4,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &pos_3_4,
        NearestTiesAwayZero,
        &pos_3_4,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &pos_3_4, ToPositive, &pos_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &pos_3_4, ToNegative, &pos_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &pos_3_4, ToZero, &pos_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &pos_3_4, AwayZero, &pos_3_4, false, false, false, false, true, true, false,
    );

    // -1
    assert_round_small(
        &neg_1,
        NearestTiesToEven,
        &neg_1,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &neg_1,
        NearestTiesAwayZero,
        &neg_1,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &neg_1, ToPositive, &neg_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_1, ToNegative, &neg_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_1, ToZero, &neg_1, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_1, AwayZero, &neg_1, false, false, false, false, false, false, false,
    );

    // 15/16
    assert_round_small(
        &neg_15_16,
        NearestTiesToEven,
        &neg_1,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &neg_15_16,
        NearestTiesAwayZero,
        &neg_1,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &neg_15_16, ToPositive, &neg_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_15_16, ToNegative, &neg_1, false, true, false, true, true, false, true,
    );
    assert_round_small(
        &neg_15_16, ToZero, &neg_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_15_16, AwayZero, &neg_1, false, true, false, true, true, false, true,
    );

    // 7/8
    assert_round_small(
        &neg_7_8,
        NearestTiesToEven,
        &neg_1,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &neg_7_8,
        NearestTiesAwayZero,
        &neg_1,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &neg_7_8, ToPositive, &neg_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_7_8, ToNegative, &neg_1, false, true, true, true, true, true, true,
    );
    assert_round_small(
        &neg_7_8, ToZero, &neg_3_4, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_7_8, AwayZero, &neg_1, false, true, true, true, true, true, true,
    );

    // 3/4
    assert_round_small(
        &neg_3_4,
        NearestTiesToEven,
        &neg_3_4,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &neg_3_4,
        NearestTiesAwayZero,
        &neg_3_4,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &neg_3_4, ToPositive, &neg_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_3_4, ToNegative, &neg_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_3_4, ToZero, &neg_3_4, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_3_4, AwayZero, &neg_3_4, false, false, false, false, true, true, false,
    );
}

// #[test]
// fn mul_small() {
//     let ctx = ieee754::Context::new(2, 4);
// }
