use std::{
    convert::{Infallible, identity},
    marker::PhantomData,
};

use tap::Pipe;

use crate::{
    hkt::{
        Applicative, CloneK, CovariantK, FoldWhile, Foldable, Functor, Hkt, HktUnsized, IntoEither,
        Monad, Pure, TCloneableOf5, Traversable, UnsizedHkt,
        UnsizedHktUnsized, boxed::BoxT, id::IdHkt, one_of::NotT5of5,
    },
    marker_classification::{AssertBlankOutput, ConstBool, TypeGuard},
    utils::CloneWrapper,
};

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
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
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

        fa.map(move |x| {
            TInner::map(
                clone_a.clone(),
                clone_b.clone(),
                ReqF1::clone_one_of_5(&f),
                x,
            )
        })
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
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
                clone_a.clone(),
                clone_b.clone(),
                ReqF1::clone_one_of_5(&f),
                b,
                ka,
            )
        })
    }
}

// Oops
// impl<
//     't,
//     ReqIn: TypeGuard<'t> + CloneK<'t>,
//     ReqOut: TypeGuard<'t> + CloneK<'t>,
//     ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
//     TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
// > Rfoldable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
// {
//     fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
//         clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
//         clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
//         f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
//         init: B,
//         mut fb: Self::F<'a, A>,
//     ) -> FoldWhile<B>
//     where
//         A: 'a,
//         B: 'b,
//         F1Once: 'f + FnOnce(B, A) -> FoldWhile<B>,
//         F1Mut: 'f + FnMut(B, A) -> FoldWhile<B>,
//         F1Fn: 'f + Fn(B, A) -> FoldWhile<B>,
//         F1Clone: 'f + Fn(B, A) -> FoldWhile<B> + Clone,
//         F1Copy: 'f + Fn(B, A) -> FoldWhile<B> + Copy,
//         'a: 'f,
//         'b: 'f,
//         't: 'a + 'b,
//     {
//         let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(B, A) -> FoldWhile<B>>);

//         fb.try_fold(init, move |b, ka| {
//             TInner::rfold_while(
//                 clone_a.clone(),
//                 clone_b.clone(),
//                 ReqF1::clone_one_of_5(&f),
//                 b,
//                 ka,
//             )
//         })
//     }
// }

