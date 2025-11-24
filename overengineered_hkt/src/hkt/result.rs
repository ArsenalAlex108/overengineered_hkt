use core::{convert::Infallible, ops::Add};

use tap::Pipe as _;

use crate::{
    hkt::{
        Applicative, CloneK, CloneOwnedK, CovariantK, FoldWhile, Foldable, Functor, Hkt, HktUnsized, Monad, PhantomMarker, Pure, Rfoldable, Traversable, hkt_classification::{self, HktClassification}, id::IdHkt, one_of::{OneOf5Hkt, OneOf5}
    }, marker_classification::{ConstBool, TypeGuard}
};

pub struct ResultT<E, TInner = IdHkt>(Infallible, PhantomMarker<(TInner, E)>);

impl<'t, TInner: Hkt<'t>, E: 't> Hkt<'t> for ResultT<E, TInner> {
    type F<'a, A: 'a> = Result<TInner::F<'a, A>, E> where 't: 'a;
}

impl<'t, TInner: HktUnsized<'t>, E: 't> HktUnsized<'t> for ResultT<E, TInner> {
    type FUnsized<'a, A: 'a + ?Sized> = Result<TInner::FUnsized<'a, A>, E> where 't: 'a;
}

impl<TInner, E> HktClassification for ResultT<E, TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<
    't,
    E: 't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for ResultT<E, TInner>
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
        fa.map(move |x| {
            TInner::map(
                clone_a.clone(),
                clone_b.clone(),
                f,
                x,
            )
        })
    }
}

impl<
    't,
    E: 't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Foldable<'t, ReqIn, ReqOut, ReqF1>,
> Foldable<'t, ReqIn, ReqOut, ReqF1> for ResultT<E, TInner>
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
            Ok(t) => TInner::fold_while(
                clone_a.clone(),
                clone_b.clone(),
                f,
                init,
                t,
            ),
            Err(_) => FoldWhile::Break(init)
        }
    }
}

impl<
    't,
    E: 't,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Rfoldable<'t, ReqIn, ReqOut, ReqF1>,
> Rfoldable<'t, ReqIn, ReqOut, ReqF1> for ResultT<E, TInner>
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
            Ok(t) => TInner::rfold_while(
                clone_a.clone(),
                clone_b.clone(),
                f,
                init,
                t,
            ),
            Err(_) => FoldWhile::Break(init)
        }
    }
}

impl<'t, E: 't, ReqIn: TypeGuard<'t>, TInner: Pure<'t, ReqIn>> Pure<'t, ReqIn>
    for ResultT<E, TInner>
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
            .pipe(Ok)
    }
}

impl<
    't,
    E: 't + Add<Output = E>,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Applicative<'t, ReqIn, ReqOut, ReqF1>,
> Applicative<'t, ReqIn, ReqOut, ReqF1> for ResultT<E, TInner>
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
            (Ok(f), Ok(a)) => Ok(TInner::apply(clone_a, clone_b, f, a)),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(f), Err(a)) => Err(f + a),
        }
    }
}

impl<
    't,
    E: 't + Add<Output = E> + Clone,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    // TODO: Consider if cloning funcs should be [Copy]
    ReqF1: OneOf5Hkt<'t>,
> Monad<'t, ReqIn, ReqOut, ReqF1> for ResultT<E>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
            fa.and_then(f)
        )
    }
}

impl<
    't,
    E: 't + Add<Output = E> + Clone,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
    TInner: Traversable<'t, ReqIn, ReqOut, ReqF1> + CloneK<'t, ReqOut>,
> Traversable<'t, ReqIn, ReqOut, ReqF1> for ResultT<E, TInner>
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
        let f_map = ReqF1::create_from(&f, |i| Ok(i));

        match fa {
            Ok(ta) => {
                let res: F::F<'a, TInner::F<'a, B>> = TInner::traverse::<_, _, F, _, _, _, _, _>(
                    clone_a.clone(),
                    clone_b.clone(),
                    f,
                    ta
                );

                <F as Functor<'t, ReqIn, ReqOut, ReqF1>>::map(
                    {
                    let clone_b = clone_b.clone();
                    move |a| TInner::clone(clone_b.clone(), a).pipe(ReqIn::into_guarded)
                    },
                    move |rb: &Result<<TInner as Hkt<'t>>::F<'a, B>, E>| match rb.as_ref() {
                        Ok(tb) => Ok(
                            TInner::clone(clone_b.clone(), tb)
                        ),
                        Err(e) => Err(e.clone()),
                    }.pipe(ReqOut::into_guarded),
                    f_map,
                    res
                )
            },
            Err(e) => F::pure(move |res| match res.as_ref() {
                        Ok(tb) => Ok(
                            TInner::clone(clone_b.clone(), tb)
                        ),
                        Err(e) => Err(e.clone()),
                    }.pipe(ReqIn::into_guarded)
                 , Err(e)),
        }
    }
}

impl<'t, E: 't, TInner: CovariantK<'t>> CovariantK<'t> for ResultT<E, TInner> {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a.map(TInner::covariant_convert)
    }
}

impl<'t, E: 't + Clone, ReqIn: TypeGuard<'t>, TInner: CloneOwnedK<'t, ReqIn>> CloneOwnedK<'t, ReqIn>
    for ResultT<E, TInner>
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
            Ok(ta) => Ok(TInner::clone_owned(clone_a, ta)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'t, E: 't + Clone, ReqIn: TypeGuard<'t>, TInner: CloneK<'t, ReqIn>> CloneK<'t, ReqIn> for ResultT<E, TInner> {
    fn clone<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        match a {
            Ok(ta) => Ok(TInner::clone(clone_a, ta)),
            Err(e) => Err(e.clone()),
        }
    }
}
