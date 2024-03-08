use num_traits::One;
use rug::Integer;

use mpmfnum::ieee754::IEEE754Context;
use mpmfnum::ops::*;
use mpmfnum::*;

trait QuadraticCtx:
    RoundingContext + RoundedNeg + RoundedAdd + RoundedSub + RoundedMul + RoundedDiv + RoundedSqrt
{
}

impl<T> QuadraticCtx for T where
    T: RoundingContext
        + RoundedNeg
        + RoundedAdd
        + RoundedSub
        + RoundedMul
        + RoundedDiv
        + RoundedSqrt
{
}

fn naive_quadp<Ctx>(a: &Ctx::Format, b: &Ctx::Format, c: &Ctx::Format, ctx: &Ctx) -> Ctx::Format
where
    Ctx: QuadraticCtx,
{
    let two = RFloat::Real(false, 1, Integer::one());
    let four = RFloat::Real(false, 2, Integer::one());
    let discr = ctx.sqrt(&ctx.sub(&ctx.mul(b, b), &ctx.mul(&four, &ctx.mul(a, c))));
    ctx.div(&ctx.add(&ctx.neg(b), &discr), &ctx.mul(&two, a))
}

#[test]
fn run() {
    let ctx = IEEE754Context::new(8, 32);
    let a = ctx.zero(false);
    let b = ctx.zero(false);
    let c = ctx.zero(false);

    let r = naive_quadp(&a, &b, &c, &ctx);
}
