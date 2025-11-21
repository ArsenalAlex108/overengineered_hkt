use composing::compose_fn;
use naan::fun::{F1, F1Once, F2Once};

use crate::hkt::{DependentCloneK, Functor};

#[must_use]
pub fn identity_law<'a, F: Functor<'a> + DependentCloneK<'a>>(
    fa: F::F<i32>,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    eq(F::clone(&fa), fa)
}

#[must_use]
pub fn composite_law<'a, F: Functor<'a> + DependentCloneK<'a>>(
    fa: F::F<i32>,
    f: impl 'a + Fn(i32) -> i32 + Copy,
    g: impl 'a + Fn(i32) -> i32 + Copy,
    eq: impl Fn(F::F<i32>, F::F<i32>) -> bool,
) -> bool {
    eq(
        F::map(move |a| g(f(a)), F::clone(&fa)),
        F::map(g, F::map(f, F::clone(&fa))),
    )
}
