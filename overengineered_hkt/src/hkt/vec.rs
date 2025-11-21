use std::{convert::Infallible, marker::PhantomData};

use tap::Pipe;

use crate::
    hkt::{
        Applicative, CloneK, CovariantK, FoldWhile, Foldable, Functor,
        Hkt, HktUnsized, Monad, Pure, PureMapInner, Rfoldable, TCloneableOf5, Traversable,
        UnsizedHkt, UnsizedHktUnsized,
        boxed::BoxT,
        id::IdHkt,
        nullary::NullaryHkt,
        reference::RefMutT,
    }
;

pub struct VecT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: Hkt<'t>> Hkt<'t> for VecT<TInner> {
    type F<'a, A: 't>
        = Vec<TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> HktUnsized<'t> for VecT<TInner> {
    type FUnsized<'a, A: 't + ?Sized>
        = Vec<TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for VecT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for VecT<TInner> {
    fn covariant_convert<'a, 'b, A: 't>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        'a: 'b,
        't: 'a + 'b,
    {
        a.into_iter().map(TInner::covariant_convert).collect()
    }
}

impl<'t, TInner: Functor<'t>> Functor<'t> for VecT<TInner> {
    fn map<'a, A, B, F1>(f: F1, fa: Self::F<'a, A>) -> Self::F<'a, B>
    where
        A: 't,
        B: 't,
        F1: 'a + Fn(A) -> B + Clone,
    {
        fa.into_iter()
            .map(|a| TInner::map(f.clone(), a))
            .collect_vec()
    }
}

impl<'t, TInner: Pure<'t>> Pure<'t> for VecT<TInner> {
    fn pure<'a, A: 't>(a: A) -> Self::F<'a, A> {
        vec![TInner::pure(a)]
    }
}

impl<'t, TInner: Applicative<'t> + CovariantK<'t> + DependentCloneK<'t>> Applicative<'t>
    for VecT<TInner>
{
    fn apply_f<'a, F, A, B, F1>(ff: Self::F<'a, F1>, fa: Self::F<'a, F::F<'t, A>>) -> Self::F<'a, B>
    where
        A: 't + Clone,
        B: 't,
        F1: 't + Fn(F::F<'t, A>) -> B + Clone,
        F: DependentCloneK<'t>,
        't: 'a {
        ff.into_iter()
            .flat_map(|f| {
                Self::clone_f::<F, _>(&fa)
                    .into_iter()
                    .map(move |a| TInner::apply_f::<F, _, _, _>(TInner::clone(&f), a).pipe(TInner::covariant_convert))
            })
            .collect()
    }
}

// impl<'t, K: Monad<'t> + Foldable<'t>> Monad<'t> for VecT<K> {
//     fn bind<'a, 'b, A, B, F1>(fa: Self::F<'a, A>, f: F1) -> Self::F<'b, B>
//     where
//         A: 't,
//         B: 't,
//         F1: Fn(A) -> Self::F<'b, B> + Clone {
//         fa.into_iter().flat_map(|ka|
//             K::fold_while(|sum, a|
//                     sum.tap_mut(|s| s.push(f.clone()(a)))
//                     .pipe(FoldWhile::Continue)
//                 ,
//                 Vec::new(),
//                 ka
//             ).unwrap_either()
//             .into_iter().flatten()
//             .collect_vec()
//         ).collect_vec()
//     }
// }

impl<'t, K: Monad<'t> + Foldable<'t> + DependentCloneK<'t> + CovariantK<'t>> Monad<'t> for VecT<K> {
    fn bind_f<'a, F, A, B, F1>(fa: Self::F<'a, A>, f: F1) -> Self::F<'a, F::F<'t, B>>
    where
        A: 't,
        B: 't + Clone,
        F1: 'a + (Fn(A) -> Self::F<'a, F::F<'t, B>>) + Clone,
        F: DependentCloneK<'t>,
        't: 'a {
fa.into_iter()
            .map(|ka| {
                K::fold_while(
                    |mut sum, a| {
                        sum.extend(f(a));
                        sum.pipe(FoldWhile::Continue)
                    },
                    Vec::new(),
                    ka,
                )
                .unwrap_either()
                .into_iter()
                .fold(Option::None, |prev, next|
                    match prev {
                        Some(prev) => {
                            // TODO: Replace Rc with DependentWrapper
                            let next = DependentWrapperT::<BindT<K, F>>::wrap(next);
                            Some(
                            K::bind_f::<F, _, _, _>(prev, move|_| next.clone().into_inner().pipe(K::covariant_convert))
                        )},
                        None => Some(next),
                    }
                )
            })
            .filter_map(identity)
            .collect_vec()
    }
}

impl<'t, TInner: DependentCloneK<'t>> DependentCloneK<'t> for VecT<TInner> {
    fn clone_f<'a, 'b, F: DependentCloneK<'t, super::SeperateLifetime>, A: 'a + Clone>(a: &Self::F<'a, F::F<'a, A>>) -> Self::F<'b, F::F<'b, A>> where 't: 'a, super::SeperateLifetime: super::EncodeLifetime<'a, 'b> {
        a
    }
}

impl<'t, TInner: Foldable<'t>> Foldable<'t> for VecT<TInner> {
    fn fold_while<'a, A, B, F>(f: F, init: B, fb: Self::F<'a, A>) -> FoldWhile<B>
    where
        A: 't,
        F: Fn(B, A) -> FoldWhile<B> + Clone,
        't: 'a,
    {
        fb.into_iter()
            .try_fold(init, |b, ka| TInner::fold_while(f.clone(), b, ka))
    }
}

