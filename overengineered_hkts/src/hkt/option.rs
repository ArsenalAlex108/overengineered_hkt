use core::{
    convert::{Infallible, identity},
    marker::PhantomData,
    mem::transmute,
};

use tap::Pipe as _;

use crate::{
    hkt::{
        Applicative, CloneK, CloneOwnedK, Converge, CovariantK, FoldWhile, Foldable, Functor, Hkt,
        HktUnsized, Monad, PhantomMarker, Pure, Rfoldable, Traversable,
        hkt_classification::{self, HktClassification},
        id::IdHkt,
        one_of::{NotT5Of5, OneOf5Hkt},
        reference::{RefMutT, RefT},
    },
    marker_classification::{ConstBool, TypeGuard},
    transmute::unsafe_transmute_id,
    utils::CloneWrapper,
};

pub struct OptionT<TInner = IdHkt>(Infallible, PhantomMarker<TInner>);

impl<'t, TInner: Hkt<'t>> Hkt<'t> for OptionT<TInner> {
    type F<'a, A: 'a>
        = Option<TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> HktUnsized<'t> for OptionT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Option<TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for OptionT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for OptionT<TInner>
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
        fa.map(move |x| TInner::map(clone_a.clone(), clone_b.clone(), f, x))
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for OptionT<TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
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
        match fb {
            Some(t) => TInner::fold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1, RefT>,
> Foldable<'t, ReqIn, ReqOut, ReqF1, RefT> for OptionT<TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        match fa {
            Some(t) => TInner::fold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1, RefMutT>,
> Foldable<'t, ReqIn, ReqOut, ReqF1, RefMutT> for OptionT<TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefMutT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        match fa {
            Some(t) => TInner::fold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1> for OptionT<TInner>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
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
        match fb {
            Some(t) => TInner::rfold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefT>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefT> for OptionT<TInner>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        match fa {
            Some(t) => TInner::rfold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefMutT>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefMutT> for OptionT<TInner>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefMutT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        match fa {
            Some(t) => TInner::rfold_while(clone_a.clone(), clone_b.clone(), f, init, t),
            None => FoldWhile::Break(init),
        }
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn> for OptionT<TInner> {
    fn pure<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: A,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TInner::pure(clone_a, a).pipe(Some)
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1>,
> Applicative<'t, ReqIn, ReqOut, ReqF1> for OptionT<TInner>
{
    fn apply<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
        match (ff, fa) {
            (Some(f), Some(a)) => Some(TInner::apply(clone_a, clone_b, f, a)),
            (_, _) => None,
        }
    }
}

// TODO: Test
impl<
    't,
    // ?: Consider if cloning funcs should be [Copy]
    // Final: No - CloneWrappers cannot implement Copy
    ReqF1: OneOf5Hkt<'t> + NotT5Of5<'t>,
    TInner: Monad<'t, ConstBool<true>, ConstBool<true>, ReqF1>
        + Traversable<'t, ConstBool<true>, ConstBool<true>, ReqF1>
        + CloneK<'t, ConstBool<true>>
        + Functor<'t, ConstBool<true>, ConstBool<false>, ReqF1>,
> Monad<'t, ConstBool<true>, ConstBool<true>, ReqF1> for OptionT<TInner>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> A + Clone,
        clone_b: impl 'a + Fn(&B) -> B + Clone,
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
        match fa {
            Some(ta) => {
                let f_tag = ReqF1::create_from(&f, ());

                let traversed = TInner::traverse::<_, _, OptionT, _, _, _, _, _>(
                    clone_a.clone(),
                    {
                        let clone_b = clone_b.clone();
                        move |tb| TInner::clone(clone_b.clone(), tb)
                    },
                    f,
                    ta,
                );

                let f_tag = CloneWrapper(f_tag, |f: &ReqF1::OneOf5F<'a, (), (), (), (), ()>| {
                    ReqF1::clone_one_of_5(f)
                });

                traversed.map({
                    move |ttb| {
                        TInner::bind(
                            {
                                let clone_b = clone_b.clone();
                                move |tb| TInner::clone(clone_b.clone(), tb)
                            },
                            clone_b.clone(),
                            ttb,
                            ReqF1::create_from(&f_tag.0, identity),
                        )
                    }
                })
                // let f_tag = ReqF1::create_from(&f, ());

                // let nested = TInner::map(
                //     clone_a.clone(),
                //     {
                //         let clone_b = clone_b.clone();
                //         move |res| {
                //             match res {
                //                 Some(tb) => Some(TInner::clone(clone_b.clone(), tb)),
                //                 None => None,
                //             }
                //             .pipe(ReqIn::into_guarded)
                //         }
                //     },
                //     f,
                //     ta,
                // );

                // let nested_ref: &TInner::F<'_, Option<TInner::F<'_, B>>> = unsafe { unsafe_transmute_id(&nested) };

                // let has_none = <TInner as Foldable<ReqIn, ReqIn, ReqF1, RefT>>::fold_while::<'_, '_, '_>(
                //     |b| {
                //         todo!()
                //     },
                //     |b| ReqIn::into_guarded(()),
                //     ReqF1::create_from(&f_tag, |(), b: &_| match b {
                //         Some(_) => FoldWhile::Continue(()),
                //         None => FoldWhile::Break(()),
                //     }),
                //     (),
                //     nested_ref,
                // )
                // .is_break();

                // match has_none {
                //     true => None,
                //     false => Some(TInner::bind(
                //         {
                //             let clone_b = clone_b.clone();
                //             move |b: &Option<<TInner as Hkt<'t>>::F<'a, B>>| {
                //                 b.as_ref()
                //                     .map(|fb| TInner::clone(clone_b.clone(), fb))
                //                     .pipe(ReqIn::into_guarded)
                //             }
                //         },
                //         clone_b,
                //         nested,
                //         ReqF1::create_from(&f_tag, |o: Option<TInner::F<'a, B>>| {
                //             o.expect("Checked for no Nones.")
                //         }),
                //     )),
                // }

                // <TInner as Foldable<ReqIn, ReqIn, ReqF1, IdHkt>>::fold_while(
                //     |b| {
                //         b.as_ref()
                //             .map(|fb| TInner::clone(clone_b.clone(), fb))
                //             .pipe(ReqIn::into_guarded)
                //     },
                //     |b| {
                //         b.as_ref()
                //             .map(|fb| TInner::clone(clone_b.clone(), fb))
                //             .pipe(ReqIn::into_guarded)
                //     },
                //     ReqF1::create_from(&f_tag, |sum: Option<<TInner as Hkt<'t>>::F<'a, B>>, a| {
                //         match (sum, a) {
                //             (Some(a), Some(b)) => Some({
                //                 // let clone_b2 = clone_b.clone();
                //                 // let b = CloneWrapper(b, move |b: &TInner::F<'a, B>| {
                //                 //     TInner::clone(clone_b2.clone(), b)
                //                 // });

                //                 // TODO: The bug - to combine Semigroup must be used instead
                //                 // TInner::bind(
                //                 //     clone_b.clone(),
                //                 //     clone_b.clone(),
                //                 //     a,
                //                 //     ReqF1::create_from(&f_tag, move |_| b.clone().0).pipe(|f| {
                //                 //         ReqF1::arbitrary_t5(
                //                 //             f,
                //                 //             PhantomData::<fn(B) -> TInner::F<'a, B>>,
                //                 //         )
                //                 //     }),
                //                 // )
                //                 TInner::combine(
                //                     a,
                //                     b,
                //                     clone_b.clone()
                //                 )
                //             }),
                //             // Behaves like a.extend(b)
                //             (a, b) => a.or(b)
                //         }
                //         .pipe(FoldWhile::Continue)
                //     })
                //     .pipe(|f| {
                //         ReqF1::arbitrary_t5(
                //             f,
                //             PhantomData::<
                //                 fn(
                //                     Option<TInner::F<'a, B>>,
                //                     Option<TInner::F<'a, B>>,
                //                 )
                //                     -> FoldWhile<Option<TInner::F<'a, B>>>,
                //             >,
                //         )
                //     }),
                //     None,
                //     nested,
                // )
                // .converge()
            }
            None => None,
        }
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Traversable<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqOut>,
> Traversable<'t, ReqIn, ReqOut, ReqF1> for OptionT<TInner>
{
    fn traverse<'a, A, B, F, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
        F: Applicative<'t, ReqIn, ReqOut, ReqF1> + Functor<'t, ReqIn, ConstBool<false>, ReqF1>,
        't: 'a,
    {
        let f_map = ReqF1::create_from(&f, Some);

        match fa {
            Some(ta) => {
                let res: F::F<'a, TInner::F<'a, B>> = TInner::traverse::<_, _, F, _, _, _, _, _>(
                    clone_a.clone(),
                    clone_b.clone(),
                    f,
                    ta,
                );

                <F as Functor<'t, ReqIn, ReqOut, ReqF1>>::map(
                    {
                        let clone_b = clone_b.clone();
                        move |a| TInner::clone(clone_b.clone(), a).pipe(ReqIn::into_guarded)
                    },
                    move |rb: &_| {
                        match rb.as_ref() {
                            Some(tb) => Some(TInner::clone(clone_b.clone(), tb)),
                            None => None,
                        }
                        .pipe(ReqOut::into_guarded)
                    },
                    f_map,
                    res,
                )
            }
            None => F::pure(
                move |res| {
                    match res.as_ref() {
                        Some(tb) => Some(TInner::clone(clone_b.clone(), tb)),
                        None => None,
                    }
                    .pipe(ReqIn::into_guarded)
                },
                None,
            ),
        }
    }
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for OptionT<TInner> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a.map(TInner::covariant_convert)
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: CloneOwnedK<'t, ReqIn>> CloneOwnedK<'t, ReqIn>
    for OptionT<TInner>
{
    fn clone_owned<'a, 'b, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b,
    {
        match a {
            Some(ta) => Some(TInner::clone_owned(clone_a, ta)),
            None => None,
        }
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: CloneK<'t, ReqIn>> CloneK<'t, ReqIn> for OptionT<TInner> {
    fn clone<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        match a {
            Some(ta) => Some(TInner::clone(clone_a, ta)),
            None => None,
        }
    }
}

#[cfg(false)]
unsafe impl<'t, TInner: CovariantTransmute<'t>> CovariantTransmute<'t> for OptionT<TInner> {
    fn covariant_transmute<'a, 'f, 'b, A, F: CovariantTransmute<'t>>(
        s: F::F<'f, Self::F<'a, A>>,
    ) -> F::F<'f, Self::F<'b, A>>
    where
        A: 'a,
        'a: 'b,
        't: 'a,
    {
        // SAFETY: https://doc.rust-lang.org/reference/subtyping.html#variance
        unsafe { unsafe_transmute_id(s) }
    }
}
