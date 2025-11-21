use std::convert::identity;

use tap::Pipe as _;

use crate::hkt::{Applicative, DependentCloneK, Functor};

#[must_use]
pub fn homomorphism_law<'a, F: Applicative<'a>>(
    a: i32,
    f: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    eq(F::apply(F::pure(f), F::pure(a)), F::pure(f(a)))
}

#[must_use]
pub fn interchange_law<'a, F: Applicative<'a>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    #[inline]
    pub fn inner<'a, F: Applicative<'a>, F1>(
        a: i32,
        f1: F1,
        eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
    ) -> bool
    where
        F1: 'a + Fn(i32) -> i32 + Copy,
    {
        let ap = |x| move |f: F1| f(x);

        eq(
            F::apply(F::pure(f1), F::pure(a)),
            F::pure(ap(a)).pipe(|fa| F::apply(fa, F::pure(f1))),
        )
    }

    inner::<F, _>(a, f1, eq)
}

#[must_use]
pub fn identity_law<'a, F: Applicative<'a>>(
    a: i32,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    homomorphism_law::<F>(a, identity, eq)
}

#[must_use]
pub fn composition_law<'a, F: Applicative<'a>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    #[inline]
    pub fn inner<'a, F: Applicative<'a>, F1, F2>(
        a: i32,
        f1: F1,
        f2: F2,
        eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
    ) -> bool
    where
        F1: 'a + Fn(i32) -> i32 + Copy,
        F2: 'a + Fn(i32) -> i32 + Copy,
    {
        let compose = move |f2: F2| move |f1: F1| move |x| f2(f1(x));

        eq(
            F::apply(
                F::apply(F::map(compose, F::pure(f2)), F::pure(f1)),
                F::pure(a),
            ),
            F::apply(F::pure(f2), F::apply(F::pure(f1), F::pure(a))),
        )
    }

    inner::<F, _, _>(a, f1, f2, eq)
}
