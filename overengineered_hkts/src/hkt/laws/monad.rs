use tap::Pipe as _;

use crate::{hkt::{CloneK, Monad, one_of::T4Of5Hkt}, marker_classification::DependentClone};

#[must_use]
pub fn left_identity_law<'a, 't: 'a, F: Monad<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, F::pure(|i| *i, a), |a| F::pure(|i| *i, a)), F::pure(|i| *i, a))
}

#[must_use]
pub(crate) fn left_identity_law_k<'a, 't: 'a, F: Monad<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>>(
    a: F::F<'a, i32>,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, F::clone(|i| *i, &a), |a| F::pure(|i| *i, a)), a)
}

/// TODO: Is this impl of law correct?
#[must_use]
pub fn right_identity_law<'a, 't: 'a, F: Monad<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(F::bind::<
            _,
            _,
            fn(fn(i32) -> _) -> _,
            fn(fn(i32) -> _) -> _,
            fn(fn(i32) -> _) -> _,
            _,
            fn(fn(i32) -> _) -> _,
        >(|i| *i, |i| *i, F::pure(|i| *i, |a| F::pure(|i| *i, a)), move |f| f(a)), F::pure(|i| *i, a))
}

#[must_use]
pub fn associativity_law<'a, 't: 'a, F: Monad<'t, DependentClone, DependentClone, T4Of5Hkt>>(
    a: i32,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    let f1 = move |a| F::pure(|i| *i,f1(a));
    let f2 = move |a| F::pure(|i| *i, f2(a));

    eq(
        F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, F::pure(|i| *i, a), f1).pipe(|a| F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, a, f2)),
        F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, F::pure(|i| *i, a), move |a| F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, f1(a), f2)),
    )
}


#[must_use]
pub fn associativity_law_k<'a, 't: 'a, F: Monad<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>>(
    a: F::F<'a, i32>,
    f1: impl 'a + Fn(i32) -> i32 + Copy,
    f2: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    let f1 = move |a| F::pure(|i| *i,f1(a));
    let f2 = move |a| F::pure(|i| *i, f2(a));

    eq(
        F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, F::clone(|i| *i, &a), f1).pipe(|a| F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, a, f2)),
        F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, a, move |a| F::bind::<
            _,
            _,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            fn(i32) -> F::F<'a, i32>,
            _,
            fn(i32) -> F::F<'a, i32>,
        >(|i| *i, |i| *i, f1(a), f2)),
    )
}
