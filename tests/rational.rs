use rug::Integer;
use std::cmp::Ordering;

use mpmfnum::rational::*;
use mpmfnum::{Real, RoundingContext, RoundingMode};

/// Testing all the required methods from [`mpmfnum::Number`].
#[test]
fn traits() {
    assert_eq!(Rational::radix(), 2, "Rational is a binary format");

    let vals = [
        Rational::zero(),                           // 0
        Rational::one(),                            // 1
        Rational::Real(true, -4, Integer::from(7)), // -7 * 2^-4
        Rational::Infinite(false),                  // +Inf
        Rational::Infinite(true),                   // -Inf,
        Rational::Nan,                              // NaN
    ];

    // Rational::sign
    let expected = [false, false, true, false, true, false];
    for (val, &expected) in vals.iter().zip(expected.iter()) {
        let actual = val.sign();
        assert_eq!(
            actual, expected,
            "{:?} has unexpected sign; expected {}, actual {}",
            val, expected, actual
        );
    }

    // Rational::exp
    let expected = [None, Some(0), Some(-4), None, None, None];
    for (val, &expected) in vals.iter().zip(expected.iter()) {
        let actual = val.exp();
        assert_eq!(
            actual, expected,
            "{:?} has unexpected exponent (exp); expected {:?}, actual {:?}",
            val, expected, actual
        );
    }

    // Rational::e
    let expected = [None, Some(0), Some(-2), None, None, None];
    for (val, &expected) in vals.iter().zip(expected.iter()) {
        let actual = val.e();
        assert_eq!(
            actual, expected,
            "{:?} has unexpected exponent (e); expected {:?}, actual {:?}",
            val, expected, actual
        );
    }

    // Rational::n
    let expected = [None, Some(-1), Some(-5), None, None, None];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.n();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} has unexpected least significant exponent (n); expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::c
    let expected = [
        Some(Integer::from(0)),
        Some(Integer::from(1)),
        Some(Integer::from(7)),
        None,
        None,
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.c();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} has unexpected significand (c): expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::m
    let expected = [
        Some(Integer::from(0)),
        Some(Integer::from(1)),
        Some(Integer::from(-7)),
        None,
        None,
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.m();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} has unexpected significand (m): expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::p
    let expected = [0, 1, 3, 0, 0, 0];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.p();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} has unexpected precision (p): expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::is_nar
    let expected = [false, false, false, true, true, true];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.is_nar();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} is unexpectedly not-a-real (NAR): expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::is_finite
    let expected = [true, true, true, false, false, false];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.is_finite();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?}  is unexpectedly finite: expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::is_infinite
    let expected = [false, false, false, true, true, false];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.is_infinite();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} is unexpectedly infinite: expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::is_zero
    let expected = [true, false, false, false, false, false];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.is_zero();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} is unexpectedly zero: expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }

    // Rational::is_negative
    let expected = [None, Some(false), Some(true), Some(false), Some(true), None];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        let actual = val.is_negative();
        assert_eq!(
            actual,
            expected.clone(),
            "{:?} is unexpectedly zero: expected {:?}, actual {:?}",
            val,
            expected,
            actual
        );
    }
}

/// Testing rounding for easy cases
#[test]
fn round_trivial() {
    // rounding context
    let ctx = RationalContext::new().with_max_precision(1);

    // round(zero) = round
    let zero = Rational::zero();
    let (_, n) = ctx.round_params(&zero);
    let (_, err) = RationalContext::split_at(&zero, n);
    let rounded_zero = ctx.round(&zero);
    assert!(rounded_zero.is_zero(), "round(0) = 0");
    assert!(
        err.is_zero(),
        "rounding 0 should have a zero lost bits"
    );

    // round(+Inf) = +Inf
    let rounded_pos_inf = ctx.round(&POS_INF);
    assert!(rounded_pos_inf.is_infinite(), "round(+Inf) = +Inf");

    // round(-Inf) = -Inf
    let rounded_neg_inf = ctx.round(&NEG_INF);
    assert!(rounded_neg_inf.is_infinite(), "round(-Inf) = -Inf");

    // round(Nan) = Nan
    let rounded_nan = ctx.round(&NAN);
    assert!(rounded_nan.is_nar(), "round(-Nan) = Nan");
}

