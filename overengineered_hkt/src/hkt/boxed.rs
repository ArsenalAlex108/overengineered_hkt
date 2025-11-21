use std::convert::identity;
use std::ops::Deref as _;
use std::{convert::Infallible, marker::PhantomData};

use tap::Pipe as _;

use crate::hkt::bind::BindT;
use crate::hkt::hkt_classification::{self, HktClassification};
use crate::hkt::nullary::NullaryHkt;
use crate::hkt::one_of::OneOf5Hkt;
use crate::hkt::reference::RefT;
use crate::hkt::{
    Applicative, CloneFnHkt, CloneK, CloneOwnedK, CovariantK, FoldWhile, Foldable, Functor, Monad,
    Pure, PureMapInner, Rfoldable, Traversable, UnsizedHkt, UnsizedHktUnsized,
};
use crate::{
    TyEq,
    hkt::{Hkt, HktUnsized, id::IdHkt},
};

use super::Marker;

pub struct BoxT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for BoxT<TInner> {
    type F<'a, A: 'a>
        = Box<TInner::UnsizedF<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for BoxT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Box<TInner::UnsizedFUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for BoxT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

// impl<'a, TInner: Functor<'a>> Functor<'a> for BoxT<TInner> {
//     fn map<A, B, F: 'a + FnMut(A) -> B + Clone>(f: F, fa: Self::F<A>) -> Self::F<B> {
//         let fa: Box<TInner::F<A>> = fa;
//         let r = TInner::map(*fa, f.clone()).pipe(Box::new);

//         unsafe {
//             transmute(r)
//         }
//     }
// }

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for BoxT<TInner>
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
        fa.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>)
            .pipe(|a| TInner::map(in_requirement, out_requirement, f, *a))
            .pipe(Box::new)
            .pipe(TInner::F::transmute_hkt_into::<BoxT<IdHkt>>)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fb: Self::F<'a, A>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, A) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, A) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, A) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, A) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, A) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        TInner::fold_while(
            in_requirement,
            out_requirement,
            f,
            init,
            *fb.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>),
        )
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<TInner>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fb: Self::F<'a, A>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, A) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, A) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, A) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, A) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, A) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        TInner::rfold_while(
            in_requirement,
            out_requirement,
            f,
            init,
            *fb.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>),
        )
    }
}

impl<'t, ReqIn: Hkt<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn> for BoxT<TInner> {
    fn pure<'a, A>(in_requirement: ReqIn::F<'a, A>, a: A) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        Box::new(TInner::pure(in_requirement, a)).pipe(TInner::F::transmute_hkt_into::<BoxT<IdHkt>>)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1>,
> Applicative<'t, ReqIn, ReqOut, ReqF1> for BoxT<TInner>
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
        let ff = *ff.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>);
        let fa = *fa.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>);

        TInner::apply(in_requirement, out_requirement, ff, fa)
            .pipe(Box::new)
            .pipe(TInner::F::transmute_hkt_into::<BoxT<IdHkt>>)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>,
    ReqOut: Hkt<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Monad<'t, ReqIn, ReqOut, ReqF1>,
> Monad<'t, ReqIn, ReqOut, ReqF1> for BoxT<TInner>
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
        let from_unsized = TInner::F::transmute_hkt_from::<BoxT<IdHkt>>;

        let f = map_one_of_5_with!(ReqF1, f, |mut f| move |a| *f(a).pipe(from_unsized));

        let fa = *fa.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>);

        TInner::bind(in_requirement, out_requirement, fa, f)
            .pipe(Box::new)
            .pipe(TInner::F::transmute_hkt_into::<BoxT<IdHkt>>)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>
        + CloneK<'t>
        + PureMapInner<'t, NullaryHkt, BoxT>
        + PureMapInner<'t, NullaryHkt, TInner>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Traversable<'t, ReqIn, ReqIn, ReqF1>,
> Traversable<'t, ReqIn, ReqIn, ReqF1> for BoxT<TInner>
{
    fn traverse<'a, A, B, F, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqIn::F<'a, B>,
        f: ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        fa: Self::F<'a, A>,
    ) -> F::F<'a, Self::F<'a, B>>
    where
        A: 'a,
        B: 'a,
        F1Once: 'a + FnOnce(A) -> F::F<'a, B>,
        F1Mut: 'a + FnMut(A) -> F::F<'a, B>,
        F1Fn: 'a + Fn(A) -> F::F<'a, B>,
        F1Clone: 'a + Fn(A) -> F::F<'a, B> + Clone,
        F1Copy: 'a + Fn(A) -> F::F<'a, B> + Copy,
        F: Applicative<'t, ReqIn, ReqIn, ReqF1>,
        't: 'a,
    {
        let get_in_req = move || ReqIn::clone((), &in_requirement);
        let get_out_req = move || ReqIn::clone((), &out_requirement);

        let f_tag = ReqF1::create_from(&f, |a: TInner::F<'a, B>| Box::new(a));

        let result: F::F<'a, TInner::F<'a, B>> = TInner::traverse::<_, _, F, _, _, _, _, _>(
            get_in_req(),
            get_out_req(),
            f,
            *fa.pipe(TInner::F::transmute_hkt_from::<BoxT<IdHkt>>),
        );

        let req_inner: ReqIn::F<'a, TInner::F<'a, B>> =
            <ReqIn as PureMapInner<_, TInner>>::pure_map_inner((), get_out_req());

        let req_box_inner: ReqIn::F<'a, Box<TInner::F<'a, B>>> =
            <ReqIn as PureMapInner<_, BoxT>>::pure_map_inner((), req_inner);

        let req_inner: ReqIn::F<'a, TInner::F<'a, B>> =
            <ReqIn as PureMapInner<_, TInner>>::pure_map_inner((), get_out_req());

        F::map(req_inner, req_box_inner, f_tag, result)
            .pipe(TInner::F::transmute_hkt_into::<BindT<F, BoxT>>)
    }
}

impl<'t, ReqIn: Hkt<'t>, TInner: CloneOwnedK<'t, ReqIn>> CloneOwnedK<'t, ReqIn> for BoxT<TInner> {
    fn clone_owned<'a, 'b, A>(in_requirement: ReqIn::F<'a, A>, a: &Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b,
    {
        TInner::clone_owned(
            in_requirement,
            a.deref().pipe(TInner::F::transmute_hkt_from::<RefT>),
        )
        .pipe(Box::new)
        .pipe(TInner::F::transmute_hkt_into::<BoxT>)
    }
}

impl<'t, ReqIn: Hkt<'t>, TInner: CloneK<'t, ReqIn>> CloneK<'t, ReqIn> for BoxT<TInner> {
    fn clone<'a, A>(in_requirement: ReqIn::F<'a, A>, a: &Self::F<'a, A>) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TInner::clone(
            in_requirement,
            a.deref().pipe(TInner::F::transmute_hkt_from::<RefT>),
        )
        .pipe(Box::new)
        .pipe(TInner::F::transmute_hkt_into::<BoxT>)
    }
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for BoxT<TInner> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        TInner::covariant_convert(*a.pipe(TInner::F::transmute_hkt_from::<BoxT>))
            .pipe(Box::new)
            .pipe(TInner::F::transmute_hkt_into::<BoxT>)
    }
}
