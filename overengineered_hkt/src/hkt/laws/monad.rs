use tap::Pipe as _;

use crate::hkt::Monad;

#[must_use]
pub fn left_identity_law<'a, F: Monad<'a>>(
    a: i32,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    eq(F::bind(F::pure(a), F::pure), F::pure(a))
}

/// TODO: Is this impl of law correct?
#[must_use]
pub fn right_identity_law<'a, F: Monad<'a>>(
    a: i32,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    eq(F::bind(F::pure(F::pure::<i32>), move |f| f(a)), F::pure(a))
}

#[must_use]
pub fn associativity_law<'a, F: Monad<'a>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    let f1 = move |a| F::pure(f1(a));
    let f2 = move |a| F::pure(f2(a));

    eq(
        F::bind(F::pure(a), f1).pipe(|a| F::bind(a, f2)),
        F::bind(F::pure(a), move |a| F::bind(f1(a), f2)),
    )
}
