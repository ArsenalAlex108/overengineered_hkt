use std::{convert::Infallible, marker::PhantomData};

use tap::Pipe;

use crate::
    hkt::{
        Applicative, CloneApplicativeFn, CloneK, CovariantK, FoldWhile, Foldable, Functor, Hkt, HktUnsized, Monad, Pure, PureMapInner, Rfoldable, TCloneableOf5, Traversable, UnsizedHkt, UnsizedHktUnsized, boxed::BoxT, id::IdHkt, nullary::NullaryHkt, reference::RefMutT, vec::VecT
    }
;

use super::one_of::OneOf5Hkt;

/// Note: a dyn iterator cannot implement [Clone]
pub struct DynIteratorT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: Hkt<'t>> UnsizedHkt<'t> for DynIteratorT<TInner> {
    type UnsizedF<'a, A: 'a>
        = dyn 'a + Iterator<Item = TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t> for DynIteratorT<TInner> {
    type UnsizedFUnsized<'a, A: 'a + ?Sized>
        = dyn 'a + Iterator<Item = TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t>,
    ReqOut: Hkt<'t> + CloneK<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> B>);

        fa.map(move |x| {
            TInner::map(
                ReqIn::clone((), &in_requirement),
                ReqOut::clone((), &out_requirement),
                ReqF1::clone_one_of_5(&f),
                x,
            )
        })
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t>,
    ReqOut: Hkt<'t> + CloneK<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        mut fb: Self::F<'a, A>,
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, A) -> FoldWhile<B>>);

        fb.try_fold(init, move |b, ka| {
            TInner::fold_while(
                ReqIn::clone((), &in_requirement),
                ReqOut::clone((), &out_requirement),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            )
        })
    }
}

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t>,
    ReqOut: Hkt<'t> + CloneK<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        mut fb: Self::F<'a, A>,
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, A) -> FoldWhile<B>>);

        fb.try_fold(init, move |b, ka| {
            TInner::rfold_while(
                ReqIn::clone((), &in_requirement),
                ReqOut::clone((), &out_requirement),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            )
        })
    }
}

impl<'t, ReqIn: Hkt<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn> for BoxT<DynIteratorT<TInner>> {
    fn pure<'a, A>(in_requirement: <ReqIn>::F<'a, A>, a: A) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TInner::pure(in_requirement, a)
            .pipe(std::iter::once)
            .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t> + PureMapInner<'t, NullaryHkt, TInner>,
    ReqOut: Hkt<'t> + CloneK<'t>,
    // Unused clone bound
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqIn>
    + CloneApplicativeFn<'t, ReqF1>
    //+ CovariantK<'t>
    ,
> Applicative<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
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
        let fa = fa.collect::<Vec<_>>();

        let clone_fa = move || {
            fa.iter()
                .map(|ta| TInner::clone(ReqIn::clone((), &in_requirement), ta))
        };

        ff.flat_map(|f| {
            clone_fa().map(move |a| {
                TInner::apply(
                    ReqIn::clone((), &in_requirement),
                    ReqOut::clone((), &out_requirement),
                    TInner::clone_applicative_func(&f),
                    a,
                )
                //.pipe(TInner::covariant_convert)
            })
        })
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: Hkt<'t>
        + CloneK<'t>
        + PureMapInner<'t, NullaryHkt, BoxT<DynIteratorT<TInner>>>
        + PureMapInner<'t, NullaryHkt, RefMutT<VecT<TInner>>>
        + PureMapInner<'t, NullaryHkt, TInner>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Monad<'t, ReqIn, ReqIn, ReqF1> + Foldable<'t, ReqIn, ReqIn, ReqF1> 
    // Unused bound from Applicative impl
    + CloneK<'t, ReqIn> + CloneApplicativeFn<'t, ReqF1>,
> Monad<'t, ReqIn, ReqIn, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqIn::F<'a, B>,
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> Self::F<'a, B>>);

        fa.flat_map(|a| {
            let nested: <TInner as Hkt<'t>>::F<'a, Box<dyn Iterator<Item = <TInner as Hkt<'t>>::F<'a, B>> + 'a>> = TInner::map(
                ReqIn::clone((), &in_requirement),
                ReqIn::clone((), &out_requirement).pipe(|r| <ReqIn as PureMapInner<'t, NullaryHkt, BoxT<DynIteratorT<TInner>>> >::pure_map_inner((), r)),
                ReqF1::clone_one_of_5(&f),
                a,
            );

            // Copied from Vec::extend_desugared source code
            let mut sum = Vec::with_capacity(TInner::size_hint(&nested).0.saturating_add(1));

            TInner::extend(
                ReqIn::clone((), &out_requirement).pipe(|r| ReqIn::pure_map_inner((), r)),
                ReqIn::clone((), &out_requirement).pipe(|r| <ReqIn as PureMapInner<'t, NullaryHkt, BoxT<DynIteratorT<TInner>>>>::pure_map_inner((), r))
                .pipe(|r| <ReqIn as PureMapInner<'t, NullaryHkt, RefMutT<VecT<TInner>>>>::pure_map_inner((), r)),
                ReqF1::create_from(&f, ()),
                &mut sum,
                nested,
            );

            // Don't flatten - flat_map reduce instead
            sum.into_iter()
            .flat_map(|iter|
                iter.reduce(|sum, b|  TInner::bind(
                    ReqIn::clone((), &out_requirement),
                    ReqIn::clone((), &out_requirement),
                    sum,
                    // This is where b needs cloning
                    ReqF1::create_from(&f, |_| b)
                ))
            )
        })
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t>,
    ReqOut: Hkt<'t>
        + CloneK<'t>
        + PureMapInner<'t, NullaryHkt, BoxT<DynIteratorT<TInner>>>
        + PureMapInner<'t, NullaryHkt, TInner>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Traversable<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqOut>,
> Traversable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn traverse<'a, A, B, F, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
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
        F: Applicative<'t, ReqIn, ReqOut, ReqF1>,
        't: 'a,
    {
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> F::F<'a, B>>);

        let len = Self::size_hint(&fa).0.saturating_add(1);
        fa.fold(
            F::pure(ReqIn::clone((), &in_requirement), Vec::with_capacity(len)),
            |sum, a| {
                let ta: F::F<'a, TInner::F<'a, B>> = TInner::traverse(
                    ReqIn::clone((), &in_requirement),
                    ReqOut::clone((), &out_requirement),
                    ReqF1::clone_one_of_5(&f),
                    a
                );

                let func = F::map(
                    ReqIn::clone((), &in_requirement)
                    .pipe(|r| R),
                    todo!(),
                    move |iter: Box<dyn 'a + Iterator<Item = TInner::F<'a, B>>>| {
                        let vec = iter.collect::<Vec<_>>();
                        move |tb| {
                            vec.iter()
                            .map(|b| TInner::clone(
                                ReqOut::clone((), &out_requirement),
                                b
                            ))
                            .chain([tb])
                            .pipe(Box::new)
                            //.collect::<Vec<_>>()
                        }
                    },
                    sum
                );

                F::apply(
                    todo!(),
                    todo!(),
                    func,
                    ta
                )
            }
        )
    }
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t>
    for BoxT<DynIteratorT<TInner>>
{
    fn covariant_convert<'a, 'b, A>(
        a: Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a.map(TInner::covariant_convert)
        .pipe(Box::new)
    }
}