impl<'t, ReqIn: TypeGuard<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn>
    for BoxT<DynIteratorT<TInner>>
{
    fn pure<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: A,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        TInner::pure(clone_a, a)
            .pipe(std::iter::once)
            .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqIn>, //+ CovariantK<'t>
> Applicative<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
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
        let fa = fa.collect::<Vec<_>>();

        let value = clone_a.clone();
        let clone_fa = move || {
            fa.iter()
                .map(|ta| TInner::clone(value.clone(), ta))
                .collect::<Vec<_>>()
        };

        ff.flat_map(move |f| {
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
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    // TODO: Consider if cloning funcs should be [Copy]
    ReqF1: TCloneableOf5<'t> + NotT5of5<'t>,
    TInner: Monad<'t, ReqIn, ReqIn, ReqF1>
        + Foldable<'t, ReqIn, ReqIn, ReqF1>
        // Unused bound from Applicative impl
        + CloneK<'t, ReqIn>,
> Monad<'t, ReqIn, ReqIn, ReqF1> for BoxT<DynIteratorT<TInner>>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqIn::Output<'a, B> + Clone,
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
        let f = ReqF1::arbitrary_uncloneable(f, PhantomData::<fn(A) -> Self::F<'a, B>>)
            // Remove this step when copying to Vec
            .pipe(|f| map_one_of_5_with!(ReqF1, f, |mut f| move |a| f(a).collect::<Vec<_>>()));

        let f = ReqF1::clone_one_of_5(&f);

        // TODO: Cannot type check for some reason
        // let f_clone = CloneWrapper(f, |f: &_| ReqF1::clone_one_of_5(f));

        // So some runtime overhead
        let f_clone = std::rc::Rc::new(f);

        fa.flat_map(move |a: <TInner as Hkt<'t>>::F<'a, A>| {
            let clone_b2 = clone_b.clone();
            let nested = TInner::map(
                clone_a.clone(),
                move |vec| {
                    let clone_b = clone_b2.clone();
                    vec.iter()
                        .map(|tb: &<TInner as Hkt<'t>>::F<'a, B>| {
                            TInner::clone(clone_b.clone(), tb)
                        })
                        .collect::<Vec<_>>()
                        .pipe(ReqIn::into_guarded)
                },
                ReqF1::clone_one_of_5(&*f_clone),
                a,
            );

            let f_clone2 = f_clone.clone();
            let sum = TInner::fold_while(
                {
                    let clone_b = clone_b.clone();
                    move |vb: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                        vb.iter()
                            .map(|b| TInner::clone(clone_b.clone(), b))
                            .collect::<Vec<_>>()
                            .pipe(ReqIn::into_guarded)
                    }
                },
                {
                    let clone_b = clone_b.clone();
                    move |vb: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                        vb.iter()
                            .map(|b| TInner::clone(clone_b.clone(), b))
                            .collect::<Vec<_>>()
                            .pipe(ReqIn::into_guarded)
                    }
                },
                {
                    let clone_b = clone_b.clone();
                    ReqF1::create_from(
                        &*f_clone,
                        move |mut sum: Vec<<TInner>::F<'a, B>>, next: Vec<<TInner>::F<'a, B>>| {
                            // Moved the reduce here:
                            let next = next.into_iter().reduce({
                                let f_clone3 = f_clone2.clone();
                                {
                                    let clone_b = clone_b.clone();
                                    move |sum, b| {
                                        let value = clone_b.clone();
                                        let b = CloneWrapper(b, move |b: &TInner::F<'a, B>| {
                                            TInner::clone(value.clone(), b)
                                        });

                                        TInner::bind(
                                            clone_b.clone(),
                                            clone_b.clone(),
                                            sum,
                                            ReqF1::create_from(&*f_clone3, move |_| b.clone().0)
                                                .pipe(|f| {
                                                    ReqF1::arbitrary_t5(
                                                        f,
                                                        PhantomData::<fn(B) -> TInner::F<'a, B>>,
                                                    )
                                                }),
                                        )
                                    }
                                }
                            });

                            if let Some(next) = next {
                                // Don't know if extend(Option) is optimized
                                sum.push(next);
                            }
                            FoldWhile::Continue(sum)
                        },
                    )
                    .pipe(|f| {
                        ReqF1::arbitrary_t5(
                            f,
                            PhantomData::<
                                fn(
                                    Vec<<TInner>::F<'a, B>>,
                                    Vec<<TInner>::F<'a, B>>,
                                )
                                    -> FoldWhile<Vec<<TInner>::F<'a, B>>>,
                            >,
                        )
                    })
                },
                // Copied from Vec::extend_desugared source code
                Vec::with_capacity(TInner::size_hint(&nested).0.saturating_add(1)),
                nested,
            )
            .into_either();

            sum
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
        .pipe(Box::new)
    }
}

impl<
    't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: TCloneableOf5<'t> + NotT5of5<'t>,
    TInner: Traversable<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqOut>,
> Traversable<'t, ReqIn, ReqOut, ReqF1> for BoxT<DynIteratorT<TInner>>
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
        fa.fold::<F::F<'a, Vec<TInner::F<'a, B>>>, _>(
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
                                        .pipe(Box::new)
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
        .pipe(|result: F::F<'a, Vec<TInner::F<'a, B>>>| {
            <F as Functor<'t, ReqIn, ConstBool<false>, ReqF1>>::map(
                {
                    let clone_b = clone_b.clone();
                    move |vec: &Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                        vec.iter()
                            .map(|b| TInner::clone(clone_b.clone(), b))
                            .collect::<Vec<_>>()
                            .pipe(ReqIn::into_guarded)
                    }
                },
                |_| AssertBlankOutput,
                ReqF1::create_from(&f, |vec: Vec<<TInner as Hkt<'t>>::F<'a, B>>| {
                    vec.into_iter().pipe(Box::new) as _
                }),
                result,
            )
        })
    }
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for BoxT<DynIteratorT<TInner>> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a.map(TInner::covariant_convert).pipe(Box::new)
    }
}
