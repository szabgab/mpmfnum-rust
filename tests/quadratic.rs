use num_traits::One;
use rug::Integer;

use mpmfnum::ieee754::IEEE754Context;
use mpmfnum::ops::*;
use mpmfnum::*;

context_alias!(
    QuadraticCtx,
    RoundedNeg + RoundedAdd + RoundedSub + RoundedMul + RoundedDiv + RoundedSqrt
);

fn naive_quad<Ctx>(
    a: &Ctx::Format,
    b: &Ctx::Format,
    c: &Ctx::Format,
    ctx: &Ctx,
) -> (Ctx::Format, Ctx::Format)
where
    Ctx: QuadraticCtx,
{
    let two = RFloat::Real(false, 1, Integer::one());
    let four = RFloat::Real(false, 2, Integer::one());
    let discr = ctx.sqrt(&ctx.sub(&ctx.mul(b, b), &ctx.mul(&four, &ctx.mul(a, c))));
    let pos = ctx.add(&ctx.neg(b), &discr);
    let neg = ctx.sub(&ctx.neg(b), &discr);
    let factor = ctx.mul(&two, a);
    (ctx.div(&pos, &factor), ctx.div(&neg, &factor))
}

#[test]
fn run() {
    let ctx = IEEE754Context::new(8, 32);
    let a = ctx.zero(false);
    let b = ctx.zero(false);
    let c = ctx.zero(false);

    let (pos, neg) = naive_quad(&a, &b, &c, &ctx);
    println!("{:?} {:?}", pos, neg);
}