impl<'t, TInner: Traversable<'t> + DependentCloneK<'t> + CovariantK<'t>> Traversable<'t> for VecT<TInner> {
    fn traverse_f<'a, K, A, B, F1, F>(f: F1, fa: Self::F<'a, A>) -> F::F<'a, Self::F<'a, K::F<'t, B>>>
    where
        A: 't,
        B: 't + Clone,
        F1: Fn(A) -> F::F<'a, K::F<'t, B>> + Clone,
        K: DependentCloneK<'t>,
        F: Applicative<'t> + DependentCloneK<'t>,
        't: 'a {
        let len = fa.len();
        fa.into_iter().fold(
            F::pure(Vec::with_capacity(len)),
            |sum: F::F<'a, Vec<TInner::F<'a, B>>>, a: TInner::F<'a, A>| {
                // Make sure the correct Hkt is passed (F)

                // F<TInner>::<B> => TODO: Transformers while agnostic over F<A>
                let rs: F::F<'a, TInner::F<'a, B>> = TInner::traverse::<_, _, _, F>(f.clone(), a);
                let rs: DependentWrapper<'_, 't, F, DependentWrapper<'a, 't, TInner, B>> =
                    DependentWrapperT::<F>::wrap(
                        rs.pipe(|a| F::map(DependentWrapperT::<TInner>::wrap, a)),
                    );

                // immutable
                #[cfg(false)]
                let append = move |v: Vec<TInner::F<'a, B>>| {
                    let v = Rc::new(v);
                    move |ftb: DependentWrapper<'a, 't, TInner, B>| {
                        v.clone()
                            .iter()
                            .map(TInner::clone)
                            .chain([ftb.into_inner()])
                            .collect()
                    }
                };

                // mutable
                #[cfg(true)]
                let append = move |v: Vec<TInner::F<'a, B>>| {
                    let v = Rc::new(RefCell::new(v));
                    move |ftb: DependentWrapper<'a, 't, TInner, B>| {
                        v.try_borrow_mut()
                            .expect("No concurrency here")
                            // Wouldn't this mutate the original?
                            .push(ftb.into_inner());

                        v.clone()
                    }
                };

                let func = F::map(append, sum);

                let func = DependentWrapperT::<F>::wrap(func);

                // Immutable
                // Clone here or something
                #[cfg(false)]
                return DependentWrapperT::<F>::apply(func, rs).into_inner().pipe(|a| F::clone(&DependentWrapperT::<VecT<_>>::wrap(a)));

                // Mutable
                #[cfg(true)]
                return DependentWrapperT::<F>::apply(func, rs)
                    .into_inner()
                    .pipe(|a| {
                        F::map(
                            |a| match Rc::try_unwrap(a) {
                                Ok(v) => v.into_inner(),
                                Err(rc) => rc
                                    .try_borrow()
                                    .expect("All borrows cannot live till here")
                                    .iter()
                                    .map(TInner::clone)
                                    .collect(),
                            },
                            a,
                        )
                    })
                    .pipe(wrap_hkt_3_unsized::<B, F, VecT, TInner>)
                    .into_inner()
                    .pipe_ref(F::clone);
            },
        )
    }

    fn traverse_once<'a, A, B, F1, F>(f: F1, fa: Self::F<'a, A>) -> F::F<'a, Self::F<'a, B>>
    where
        A: 't,
        B: 't,
        F1: Fn(A) -> F::F<'a, B> + Clone,
        F: CloneFreeApplicative<'t>,
        't: 'a {
        let len = fa.len();
        fa.into_iter().fold(
            F::pure(Vec::with_capacity(len)),
            |sum: F::F<'a, Vec<TInner::F<'a, B>>>, a: TInner::F<'a, A>| {
                // Make sure the correct Hkt is passed (F)
                let rs: F::F<'a, TInner::F<'a, B>> =
                    TInner::traverse_once::<_, _, _, F>(f.clone(), a);
                let rs: DependentWrapper<'_, F, DependentWrapper<'a, TInner, B>> =
                    DependentWrapperT::<F>::wrap(
                        rs.pipe(|a| F::map(DependentWrapperT::<TInner>::wrap, a)),
                    );

                // immutable
                #[cfg(false)]
                let append = move |v: Vec<TInner::F<'a, B>>| {
                    let v = Rc::new(v);
                    move |ftb: DependentWrapper<'a, TInner, B>| {
                        v.clone()
                            .iter()
                            .map(TInner::clone)
                            .chain([ftb.into_inner()])
                            .collect()
                    }
                };

                // mutable
                #[cfg(true)]
                let append = move |v: Vec<TInner::F<'a, B>>| {
                    use crate::hkt::dependent_wrapper::DependentWrapper;

                    let v = Rc::new(RefCell::new(v));
                    move |ftb: DependentWrapper<'a, TInner, B>| {
                        v.try_borrow_mut()
                            .expect("No concurrency here")
                            .push(ftb.into_inner());

                        v.clone()
                    }
                };

                let func = F::map(append, sum);

                let func = DependentWrapperT::<F>::wrap(func);

                // Immutable
                #[cfg(false)]
                return DependentWrapperT::<F>::apply_once(func, rs).into_inner();

                // Mutable
                #[cfg(true)]
                return DependentWrapperT::<F>::apply_once(func, rs)
                    .into_inner()
                    .pipe(|a| {
                        F::map(|a| Rc::into_inner(a).expect("Rc is unique").into_inner(), a)
                    });
            },
        )
    }
}

impl<'t> IntoIteratorHkt<'t> for VecT {
    fn into_iter<'a, A: 't>(iter: Self::F<'a, A>) -> impl Iterator<Item = A>
    where
        't: 'a,
    {
        iter.into_iter()
    }
}

