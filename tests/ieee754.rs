// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// ieee754.rs
//
// Tests for the IEEE 754 module

use mpmfnum::rational::{Rational, RoundingMode};
use mpmfnum::RoundingContext;
use mpmfnum::{ieee754, Number};
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

#[test]
fn bits_small() {
    let ctx = ieee754::Context::new(2, 5);

    // 0
    let num = ctx.from_bits(Integer::from(0));
    assert!(num.is_zero(), "0 is zero");
    // 1
    let num = ctx.from_bits(Integer::from(1));
    assert!(num.is_subnormal(), "1 is subnormal");
    assert_eq!(num.c().unwrap(), Integer::from(1), "mantissa is 1");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 2
    let num = ctx.from_bits(Integer::from(2));
    assert!(num.is_subnormal(), "2 is subnormal");
    assert_eq!(num.c().unwrap(), Integer::from(2), "mantissa is 2");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 3
    let num = ctx.from_bits(Integer::from(3));
    assert!(num.is_subnormal(), "3 is subnormal");
    assert_eq!(num.c().unwrap(), Integer::from(3), "mantissa is 3");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 4
    let num = ctx.from_bits(Integer::from(4));
    assert!(num.is_normal(), "4 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(4), "mantissa is 4");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 5
    let num = ctx.from_bits(Integer::from(5));
    assert!(num.is_normal(), "5 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(5), "mantissa is 5");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 6
    let num = ctx.from_bits(Integer::from(6));
    assert!(num.is_normal(), "6 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(6), "mantissa is 6");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 7
    let num = ctx.from_bits(Integer::from(7));
    assert!(num.is_normal(), "5 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(7), "mantissa is 7");
    assert_eq!(num.exp().unwrap(), -2, "exponent is -2");
    // 8
    let num = ctx.from_bits(Integer::from(8));
    assert!(num.is_normal(), "8 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(4), "mantissa is 4");
    assert_eq!(num.exp().unwrap(), -1, "exponent is -1");
    // 9
    let num = ctx.from_bits(Integer::from(9));
    assert!(num.is_normal(), "9 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(5), "mantissa is 5");
    assert_eq!(num.exp().unwrap(), -1, "exponent is -1");
    // 10
    let num = ctx.from_bits(Integer::from(10));
    assert!(num.is_normal(), "10 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(6), "mantissa is 6");
    assert_eq!(num.exp().unwrap(), -1, "exponent is -1");
    // 11
    let num = ctx.from_bits(Integer::from(11));
    assert!(num.is_normal(), "11 is normal");
    assert_eq!(num.c().unwrap(), Integer::from(7), "mantissa is 7");
    assert_eq!(num.exp().unwrap(), -1, "exponent is -1");
    // 12
    let num = ctx.from_bits(Integer::from(12));
    assert!(num.is_infinite(), "12 is infinity");
    // 13
    let num = ctx.from_bits(Integer::from(13));
    assert!(num.is_nan(), "13 is NaN");
    assert!(!num.nan_quiet().unwrap(), "13 is a signaling NaN");
    assert!(num.nan_payload().unwrap() == 1, "13 has a payload of 1");
    // 14
    let num = ctx.from_bits(Integer::from(14));
    assert!(num.is_nan(), "14 is NaN");
    assert!(num.nan_quiet().unwrap(), "14 is a quiet NaN");
    assert!(num.nan_payload().unwrap() == 0, "14 has a payload of 0");
    // 15
    let num = ctx.from_bits(Integer::from(15));
    assert!(num.is_nan(), "15 is NaN");
    assert!(num.nan_quiet().unwrap(), "15 is a quiet NaN");
    assert!(num.nan_payload().unwrap() == 1, "15 has a payload of 1");
}
