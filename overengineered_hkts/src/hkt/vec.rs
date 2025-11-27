use core::{
    convert::{Infallible, identity},
    marker::PhantomData,
};

use alloc::vec;
use alloc::vec::Vec;
use tap::Pipe;

use crate::{
    hkt::{
        Applicative, CloneK, CloneOwnedK, Converge, CovariantK, FoldWhile,
        Foldable, Functor, Hkt, HktUnsized, Monad, Pure, Rfoldable, TCloneableOf5, Traversable,
        hkt_classification::{self, HktClassification},
        id::IdHkt,
        one_of::{NotT1Of5, NotT5Of5},
        reference::{RefMutT, RefT},
    },
    marker_classification::{AssertBlankOutput, ConstBool, DependentClone, TypeGuard},
    transmute::unsafe_transmute_id,
    utils::CloneWrapper,
};

use super::one_of::OneOf5Hkt;

pub struct VecT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: Hkt<'t>> Hkt<'t> for VecT<TInner> {
    type F<'a, A: 'a>
        = Vec<TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> HktUnsized<'t> for VecT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Vec<TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for VecT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for VecT<TInner>
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> B>);

        fa.into_iter()
            .map(move |x| {
                TInner::map(
                    clone_a.clone(),
                    clone_b.clone(),
                    ReqF1::clone_one_of_5(&f),
                    x,
                )
            })
            .collect::<Vec<_>>()
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t> + NotT1Of5<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for VecT<TInner>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: Self::F<'a, A>,
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
        let mut f = ReqF1::arbitrary_t1(f, PhantomData::<fn(B, A) -> FoldWhile<B>>);

        fa.into_iter().try_fold(init, move |b, ka| {
            let f_ref = ReqF1::as_mut(&mut f)
                .pipe(|f| map_one_of_5_with!(ReqF1, f, |mut f| |b, a| f(b, a)));

            TInner::fold_while(clone_a.clone(), clone_b.clone(), f_ref, b, ka)
        })
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t> + NotT1Of5<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1, RefT>,
> Foldable<'t, ReqIn, ReqOut, ReqF1, RefT> for VecT<TInner>
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
        let mut f = ReqF1::arbitrary_t1(f, PhantomData::<fn(B, &'a A) -> FoldWhile<B>>);

        fa.iter().try_fold(init, move |b, ka| {
            let f_ref = ReqF1::as_mut(&mut f)
                .pipe(|f| map_one_of_5_with!(ReqF1, f, |mut f| |b, a| f(b, a)));

            TInner::fold_while(clone_a.clone(), clone_b.clone(), f_ref, b, ka)
        })
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t> + NotT1Of5<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1, RefMutT>,
> Foldable<'t, ReqIn, ReqOut, ReqF1, RefMutT> for VecT<TInner>
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
        let mut f = ReqF1::arbitrary_t1(f, PhantomData::<fn(B, &'a mut A) -> FoldWhile<B>>);

        fa.iter_mut().try_fold(init, move |b, ka| {
            let f_ref = ReqF1::as_mut(&mut f)
                .pipe(|f| map_one_of_5_with!(ReqF1, f, |mut f| |b, a| f(b, a)));

            TInner::fold_while(clone_a.clone(), clone_b.clone(), f_ref, b, ka)
        })
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t> + CloneK<'t>,
    ReqOut: TypeGuard<'t> + CloneK<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1> for VecT<TInner>
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, A) -> FoldWhile<B>>);

        fb.into_iter().try_fold(init, move |b, ka| {
            TInner::rfold_while(
                clone_a.clone(),
                clone_b.clone(),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            )
        })
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn> for VecT<TInner> {
    fn pure<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: A,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TInner::pure(clone_a, a).pipe(|a| vec![a])
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqIn>, //+ CovariantK<'t>
> Applicative<'t, ReqIn, ReqOut, ReqF1> for VecT<TInner>
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
        let value = clone_a.clone();
        let clone_fa = move || {
            fa.iter()
                .map(|ta| TInner::clone(value.clone(), ta))
                .collect::<Vec<_>>()
        };

        ff.into_iter()
            .flat_map(move |f| {
                clone_fa().into_iter().map({
                    let clone_a2 = clone_a.clone();
                    {
                        let clone_b2 = clone_b.clone();
                        move |a| {
                            TInner::apply(
                                clone_a2.clone(),
                                clone_b2.clone(),
                                TInner::clone(
                                    |tf| {
                                        // Yes, this is how &RefF1::OneOf5 is cloned while preserving uncloneable variants
                                        ReqF1::as_ref(tf)
                                            .pipe(|f| ReqF1::arbitrary_t1(f, PhantomData::<F1Once>))
                                            .pipe(|f| ReqF1::arbitrary_t2(f, PhantomData::<F1Mut>))
                                            .pipe(|f| ReqF1::arbitrary_t3(f, PhantomData::<F1Fn>))
                                            .pipe(|f| {
                                                ReqF1::map_one_of_5::<'_, 'a, '_>(
                                                    f,
                                                    identity,
                                                    identity,
                                                    identity,
                                                    |f| f.clone(),
                                                    |f| *f,
                                                )
                                            })
                                            .pipe(ReqIn::into_guarded)
                                    },
                                    &f,
                                ),
                                a,
                            )
                            //.pipe(TInner::covariant_convert)
                        }
                    }
                })
            })
            .collect::<Vec<_>>()
    }
}