/// Testing rounding using fixed-point rounding
#[test]
fn round_fixed() {
    let one_3_4 = Rational::Real(false, -2, Integer::from(7));
    let one_1_2 = Rational::Real(false, -1, Integer::from(3));
    let one = Rational::one();
    let three_4 = Rational::Real(false, -2, Integer::from(3));
    let one_4 = Rational::Real(false, -2, Integer::from(1));
    let zero = Rational::zero();

    let neg_one = Rational::Real(true, 0, Integer::from(1));

    // 1 (min_n == -1) => 1
    let ctx = RationalContext::new()
        .with_min_n(-1)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&one);
    let (_, err) = RationalContext::split_at(&one, n);
    let rounded_one = ctx.round(&one);
    assert_eq!(
        rounded_one,
        Rational::one(),
        "rounding should not have lost bits"
    );
    assert!(err.is_zero(), "lost bits should be 0");

    // 1 (min_n == 0) => 0
    let ctx = RationalContext::new()
        .with_min_n(0)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&one);
    let (_, err) = RationalContext::split_at(&one, n);
    let rounded_one = ctx.round(&one);
    assert_eq!(rounded_one, zero, "rounding should truncated to 0");
    assert_eq!(err, Rational::one(), "lost bits should be 1");

    // -1 (min_n == 0) => 0
    let ctx = RationalContext::new()
        .with_min_n(0)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&neg_one);
    let (_, err) = RationalContext::split_at(&neg_one, n);
    let rounded_one = ctx.round(&neg_one);
    assert_eq!(rounded_one, zero, "rounding should truncated to 0");
    assert_eq!(err, neg_one, "lost bits should be -1");

    // 1.75 (min_n == -1) => 1
    let ctx = RationalContext::new()
        .with_min_n(-1)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&one_3_4);
    let (_, err) = RationalContext::split_at(&one_3_4, n);
    let rounded = ctx.round(&one_3_4);
    assert_eq!(rounded, one, "rounding should truncated to 0");
    assert_eq!(err, three_4, "lost bits should be 3/4");

    // 1.75 (min_n == -2) => 1.5
    let ctx = RationalContext::new()
        .with_min_n(-2)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&one_3_4);
    let (_, err) = RationalContext::split_at(&one_3_4, n);
    let rounded = ctx.round(&one_3_4);
    assert_eq!(rounded, one_1_2, "rounding should truncated to 0");
    assert_eq!(err, one_4, "lost bits should be 1/4");

    // 1 (min_n == 10) => 0
    let ctx = RationalContext::new()
        .with_min_n(10)
        .with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&one);
    let (_, err) = RationalContext::split_at(&one, n);
    let rounded = ctx.round(&one);
    assert_eq!(rounded, zero, "rounding should truncated to 0");
    assert_eq!(err, one, "lost bits should be 1");
}

