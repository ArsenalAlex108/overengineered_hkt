use core::{convert::Infallible, marker::PhantomData};

use tap::Pipe as _;

use crate::{
    hkt::{
        CloneK, CovariantK, FoldWhile, Foldable, Functor, Hkt, HktUnsized, Rfoldable, TCloneableOf5, UnsizedHkt, UnsizedHktUnsized, one_of::{NotT5Of5, OneOf5Hkt, T5Of5Hkt}
    },
    marker_classification::{AssertBlankOutput, ConstBool, TypeGuard},
    utils::CloneWrapper,
};

/// [BindT] is required to implement [Functor], [DependentCloneK], [DependentExpandLifetimeK], [CovariantK] so try implementing these for `TOuter::F<()>` first... It is not possible for [BindT] to implement [Applicative][Applicative], [Monad][Monad]...
///
/// [Applicative]: crate::hkt::Applicative
/// [Monad]: crate::hkt::Monad
pub struct BindT<TOuter, TInner>(Infallible, PhantomData<(TOuter, TInner)>);

// impl<TOuter: UnsizedHktUnsized, TInner: UnsizedHkt> UnsizedHkt for BindT<TOuter, TInner> {
//     type UnsizedF<'a, A: 'a> = TOuter::UnsizedFUnsized<'a, TInner::UnsizedF<'t, A>>
//     where
//         't: 'a;
// }

// Conflict with above - there are 16 possible combinations
// impls prioritizing Sized inputs bounds
impl<'t, TOuter: UnsizedHkt<'t>, TInner: Hkt<'t>> UnsizedHkt<'t> for BindT<TOuter, TInner> {
    type UnsizedF<'a, A: 'a>
        = TOuter::UnsizedF<'a, TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: Hkt<'t>, TInner: Hkt<'t>> Hkt<'t> for BindT<TOuter, TInner> {
    type F<'a, A: 'a>
        = TOuter::F<'a, TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: UnsizedHkt<'t>, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t>
    for BindT<TOuter, TInner>
{
    type UnsizedFUnsized<'a, A: ?Sized + 'a>
        = TOuter::UnsizedF<'a, TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: Hkt<'t>, TInner: HktUnsized<'t>> HktUnsized<'t> for BindT<TOuter, TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = TOuter::F<'a, TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

// pub struct BindUnsizedT<TOuter = IdHkt, TInner = IdHkt>(Infallible, PhantomData<(TOuter, TInner)>);

// impl<'t, TOuter: UnsizedHkt<'t>, TInner: Hkt<'t>> UnsizedHkt<'t> for BindUnsizedT<TOuter, TInner> {
//     type UnsizedF<'a, A: 'a>
//         = TOuter::UnsizedF<'a, TInner::F<'a, A>>
//     where
//         't: 'a;
// }

// impl<'t, TOuter: UnsizedHktUnsized<'t>, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t> for BindUnsizedT<TOuter, TInner> {
//     type UnsizedFUnsized<'a, A: ?Sized + 'a> = TOuter::UnsizedFUnsized<'a, TInner::FUnsized<'a, A>>
//     where
//         't: 'a;
// }

// impl<'t, TOuter: HktUnsized<'t>, TInner: HktUnsized<'t>> HktUnsized<'t> for BindUnsizedT<TOuter, TInner> {
//     type FUnsized<'a, A: ?Sized + 'a> = TOuter::FUnsized<'a, TInner::FUnsized<'a, A>>
//     where
//         't: 'a;
// }

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t> + NotT5Of5<'t>,
    TOuter: Functor<'t, ReqIn, ReqOut, ReqF1>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqIn> + CloneK<'t, ReqOut>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for BindT<TOuter, TInner>
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> <ReqOut>::Output<'a, B> + Clone,
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> B>);

        let f = CloneWrapper(f, |f: &_| ReqF1::clone_one_of_5(f));

        TOuter::map(
            {
                let clone_a2 = clone_a.clone();
                move |a| {
                    <TInner as CloneK<'t, ReqIn>>::clone(clone_a2.clone(), a)
                        .pipe(ReqIn::into_guarded)
                }
            },
            {
                let clone_b2 = clone_b.clone();
                move |b| {
                    <TInner as CloneK<'t, ReqOut>>::clone(clone_b2.clone(), b)
                        .pipe(ReqOut::into_guarded)
                }
            },
            ReqF1::create_from(&f.clone().0, move |ka| {
                TInner::map(clone_a.clone(), clone_b.clone(), f.clone().0, ka)
            })
            .pipe(|f| {
                ReqF1::arbitrary_t5(f, PhantomData::<fn(TInner::F<'a, A>) -> TInner::F<'a, B>>)
            }),
            fa,
        )
    }
}

// impl<'t, TOuter: DependentCloneK<'t>, TInner: DependentCloneK<'t>> DependentCloneK<'t> for BindT<TOuter, TInner> {
//     fn clone_f<'a, 'b, F: DependentCloneK<'t>, A: 'a + Clone>(a: &Self::F<'a, F::F<'a, A>>) -> Self::F<'b, F::F<'b, A>> where 't: 'a {
//         TOuter::clone_f::<BindT<TInner, F>, _>(a)
//     }
// }

impl<
    't,
    TOuter: CovariantK<'t> + Functor<'t, ConstBool<false>, ConstBool<false>, T5Of5Hkt>,
    TInner: CovariantK<'t>,
> CovariantK<'t> for BindT<TOuter, TInner>
{
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        //  TODO: about this syntax
        TOuter::map::<
            _,
            _,
            fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
            fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
            fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
            fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
            _,
        >(
            |_| AssertBlankOutput,
            |_| AssertBlankOutput,
            |a| TInner::covariant_convert::<'a, 'b>(a),
            TOuter::covariant_convert::<'a, 'b>(a),
        )
    }
}