/// TODO: test
impl<
    't,
    // TODO: Consider if cloning funcs should be [Copy]
    ReqF1: TCloneableOf5<'t> + NotT5Of5<'t>,
    TInner: Monad<'t, ConstBool<true>, ConstBool<true>, ReqF1>
        + Traversable<'t, ConstBool<true>, ConstBool<true>, ReqF1>
        + Functor<'t, ConstBool<true>, ConstBool<false>, ReqF1>
        // Unused bound from Applicative impl
        + CloneK<'t, ConstBool<true>>,
> Monad<'t, ConstBool<true>, ConstBool<true>, ReqF1> for VecT<TInner>
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> Self::F<'a, B>>);

        let f_tag = ReqF1::create_from(&f, ());

        // This compiles
        // let f = ReqF1::clone_one_of_5(&f);

        // TODO: Cannot type check for some reason
        // let f_clone = CloneWrapper(f, |f: &_| ReqF1::clone_one_of_5(f));

        // So some runtime overhead
        let f_clone = alloc::rc::Rc::new(f);

        let f_tag = CloneWrapper(f_tag, |f: &ReqF1::OneOf5F<'a, (), (), (), (), ()>| {
            ReqF1::clone_one_of_5(f)
        });

        fa.into_iter()
            .flat_map(move |a: <TInner as Hkt<'t>>::F<'a, A>| {
                // Both VecT and VecT<TInner> type checks - but only one is right?
                let traversed = TInner::traverse::<_, _, VecT, _, _, _, _, _>(
                    clone_a.clone(),
                    {
                        let clone_b = clone_b.clone();
                        move |b|   TInner::clone(clone_b.clone(), b)
                        },
                    ReqF1::clone_one_of_5(&*f_clone),
                    a,
                );

                traversed.into_iter().map({
                    let clone_b = clone_b.clone();
                    let f_tag = f_tag.clone();
                    move |ttb| {
                        TInner::bind(
                            {
                                let clone_b = clone_b.clone();
                                move |tb| {
                                    TInner::clone(clone_b.clone(), tb)
                                }
                            },
                            clone_b.clone(),
                            ttb,
                            ReqF1::create_from(&f_tag.0, identity),
                        )
                    }
                })

                // let clone_b2 = clone_b.clone();
                // let nested = TInner::map(
                //     clone_a.clone(),
                //     move |vec| {
                //         let clone_b = clone_b2.clone();
                //         vec.iter()
                //             .map(|tb: &<TInner as Hkt<'t>>::F<'a, B>| {
                //                 TInner::clone(clone_b.clone(), tb)
                //             })
                //             .collect::<Vec<_>>()
                //             .pipe(ReqIn::into_guarded)
                //     },
                //     ReqF1::clone_one_of_5(&*f_clone),
                //     a,
                // );

                // let f_tag = f_tag.clone();

                // TInner::fold_while(
                //     {
                //         let clone_b = clone_b.clone();
                //         move |vb: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                //             vb.iter()
                //                 .map(|b| TInner::clone(clone_b.clone(), b))
                //                 .collect::<Vec<_>>()
                //                 .pipe(ReqIn::into_guarded)
                //         }
                //     },
                //     {
                //         let clone_b = clone_b.clone();
                //         move |vb: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                //             vb.iter()
                //                 .map(|b| TInner::clone(clone_b.clone(), b))
                //                 .collect::<Vec<_>>()
                //                 .pipe(ReqIn::into_guarded)
                //         }
                //     },
                //     {
                //         let clone_b = clone_b.clone();
                //         ReqF1::create_from(
                //             &*f_clone,
                //             move |mut sum: Vec<<TInner>::F<'a, B>>,
                //                   next: Vec<<TInner>::F<'a, B>>| {
                //                 // Moved the reduce here:
                //                 let next = next.into_iter().reduce({
                //                     let f_tag = f_tag.clone();
                //                     {
                //                         let clone_b = clone_b.clone();
                //                         move |sum, b| {
                //                             let clone_b2 = clone_b.clone();
                //                             let b = CloneWrapper(b, move |b: &TInner::F<'a, B>| {
                //                                 TInner::clone(clone_b2.clone(), b)
                //                             });

                //                             // TODO: Use SemigroupK instead
                //                             TInner::bind(
                //                             clone_b.clone(),
                //                             clone_b.clone(),
                //                             sum,
                //                             ReqF1::create_from(&f_tag.0, move |_| b.clone().0)
                //                                 .pipe(|f| {
                //                                     ReqF1::arbitrary_t5(
                //                                         f,
                //                                         PhantomData::<fn(B) -> TInner::F<'a, B>>,
                //                                     )
                //                                 }),
                //                         )
                //                         }
                //                     }
                //                 });

                //                 if let Some(next) = next {
                //                     // Don't know if extend(Option) is optimized
                //                     sum.push(next);
                //                 }

                //                 // Correct if TInner == IdHkt
                //                 // sum.extend(next);

                //                 FoldWhile::Continue(sum)
                //             },
                //         )
                //         .pipe(|f| {
                //             ReqF1::arbitrary_t5(
                //                 f,
                //                 PhantomData::<
                //                     fn(
                //                         Vec<<TInner>::F<'a, B>>,
                //                         Vec<<TInner>::F<'a, B>>,
                //                     )
                //                         -> FoldWhile<Vec<<TInner>::F<'a, B>>>,
                //                 >,
                //             )
                //         })
                //     },
                //     // Copied from Vec::extend_desugared source code
                //     Vec::with_capacity(TInner::size_hint(&nested).0.saturating_add(1)),
                //     nested,
                // )
                // .converge()

                // Don't flatten - flat_map reduce instead
                // sum.into_iter().flat_map({
                //     let f_clone2 = f_clone.clone();
                //     let clone_b2 = clone_b.clone();
                //     move |iter| {
                //         let clone_b = clone_b2.clone();
                //         iter.into_iter().reduce({
                //             let f_clone3 = f_clone2.clone();
                //             move |sum, b| {
                //                 let clone_b2 = clone_b.clone();
                //                 let b = CloneWrapper(b, move |b: &TInner::F<'a, B>| {
                //                     TInner::clone(clone_b2.clone(), b)
                //                 });

                //                 TInner::bind(
                //                     clone_b.clone(),
                //                     clone_b.clone(),
                //                     sum,
                //                     ReqF1::create_from(&*f_clone3, move |_| b.clone().0).pipe(|f| {
                //                         ReqF1::arbitrary_t5(f, PhantomData::<fn(B) -> TInner::F<'a, B>>)
                //                     }),
                //                 )
                //             }
                //         })
                //     }
                // })
            })
            .collect::<Vec<_>>()
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t> + NotT5Of5<'t>,
    TInner: Traversable<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqOut>,