/// Testing rounding using floating-point rounding
#[test]
fn round_float() {
    let one_1_2 = Rational::Real(false, -1, Integer::from(3));
    let one_1_4 = Rational::Real(false, -2, Integer::from(5));
    let one_1_8 = Rational::Real(false, -3, Integer::from(9));
    let one = Rational::one();
    let one_4 = Rational::Real(false, -2, Integer::from(1));
    let one_8 = Rational::Real(false, -3, Integer::from(1));
    let zero = Rational::zero();

    // 1.25, 3 bits

    // rounding 1.25 with 3 bits, exact
    let ctx = RationalContext::new().with_max_precision(3);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one_1_4, "rounding should be exact");
    assert_eq!(err, zero, "lost bits is zero");

    // 1.25, 2 bits

    // rounding 1.25 with 2 bits, round-to-nearest
    let ctx = ctx.with_max_precision(2);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // rounding 1.25 with 2 bits, round-to-positive
    let ctx = ctx.with_rounding_mode(RoundingMode::ToPositive);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one_1_2, "rounding goes to 3/2");
    assert_eq!(err, one_4, "lost bits is -1/4");

    // rounding 1.25 with 2 bits, round-to-negative
    let ctx = ctx.with_rounding_mode(RoundingMode::ToNegative);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // rounding 1.25 with 2 bits, round-to-even
    let ctx = ctx.with_rounding_mode(RoundingMode::ToEven);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // rounding 1.25 with 2 bits, round-to-odd
    let ctx = ctx.with_rounding_mode(RoundingMode::ToOdd);
    let (_, n) = ctx.round_params(&one_1_4);
    let (_, err) = RationalContext::split_at(&one_1_4, n);
    let rounded = ctx.round(&one_1_4);
    assert_eq!(rounded, one_1_2, "rounding goes to 3/2");
    assert_eq!(err, one_4, "lost bits is -1/4");

    // 1.125, 2 bit

    // rounding 1.125 with 2 bits, round-to-nearest
    let ctx = ctx.with_rounding_mode(RoundingMode::NearestTiesToEven);
    let (_, n) = ctx.round_params(&one_1_8);
    let (_, err) = RationalContext::split_at(&one_1_8, n);
    let rounded = ctx.round(&one_1_8);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // rounding 1.125 with 2 bits, round-to-positive
    let ctx = ctx.with_rounding_mode(RoundingMode::ToPositive);
    let (_, n) = ctx.round_params(&one_1_8);
    let (_, err) = RationalContext::split_at(&one_1_8, n);
    let rounded = ctx.round(&one_1_8);
    assert_eq!(rounded, one_1_2, "rounding goes to 3/2");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // rounding 1.125 with 2 bits, round-to-negative
    let ctx = ctx.with_rounding_mode(RoundingMode::ToNegative);
    let (_, n) = ctx.round_params(&one_1_8);
    let (_, err) = RationalContext::split_at(&one_1_8, n);
    let rounded = ctx.round(&one_1_8);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // rounding 1.125 with 2 bits, round-to-even
    let ctx = ctx.with_rounding_mode(RoundingMode::ToEven);
    let (_, n) = ctx.round_params(&one_1_8);
    let (_, err) = RationalContext::split_at(&one_1_8, n);
    let rounded = ctx.round(&one_1_8);
    assert_eq!(rounded, one, "rounding goes to 1");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // rounding 1.125 with 2 bits, round-to-odd
    let ctx = ctx.with_rounding_mode(RoundingMode::ToOdd);
    let (_, n) = ctx.round_params(&one_1_8);
    let (_, err) = RationalContext::split_at(&one_1_8, n);
    let rounded = ctx.round(&one_1_8);
    assert_eq!(rounded, one_1_2, "rounding goes to 3/2");
    assert_eq!(err, one_8, "lost bits is -3/8");
}

