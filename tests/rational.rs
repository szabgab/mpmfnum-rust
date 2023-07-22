// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// rational.rs
//
// Tests for the rational module

use gmp::mpz::Mpz;

use mpmfnum::rational::*;
use mpmfnum::Number;

/// Testing all the required methods from [`mpmfnum::Number`].
#[test]
fn traits() {
    assert_eq!(Rational::radix(), 2, "Rational is a binary format");

    let vals = [
        Rational::zero(),                       // 0
        Rational::one(),                        // 1
        Rational::Real(true, -4, Mpz::from(7)), // 7 * 2^-4
        Rational::Infinite(false),              // +Inf
        Rational::Infinite(true),               // -Inf,
        Rational::Nan,                          // NaN
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
        Some(Mpz::from(0)),
        Some(Mpz::from(1)),
        Some(Mpz::from(7)),
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
        Some(Mpz::from(0)),
        Some(Mpz::from(1)),
        Some(Mpz::from(-7)),
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
    let ctx = Context::new().with_max_precision(1);

    // round(zero) = round
    let zero = Rational::zero();
    let (rounded_zero, err) = ctx.round(&zero);
    assert!(rounded_zero.is_zero(), "round(0) = 0");
    assert!(err.is_none(), "rounding 0 should have no error");

    // round(+Inf) = +Inf
    let (rounded_pos_inf, err) = ctx.round(&POS_INF);
    assert!(rounded_pos_inf.is_infinite(), "round(+Inf) = +Inf");
    assert!(err.is_none(), "rounding +Inf should have no error");

    // round(-Inf) = -Inf
    let (rounded_neg_inf, err) = ctx.round(&NEG_INF);
    assert!(rounded_neg_inf.is_infinite(), "round(-Inf) = -Inf");
    assert!(err.is_none(), "rounding -Inf should have no error");

    // round(Nan) = Nan
    let (rounded_nan, err) = ctx.round(&NAN);
    assert!(rounded_nan.is_nar(), "round(-Nan) = Nan");
    assert!(err.is_none(), "rounding Nan should have no error");
}

/// Testing rounding using fixed-point rounding
#[test]
fn round_fixed() {
    // 1 (min_n == -1)
    let ctx = Context::new().with_min_n(-1);
    let one = Rational::Real(false, -2, Mpz::from(4));
    let (rounded_one, _) = ctx.round(&one);
    assert_eq!(rounded_one, one, "rounding should not have lost bits");

    // 1 (min_n == 0)
    let ctx = Context::new().with_min_n(0);
    let one = Rational::Real(false, -2, Mpz::from(4));
    let (rounded_one, _) = ctx.round(&one);
    assert_eq!(rounded_one, Rational::zero(), "rounding should have truncated to 0");

}