> Traversable<'t, ReqIn, ReqOut, ReqF1> for VecT<TInner>
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> F::F<'a, B>>);

        let clone_vec_reqin = {
            let clone_b = clone_b.clone();
            move |vec: &Vec<TInner::F<'a, B>>| {
                vec.iter()
                    .map(|b| TInner::clone(clone_b.clone(), b))
                    .collect::<Vec<_>>()
                    .pipe(ReqIn::into_guarded)
            }
        };

        let len = Self::size_hint(&fa).0.saturating_add(1);
        fa.into_iter().fold::<F::F<'a, Vec<TInner::F<'a, B>>>, _>(
            F::pure(
                clone_vec_reqin.clone(),
                Vec::<TInner::F<'a, B>>::with_capacity(len),
            ),
            {
                let clone_a = clone_a.clone();
                let clone_b = clone_b.clone();
                let f = ReqF1::clone_one_of_5(&f);
                let clone_vec_reqin = clone_vec_reqin.clone();

                move |sum: F::F<'a, Vec<TInner::F<'a, B>>>, a| {
                    let ta: F::F<'a, TInner::F<'a, B>> = TInner::traverse::<A, B, F, _, _, _, _, _>(
                        clone_a.clone(),
                        clone_b.clone(),
                        ReqF1::clone_one_of_5(&f),
                        a,
                    );

                    // let sum: F::F<'a, Vec<TInner::F<'a, B>>> = F::map(
                    //     |_| AssertBlankOutput,
                    //     |vec: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| vec.iter().map(|b| TInner::clone(clone_b.clone(), b)).collect::<Vec<_>>(),
                    //     ReqF1::create_from(&f, |iter| iter.collect::<Vec<_>>()),
                    //     sum
                    // );

                    let append = {
                        let clone_b = clone_b.clone();
                        let f = ReqF1::clone_one_of_5(&f);
                        let f = CloneWrapper(f, |f: &_| ReqF1::clone_one_of_5(f));

                        move |vec: Vec<TInner::F<'a, B>>| {
                            let vec = CloneWrapper(vec, {
                                let clone_b = clone_b.clone();
                                move |v: &Vec<TInner::F<'a, B>>| {
                                    v.iter()
                                        .map(|b| TInner::clone(clone_b.clone(), b))
                                        .collect::<Vec<_>>()
                                }
                            });

                            let f = f.clone().0;

                            let f = ReqF1::create_from(&f, {
                                let clone_b = clone_b.clone();
                                move |tb| {
                                    vec.get_ref_t()
                                        .iter()
                                        .map(|b| TInner::clone(clone_b.clone(), b))
                                        .chain([tb])
                                        .collect::<Vec<_>>()
                                }
                            });

                            ReqF1::arbitrary_t5(
                                f,
                                PhantomData::<fn(TInner::F<'a, B>) -> Vec<TInner::F<'a, B>>>,
                            )
                        }
                    };

                    // Obscure lifetime error about '1 having to outlive 'a? Just `move` everything!
                    let func: <F as Hkt<'t>>::F<
                        'a,
                        <ReqF1 as OneOf5Hkt<'t>>::OneOf5F<'a, _, _, _, _, _>,
                    > = <F as Functor<'t, ReqIn, ConstBool<false>, ReqF1>>::map::<
                        'a,
                        Vec<TInner::F<'a, B>>,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    >(
                        clone_vec_reqin.clone(),
                        |_| AssertBlankOutput,
                        ReqF1::create_from(&f, append).pipe(|f| {
                            ReqF1::arbitrary_t5(
                                f,
                                PhantomData::<fn(Vec<<TInner as Hkt<'t>>::F<'a, B>>) -> _>,
                            )
                        }),
                        sum,
                    );

                    F::apply::<TInner::F<'a, B>, Vec<TInner::F<'a, B>>, _, _, _, _, _>(
                        {
                            let clone_b = clone_b.clone();
                            move |a| TInner::clone(clone_b.clone(), a).pipe(ReqIn::into_guarded)
                        },
                        {
                            let clone_b = clone_b.clone();
                            move |b| {
                                b.iter()
                                    .map(|b| TInner::clone(clone_b.clone(), b))
                                    .collect::<Vec<_>>()
                                    .pipe(ReqOut::into_guarded)
                            }
                        },
                        func,
                        ta,
                    )
                }
            },
        )
    }
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for VecT<TInner> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a.into_iter()
            .map(TInner::covariant_convert)
            .collect::<Vec<_>>()
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: CloneOwnedK<'t, ReqIn>> CloneOwnedK<'t, ReqIn>
    for VecT<TInner>
{
    fn clone_owned<'a, 'b, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b,
    {
        a.iter()
            .map(|ta| TInner::clone_owned(clone_a.clone(), ta))
            .collect::<Vec<_>>()
    }
}

impl<'t, ReqIn: TypeGuard<'t>, TInner: CloneK<'t, ReqIn>> CloneK<'t, ReqIn> for VecT<TInner> {
    fn clone<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        a.iter()
            .map(|ta| TInner::clone(clone_a.clone(), ta))
            .collect::<Vec<_>>()
    }
}

#[cfg(false)]
unsafe impl<'t, TInner: CovariantTransmute<'t>> CovariantTransmute<'t> for VecT<TInner> {
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