// Can;t figure out how to implement
// impl<
//     't,
//     ReqIn: TypeGuard<'t>,
//     TOuter: CloneK<'t, ReqIn> + CloneOwnedK<'t, ReqIn> + Functor<'t, ConstBool<false>, ConstBool<false>, T4Of5Hkt>,
//     TInner: CloneOwnedK<'t, ReqIn> + CloneK<'t, ReqIn>,
// > CloneOwnedK<'t, ReqIn> for BindT<TOuter, TInner>
// {
//     fn clone_owned<'a, 'b, A>(
//         clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone,
//         a: &Self::F<'a, A>,
//     ) -> Self::F<'b, A>
//     where
//         A: 'a + 'b,
//         't: 'a + 'b,
//     {
//         let clone_a2 = clone_a.clone();
//         let clone_a3 = clone_a2.clone();
//         let r = TOuter::clone_owned(
//             move |a| {
//                 let r = TInner::clone_owned(clone_a3.clone(), a).pipe(ReqIn::into_guarded);
//                 r
//             },
//             a,
//         );
        
//         let r = TOuter::map::<
//             _,
//             _,
//             fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
//             fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
//             fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
//             _,
//             fn(TInner::F<'a, A>) -> TInner::F<'b, A>,
//         >(
//             |_| AssertBlankOutput,
//             |_| AssertBlankOutput,
//             move |a| {let r = TInner::clone_owned(clone_a.clone(), &a); r},
//             r,
//         );

//         TOuter::clone_owned(
//             move |a| {
//                 let r = TInner::clone_owned(clone_a2.clone(), a).pipe(ReqIn::into_guarded);
//                 r
//             },
//             &r,
//         )
//     }
// }

impl<'t, ReqIn: TypeGuard<'t>, TOuter: CloneK<'t, ReqIn>, TInner: CloneK<'t, ReqIn>>
    CloneK<'t, ReqIn> for BindT<TOuter, TInner>
{
    fn clone<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TOuter::clone(
            move |a| TInner::clone(clone_a.clone(), a).pipe(ReqIn::into_guarded),
            a,
        )
    }
}

// Limited due to input clone func lifetime limit
impl<
    't,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t> + NotT5Of5<'t>,
    InHkt: Hkt<'t>,
    TOuter: Foldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt>,
    TInner: Foldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt>,
> Foldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt> for BindT<TOuter, TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _: impl 'f + Fn(&A) -> AssertBlankOutput + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: InHkt::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, InHkt::F<'a, A>) -> FoldWhile<B>>);

        TOuter::fold_while(
            |_| AssertBlankOutput,
            clone_b.clone(),
            ReqF1::create_from(&f, |b, ka|
            TInner::fold_while(
                |_| AssertBlankOutput,
                clone_b.clone(),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            ))
            .pipe(|f| ReqF1::arbitrary_t5(f, PhantomData::<fn(B, InHkt::F<'a, TInner::F<'a, A>>) -> FoldWhile<B>>)),
            init,
            fa
        )
    }
}

// Limited due to input clone func lifetime limit
impl<
    't,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t> + NotT5Of5<'t>,
    InHkt: Hkt<'t>,
    TOuter: Rfoldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt>,
    TInner: Rfoldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt>,
> Rfoldable<'t, ConstBool<false>, ReqOut, ReqF1, InHkt> for BindT<TOuter, TInner>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _: impl 'f + Fn(&A) -> AssertBlankOutput + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: InHkt::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, InHkt::F<'a, A>) -> FoldWhile<B>>);

        TOuter::rfold_while(
            |_| AssertBlankOutput,
            clone_b.clone(),
            ReqF1::create_from(&f, |b, ka|
            TInner::rfold_while(
                |_| AssertBlankOutput,
                clone_b.clone(),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            ))
            .pipe(|f| ReqF1::arbitrary_t5(f, PhantomData::<fn(B, InHkt::F<'a, TInner::F<'a, A>>) -> FoldWhile<B>>)),
            init,
            fa
        )
    }
}
