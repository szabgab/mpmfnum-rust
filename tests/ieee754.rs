// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754.rs
//
// Tests for the IEEE 754 module

use gmp::mpz::Mpz;
use mpmfnum::ieee754;
use mpmfnum::rational::{Rational, RoundingMode};
use mpmfnum::RoundingContext;

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
    let ctx = ieee754::Context::new(2, 4).with_rounding_mode(rm);
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
    let one = Rational::Real(false, 0, Mpz::from(1));
    let seven_8 = Rational::Real(false, -3, Mpz::from(7));
    let three_4 = Rational::Real(false, -2, Mpz::from(3));
    let one_2 = Rational::Real(false, -1, Mpz::from(1));

    let neg_one = -one.clone();
    let neg_7_8 = -seven_8.clone();
    let neg_3_4 = -three_4.clone();
    let neg_1_2 = -one_2.clone();

    // 1
    assert_round_small(
        &one,
        NearestTiesToEven,
        &one,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &one,
        NearestTiesAwayZero,
        &one,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &one, ToPositive, &one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &one, ToNegative, &one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &one, ToZero, &one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &one, AwayZero, &one, false, false, false, false, false, false, false,
    );

    // 7/8
    assert_round_small(
        &seven_8,
        NearestTiesToEven,
        &one,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &seven_8,
        NearestTiesAwayZero,
        &one,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &seven_8, ToPositive, &one, false, true, false, true, true, false, true,
    );
    assert_round_small(
        &seven_8, ToNegative, &one_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &seven_8, ToZero, &one_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &seven_8, AwayZero, &one, false, true, false, true, true, false, true,
    );

    // 3/4
    assert_round_small(
        &three_4,
        NearestTiesToEven,
        &one,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &three_4,
        NearestTiesAwayZero,
        &one,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &three_4, ToPositive, &one, false, true, true, true, true, true, true,
    );
    assert_round_small(
        &three_4, ToNegative, &one_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &three_4, ToZero, &one_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &three_4, AwayZero, &one, false, true, true, true, true, true, true,
    );

    // 1/2
    assert_round_small(
        &one_2,
        NearestTiesToEven,
        &one_2,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &one_2,
        NearestTiesAwayZero,
        &one_2,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &one_2, ToPositive, &one_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &one_2, ToNegative, &one_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &one_2, ToZero, &one_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &one_2, AwayZero, &one_2, false, false, false, false, true, true, false,
    );

    // -1
    assert_round_small(
        &neg_one,
        NearestTiesToEven,
        &neg_one,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &neg_one,
        NearestTiesAwayZero,
        &neg_one,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    assert_round_small(
        &neg_one, ToPositive, &neg_one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_one, ToNegative, &neg_one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_one, ToZero, &neg_one, false, false, false, false, false, false, false,
    );
    assert_round_small(
        &neg_one, AwayZero, &neg_one, false, false, false, false, false, false, false,
    );

    // 7/8
    assert_round_small(
        &neg_7_8,
        NearestTiesToEven,
        &neg_one,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &neg_7_8,
        NearestTiesAwayZero,
        &neg_one,
        false,
        true,
        false,
        true,
        true,
        false,
        true,
    );
    assert_round_small(
        &neg_7_8, ToPositive, &neg_1_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_7_8, ToNegative, &neg_one, false, true, false, true, true, false, true,
    );
    assert_round_small(
        &neg_7_8, ToZero, &neg_1_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_7_8, AwayZero, &neg_one, false, true, false, true, true, false, true,
    );

    // 3/4
    assert_round_small(
        &neg_3_4,
        NearestTiesToEven,
        &neg_one,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &neg_3_4,
        NearestTiesAwayZero,
        &neg_one,
        false,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    assert_round_small(
        &neg_3_4, ToPositive, &neg_1_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_3_4, ToNegative, &neg_one, false, true, true, true, true, true, true,
    );
    assert_round_small(
        &neg_3_4, ToZero, &neg_1_2, false, true, true, true, true, true, false,
    );
    assert_round_small(
        &neg_3_4, AwayZero, &neg_one, false, true, true, true, true, true, true,
    );

    // 1/2
    assert_round_small(
        &neg_1_2,
        NearestTiesToEven,
        &neg_1_2,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &neg_1_2,
        NearestTiesAwayZero,
        &neg_1_2,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
    );
    assert_round_small(
        &neg_1_2, ToPositive, &neg_1_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_1_2, ToNegative, &neg_1_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_1_2, ToZero, &neg_1_2, false, false, false, false, true, true, false,
    );
    assert_round_small(
        &neg_1_2, AwayZero, &neg_1_2, false, false, false, false, true, true, false,
    );
}