/// Testing rounding using floating-point rounding using subnormals
#[test]
fn round_float_subnorm() {
    let one = Rational::one();
    let half_way = Rational::Real(false, -3, Integer::from(7));
    let tiny_val = Rational::Real(false, -2, Integer::from(3));
    let one_2 = Rational::Real(false, -1, Integer::from(1));
    let one_4 = Rational::Real(false, -2, Integer::from(1));
    let one_8 = Rational::Real(false, -3, Integer::from(1));

    // No subnormals, round-to-nearest
    let ctx = RationalContext::new().with_max_precision(2);
    let (_, n) = ctx.round_params(&half_way);
    let (_, err) = RationalContext::split_at(&half_way, n);
    let rounded = ctx.round(&half_way);
    assert_eq!(one, rounded, "rounding to 1");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // No subnormals, round-away-zero
    let ctx = ctx.with_rounding_mode(RoundingMode::AwayZero);
    let (_, n) = ctx.round_params(&half_way);
    let (_, err) = RationalContext::split_at(&half_way, n);
    let rounded = ctx.round(&half_way);
    assert_eq!(one, rounded, "rounding to 1");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // No subnormals, round-to-zero
    let ctx = ctx.with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&half_way);
    let (_, err) = RationalContext::split_at(&half_way, n);
    let rounded = ctx.round(&half_way);
    assert_eq!(tiny_val, rounded, "rounding to 3/4");
    assert_eq!(err, one_8, "lost bits is 1/8");

    // Rational<2, 4>, round-to-nearest
    let ctx = RationalContext::new().with_max_precision(2).with_min_n(-2);
    let (_, n) = ctx.round_params(&tiny_val);
    let (_, err) = RationalContext::split_at(&tiny_val, n);
    let rounded = ctx.round(&tiny_val);
    assert_eq!(one, rounded, "rounding to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // Rational<2, 4>, round-away-zero
    let ctx = ctx.with_rounding_mode(RoundingMode::AwayZero);
    let (_, n) = ctx.round_params(&tiny_val);
    let (_, err) = RationalContext::split_at(&tiny_val, n);
    let rounded = ctx.round(&tiny_val);
    assert_eq!(one, rounded, "rounding to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // Rational<2, 4>, round-to-zero
    let ctx = ctx.with_rounding_mode(RoundingMode::ToZero);
    let (_, n) = ctx.round_params(&tiny_val);
    let (_, err) = RationalContext::split_at(&tiny_val, n);
    let rounded = ctx.round(&tiny_val);
    assert_eq!(one_2, rounded, "rounding to 1/2");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // Rational<2, 4>, round-to-even
    let ctx = ctx.with_rounding_mode(RoundingMode::ToEven);
    let (_, n) = ctx.round_params(&tiny_val);
    let (_, err) = RationalContext::split_at(&tiny_val, n);
    let rounded = ctx.round(&tiny_val);
    assert_eq!(one, rounded, "rounding to 1");
    assert_eq!(err, one_4, "lost bits is 1/4");

    // Rational<2, 4>, round-to-odd
    let ctx = ctx.with_rounding_mode(RoundingMode::ToOdd);
    let (_, n) = ctx.round_params(&tiny_val);
    let (_, err) = RationalContext::split_at(&tiny_val, n);
    let rounded = ctx.round(&tiny_val);
    assert_eq!(one_2, rounded, "rounding to 1/2");
    assert_eq!(err, one_4, "lost bits is 1/4");
}

fn assert_expected_cmp(x: &Rational, y: &Rational, expected: &Option<Ordering>) {
    let actual = x.partial_cmp(y);
    assert_eq!(
        actual,
        expected.clone(),
        "unexpected comparison result between {:?} and {:?}: expected {:?}, actual {:?}",
        x,
        y,
        expected,
        actual
    );
}

#[test]
fn ordering() {
    // values to compare against
    let vals = [
        Rational::zero(),
        Rational::one(),
        POS_INF.clone(),
        NEG_INF.clone(),
        NAN.clone(),
    ];

    // compare with 0
    let zero = Rational::zero();
    let expected = [
        Some(Ordering::Equal),
        Some(Ordering::Less),
        Some(Ordering::Less),
        Some(Ordering::Greater),
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        assert_expected_cmp(&zero, val, expected);
    }

    // compare with 1
    let one = Rational::one();
    let expected = [
        Some(Ordering::Greater),
        Some(Ordering::Equal),
        Some(Ordering::Less),
        Some(Ordering::Greater),
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        assert_expected_cmp(&one, val, expected);
    }

    // compare with +Inf
    let expected = [
        Some(Ordering::Greater),
        Some(Ordering::Greater),
        Some(Ordering::Equal),
        Some(Ordering::Greater),
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        assert_expected_cmp(&POS_INF, val, expected);
    }

    // compare with -Inf
    let expected = [
        Some(Ordering::Less),
        Some(Ordering::Less),
        Some(Ordering::Less),
        Some(Ordering::Equal),
        None,
    ];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        assert_expected_cmp(&NEG_INF, val, expected);
    }

    // compare with Nan
    let expected = [None, None, None, None, None];
    for (val, expected) in vals.iter().zip(expected.iter()) {
        assert_expected_cmp(&NAN, val, expected);
    }

    // test normalization
    let one = Rational::one();
    let also_one = Rational::Real(false, -1, Integer::from(2));
    assert_eq!(
        one.partial_cmp(&also_one),
        Some(Ordering::Equal),
        "should be the same"
    );

    let still_one = Rational::Real(false, -2, Integer::from(4));
    assert_eq!(
        one.partial_cmp(&still_one),
        Some(Ordering::Equal),
        "should be the same"
    );
}

fn is_equal(x: &Rational, y: &Rational) -> bool {
    match (x, y) {
        (Rational::Nan, Rational::Nan) => true,
        (_, _) => *x == *y,
    }
}

fn assert_expected_mul(x: &Rational, y: &Rational, expected: &Rational) {
    let left = x.clone() * y.clone();
    let right = y.clone() * x.clone();
    assert!(
        is_equal(&left, expected),
        "for {:?} * {:?}: expected {:?}, actual {:?}",
        x,
        y,
        expected,
        left
    );
    assert!(
        is_equal(&left, expected),
        "multiplication is commutative: {:?} != {:?}",
        left,
        right
    );
}

