use std::{convert::Infallible, future::ready, marker::PhantomData, pin::Pin};

use futures::FutureExt;
use tap::Pipe;

use crate::hkt::{
    Applicative, CloneK, CovariantK, Foldable, Functor, Hkt, HktUnsized, Monad, Pure, UnwrapEither, hkt_classification::{self, HktClassification}, id::IdHkt, one_of::OneOf5Hkt
};

use super::one_of::OneOf5;

pub struct PinBoxFutureT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: Hkt<'t>> Hkt<'t> for PinBoxFutureT<TInner> {
    type F<'a, A: 'a>
        = Pin<Box<dyn 'a + Future<Output = TInner::F<'a, A>>>>
    where
        't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> HktUnsized<'t> for PinBoxFutureT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Pin<Box<dyn 'a + Future<Output = TInner::FUnsized<'a, A>>>>
    where
        't: 'a;
}

impl<TInner> HktClassification for PinBoxFutureT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<'t, K: CovariantK<'t>> CovariantK<'t> for PinBoxFutureT<K> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        Box::pin(a.map(|a| K::covariant_convert(a)))
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for PinBoxFutureT<TInner>
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
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
        Box::pin(fa.map(|a| TInner::map(in_requirement, out_requirement, f, a)))
    }
}

impl<'t, ReqIn: Hkt<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn> for PinBoxFutureT<TInner> {
    fn pure<'a, A: 'a>(in_requirement: ReqIn::F<'a, A>, a: A) -> Self::F<'a, A>
    where
        't: 'a,
    {
        a.pipe(|a| TInner::pure(in_requirement, a))
            .pipe(ready)
            .pipe(Box::pin)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1>,
> Applicative<'t, ReqIn, ReqOut, ReqF1> for PinBoxFutureT<TInner>
{
    fn apply<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        ff: Self::F<'a, <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>>,
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
        Box::pin(
            fa.map(async |a| {
                TInner::apply(in_requirement, out_requirement, ff.await, a)
            })
            .flatten(),
        )
    }
}

// TODO: Check if Monad Transformers VecT and PinBoxT are possible
// impl<'t, K: Foldable<'t> + Monad<'t> + DependentCloneK<'t>> Monad<'t> for PinBoxFutureT<K> {
//     fn bind<'a, A, B, F1>(fa: Self::F<'a, A>, f: F1) -> Self::F<'a, B>
//     where
//         A: 't,
//         B: 't,
//         F1: Fn(A) -> Self::F<'a, B> + Clone,
//         't: 'a {
//         fa.map(|a| K::fold_while(
//             |sum, a| {
//                 sum.push(f.clone()(a));
//                 std::ops::ControlFlow::Continue(sum)
//             },
//             Vec::new(),
//             a
//         ).pipe(async |sum| {
//             let mut org = None;

//             for fut in sum.unwrap_either() {
//                 let next = fut.await;

//                 match &mut org {
//                     Some(_) => {
//                         let old = org.take().unwrap();
//                         org = Some(K::bind(old, |_|
//                             // B must be cloneable => Prototype
//                             K::clone(&next)));
//                     },
//                     None => org = Some(next),
//                 }
//             }

//             // No value to use if foldable returns None => DefaultK
//             org.unwrap_or(a)
//         }))
//         .flatten()
//         .pipe(Box::pin)
//     }
// }

impl<'t, ReqIn: Hkt<'t>, ReqOut: Hkt<'t>, ReqF1: OneOf5Hkt<'t>> Monad<'t, ReqIn, ReqOut, ReqF1>
    for PinBoxFutureT<IdHkt>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        fa: Self::F<'a, A>,
        f: <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
    ) -> Self::F<'a, B>
    where
        A: 'a,
        B: 'a,
        F1Once: 'a + FnOnce(A) -> Self::F<'a, B>,
        F1Mut: 'a + FnMut(A) -> Self::F<'a, B>,
        F1Fn: 'a + Fn(A) -> Self::F<'a, B>,
        F1Clone: 'a + Fn(A) -> Self::F<'a, B> + Clone,
        F1Copy: 'a + Fn(A) -> Self::F<'a, B> + Copy,
        't: 'a,
    {
        match_one_of_5!(
            ReqF1::into_one_of_5_enum(f),
            f,
            fa.map(f).flatten().pipe(Box::pin)
        )
    }
}
