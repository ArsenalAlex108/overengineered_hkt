
use crate::{
    hkt::{CloneK, Functor, one_of::T4Of5Hkt},
    marker_classification::DependentClone,
};

#[must_use]
pub fn identity_law<
    'a,
    't: 'a,
    F: Functor<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>,
>(
    fa: F::F<'a, i32>,
    eq: impl 'a + Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(F::clone(|i| *i, &fa), fa)
}

#[must_use]
pub fn composite_law<
    'a,
    't: 'a,
    F: Functor<'t, DependentClone, DependentClone, T4Of5Hkt> + CloneK<'t, DependentClone>,
>(
    fa: F::F<'a, i32>,
    f: impl 'a + Fn(i32) -> i32 + Copy,
    g: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<'a, i32>, F::F<'a, i32>) -> bool,
) -> bool {
    eq(
        F::map::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
            |i| *i,
            |i| *i,
            move |a| g(f(a)),
            F::clone(|i| *i, &fa),
        ),
        F::map::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
            |i| *i,
            |i| *i,
            g,
            F::map::<_, _, fn(i32) -> i32, fn(i32) -> i32, fn(i32) -> i32, _, fn(i32) -> i32>(
                |i| *i,
                |i| *i,
                f,
                F::clone(|i| *i, &fa),
            ),
        ),
    )
}