#[test]
fn multiplication() {
    // test values
    let zero = Rational::zero(); // 0
    let one = Rational::one(); // 1
    let frac = Rational::Real(true, -4, Integer::from(7)); // -7 * 2^-4
    let pos_inf = POS_INF; // +Inf
    let neg_inf = NEG_INF; // -Inf,
    let nan = NAN; // NaN

    let vals = [&zero, &one, &frac, &pos_inf, &neg_inf, &nan];

    // Multiply by 0
    let expected = [&zero, &zero, &zero, &nan, &nan, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&zero, val, expected);
    }

    // Multiply by 1
    let expected = [&zero, &one, &frac, &pos_inf, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&one, val, expected);
    }

    // Multiply by -7 * 2^-4
    let frac_sqr = Rational::Real(false, -8, Integer::from(49));
    let expected = [&zero, &frac, &frac_sqr, &neg_inf, &pos_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&frac, val, expected);
    }

    // Multiply by +Inf
    let expected = [&nan, &pos_inf, &neg_inf, &pos_inf, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&pos_inf, val, expected);
    }

    // Multiply by -Inf
    let expected = [&nan, &neg_inf, &pos_inf, &neg_inf, &pos_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&neg_inf, val, expected);
    }

    // Multiply by Nan
    let expected = [&nan; 6];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_mul(&nan, val, expected);
    }
}

fn assert_expected_add(x: &Rational, y: &Rational, expected: &Rational) {
    let left = x.clone() + y.clone();
    let right = y.clone() + x.clone();
    assert!(
        is_equal(&left, expected),
        "for {:?} + {:?}: expected {:?}, actual {:?}",
        x,
        y,
        expected,
        left
    );
    assert!(
        is_equal(&left, expected),
        "addition is commutative: {:?} != {:?}",
        left,
        right
    );
}

#[test]
fn addition() {
    // test values
    let zero = Rational::zero(); // 0
    let one = Rational::one(); // 1
    let frac = Rational::Real(true, -4, Integer::from(7)); // -7 * 2^-4
    let pos_inf = POS_INF; // +Inf
    let neg_inf = NEG_INF; // -Inf,
    let nan = NAN; // NaN

    let two = Rational::Real(false, 0, Integer::from(2)); // 2
    let two_frac = Rational::Real(true, -4, Integer::from(14)); // 14 * 2^-4
    let one_m_frac = Rational::Real(false, -4, Integer::from(9)); // 9 * 2^-4

    let vals = [&zero, &one, &frac, &pos_inf, &neg_inf, &nan];

    // Add by 0
    let expected = [&zero, &one, &frac, &pos_inf, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&zero, val, expected);
    }

    // Add by 1
    let expected = [&one, &two, &one_m_frac, &pos_inf, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&one, val, expected);
    }

    // Add by -7 * 2^-4
    let expected = [&frac, &one_m_frac, &two_frac, &pos_inf, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&frac, val, expected);
    }

    // Add by +Inf
    let expected = [&pos_inf, &pos_inf, &pos_inf, &pos_inf, &nan, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&pos_inf, val, expected);
    }

    // Add by -Inf
    let expected = [&neg_inf, &neg_inf, &neg_inf, &nan, &neg_inf, &nan];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&neg_inf, val, expected);
    }

    // Add by Nan
    let expected = [&nan; 6];
    for (&val, &expected) in vals.iter().zip(expected.iter()) {
        assert_expected_add(&nan, val, expected);
    }
}

#[test]
fn neg() {
    // test values
    let zero = Rational::zero(); // 0
    let one = Rational::one(); // 1
    let frac = Rational::Real(true, -4, Integer::from(7)); // -7 * 2^-4
    let pos_inf = POS_INF; // +Inf
    let neg_inf = NEG_INF; // -Inf,
    let nan = NAN; // NaN

    let neg_zero = -zero;
    let neg_one = -one;
    let neg_frac = -frac;
    let neg_pos_inf = -pos_inf;
    let neg_neg_inf = -neg_inf;
    let neg_nan = nan;

    assert!(!neg_zero.sign(), "-0 should not have a sign");
    assert!(neg_one.sign(), "-1 is signed");
    assert!(!neg_frac.sign(), "-(-7 * 2^-4) is not signed");
    assert!(neg_pos_inf.sign(), "-(+Inf) is signed");
    assert!(!neg_neg_inf.sign(), "-(-Inf) is not signed");
    assert!(!neg_nan.sign(), "-Nan is not signed");
}

#[test]
fn mpfr_integration() {
    // test values
    let zero = Rational::zero(); // 0
    let one = Rational::one(); // 1
    let frac = Rational::Real(true, -4, Integer::from(7)); // -7 * 2^-4
    let pos_inf = POS_INF; // +Inf
    let neg_inf = NEG_INF; // -Inf,
    let nan = NAN; // NaN

    let vals = [zero, one, frac, pos_inf, neg_inf, nan];

    for val in &vals {
        let f: Rational = val.clone().into();
        let val2 = Rational::from(f);
        assert!(
            is_equal(val, &val2),
            "conversion should have been exact: {:?} != {:?}",
            val,
            val2
        );
    }
}
