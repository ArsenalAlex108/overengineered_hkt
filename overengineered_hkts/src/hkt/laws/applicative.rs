use core::convert::identity;

use tap::Pipe as _;

use crate::{
    hkt::{
        Applicative, CloneK, Functor, one_of::T4Of5Hkt
    },
    marker_classification::DependentClone,
};

#[must_use]
pub fn homomorphism_law<
    'a,
    't: 'a,
    F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>,
>(
    a: i32,
    f: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl 'a + Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(
        F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
            |i| *i,
            |i| *i,
            F::pure(|i| *i, f),
            F::pure(|i| *i, a),
        ),
        F::pure(|i| *i, f(a)),
    )
}

#[must_use]
pub fn interchange_law<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl 'a + Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    #[inline]
    pub fn inner<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>, F1>(
        a: i32,
        f1: F1,
        eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
    ) -> bool
    where
        F1: 'a + Fn(i32) -> i32 + Copy,
    {
        let ap = |x| move |f: F1| f(x);

        eq(
            F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                F::pure(|i| *i, f1),
                F::pure(|i| *i, a),
            ),
            F::pure(|i| *i, ap(a)).pipe(|fa| {
                F::apply::<_, _, fn(F1) -> i32, fn(F1) -> i32, fn(F1) -> i32, _, fn(F1) -> i32>(
                    |i| *i,
                    |i| *i,
                    fa,
                    F::pure(|i| *i, f1),
                )
            }),
        )
    }

    inner::<F, _>(a, f1, eq)
}

#[must_use]
pub fn identity_law<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    eq: impl 'a + Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    homomorphism_law::<F>(a, identity, eq)
}

#[must_use]
pub fn composition_law<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    #[inline]
    pub fn inner<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt>, F1, F2>(
        a: i32,
        f1: F1,
        f2: F2,
        eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
    ) -> bool
    where
        F1: 'a + Fn(i32) -> i32 + Copy,
        F2: 'a + Fn(i32) -> i32 + Copy,
    {
        let compose = move |f2: F2| move |f1: F1| move |x| f2(f1(x));

        eq(
            F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                F::apply::<_, _, fn(F1) -> _, fn(F1) -> _, fn(F1) -> _, _, fn(F1) -> _>(
                    |i| *i,
                    |i| *i,
                    F::map::<_, _, fn(F2) -> _, fn(F2) -> _, fn(F2) -> _, _, fn(F2) -> _>(
                        |i| *i,
                        |i| *i,
                        compose,
                        F::pure(|i| *i, f2),
                    ),
                    F::pure(|i| *i, f1),
                ),
                F::pure(|i| *i, a),
            ),
            F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                F::pure(|i| *i, f2),
                F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                    |i| *i,
                    |i| *i,
                    F::pure(|i| *i, f1),
                    F::pure(|i| *i, a),
                ),
            ),
        )
    }

    inner::<F, _, _>(a, f1, f2, eq)
}

#[must_use]
pub(crate) fn composition_law_k<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>>(
    a: F::F<'a, i32>,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    #[inline]
    pub fn inner<'a, 't: 'a, F: Applicative<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>, F1, F2>(
        a: F::F<'a, i32>,
        f1: F1,
        f2: F2,
        eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
    ) -> bool
    where
        F1: 'a + Fn(i32) -> i32 + Copy,
        F2: 'a + Fn(i32) -> i32 + Copy,
    {
        let compose = move |f2: F2| move |f1: F1| move |x| f2(f1(x));

        eq(
            F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                F::apply::<_, _, fn(F1) -> _, fn(F1) -> _, fn(F1) -> _, _, fn(F1) -> _>(
                    |i| *i,
                    |i| *i,
                    F::map::<_, _, fn(F2) -> _, fn(F2) -> _, fn(F2) -> _, _, fn(F2) -> _>(
                        |i| *i,
                        |i| *i,
                        compose,
                        F::pure(|i| *i, f2),
                    ),
                    F::pure(|i| *i, f1),
                ),
                F::clone(|i| *i, &a),
            ),
            F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                F::pure(|i| *i, f2),
                F::apply::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                    |i| *i,
                    |i| *i,
                    F::pure(|i| *i, f1),
                    a,
                ),
            ),
        )
    }

    inner::<F, _, _>(a, f1, f2, eq)
}
