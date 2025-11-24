use core::convert::Infallible;

use crate::{hkt::{
    CloneK, CloneOwnedK, CovariantK, Functor, Hkt, PhantomMarker, UnsizedHkt, UnsizedHktUnsized,
    hkt_classification::{self, HktClassification},
    one_of::OneOf5Hkt,
}, marker_classification::TypeGuard};

/// A Hkt wrapper around `T` that is not actually higher kinded over anything.
pub struct NullaryHkt<T = ()>(Infallible, PhantomMarker<T>);

impl<'t, T: 't> UnsizedHkt<'t> for NullaryHkt<T> {
    type UnsizedF<'a, A: 'a>
        = T
    where
        't: 'a;
}

impl<'t, T: 't> Hkt<'t> for NullaryHkt<T> {
    type F<'a, A: 'a>
        = T
    where
        't: 'a;
}

impl<'t, T: 't> UnsizedHktUnsized<'t> for NullaryHkt<T> {
    type UnsizedFUnsized<'a, A: 'a + ?Sized>
        = T
    where
        't: 'a;
}

impl<T> HktClassification for NullaryHkt<T> {
    type Choice = hkt_classification::TransparentHkt;
}

impl<'t, T: 't, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Functor<'t, ReqIn, ReqOut, ReqF1> for NullaryHkt<T>
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        _f: <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        fa: Self::F<'a, A>,
    ) -> Self::F<'a, B>
    where
        A: 'a,
        B: 'a,
        F1Once: 'a + FnOnce(A) -> B,
        F1Mut: 'a + FnMut(A) -> B,
        F1Fn: 'a + Fn(A) -> B,
        F1Clone: 'a + Fn(A) -> B + Clone,
        F1Copy: 'a + Fn(A) -> B + Copy,
        't: 'a,
    {
        fa
    }
}

// impl<'t, T: 't, ReqIn: CloneK<'t, ReqIn> + CloneFnHkt<'t>> CloneK<'t, ReqIn> for UnitHkt<T> {
//     fn clone<'a, 'b, A: 'a + 'b>(requirements: impl 'a + Into<<ReqIn>::F<'a, A>>, a: &Self::F<'a, A>) -> Self::F<'b, A>
//     where
//         't: 'a + 'b {
//         // Expected A got T
//         ReqIn::call_clone(requirements.into(), a)
//     }
// }

impl<'t, T: 't + Clone, ReqIn: TypeGuard<'t>> CloneK<'t, ReqIn> for NullaryHkt<T> {
    fn clone<'a, A>(_clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone, a: &Self::F<'a, A>) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        a.clone()
    }
}

impl<'t, T: 't + Clone, ReqIn: TypeGuard<'t>> CloneOwnedK<'t, ReqIn> for NullaryHkt<T> {
    fn clone_owned<'a, 'b, A>(_clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone, a: &Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b,
    {
        a.clone()
    }
}

impl<'t, T: 't> CovariantK<'t> for NullaryHkt<T> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a
    }
}
