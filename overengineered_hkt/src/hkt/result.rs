use std::{cell::Cell, convert::Infallible, fmt::Debug, marker::PhantomData, rc::Rc};

use naan::fun::{F1, F2Once};

use crate::{
    hkt::{
        Applicative, CloneFreeApplicative, Functor, Hkt, HktUnsized, Monad, PhantomMarker, Pure,
    },
    utils::AsFn,
};

pub struct ResultT<TInner, E>(Infallible, PhantomMarker<(TInner, E)>);

impl<'a, TInner: Hkt<'a>, E: 'a> Hkt<'a> for ResultT<TInner, E> {
    type F<A: 'a> = Result<TInner::F<A>, E>;
}

impl<'a, TInner: HktUnsized<'a>, E: 'a> HktUnsized<'a> for ResultT<TInner, E> {
    type FUnsized<A: 'a + ?Sized> = Result<TInner::FUnsized<A>, E>;
}

impl<'a, TInner: Functor<'a>, E: 'a> Functor<'a> for ResultT<TInner, E> {
    fn map<A, B, F>(f: F, fa: Self::F<A>) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.map(TInner::map.curry().call(f).as_fn())
    }
}

impl<'a, TInner: Pure<'a>, E: 'a> Pure<'a> for ResultT<TInner, E> {
    fn pure<A>(a: A) -> Self::F<A> {
        Ok(TInner::pure(a))
    }
}

impl<'a, TInner: Applicative<'a>, E: 'a> Applicative<'a> for ResultT<TInner, E> {
    fn apply<A, B, F>(ff: Self::F<F>, fa: Self::F<A>) -> Self::F<B>
    where
        A: 'a + Clone,
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.and_then(|a| ff.map(|f| TInner::apply(f, a)))
    }
}

impl<'a, TInner: CloneFreeApplicative<'a>, E: 'a> CloneFreeApplicative<'a> for ResultT<TInner, E> {
    fn apply_once<A, B, F>(ff: Self::F<F>, fa: Self::F<A>) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.and_then(|a| ff.map(|f| TInner::apply_once(f, a)))
    }
}

// TODO
impl<'a, TInner: Monad<'a> /* + Traversable<'a> */, E: 'a + Debug> Monad<'a>
    for ResultT<TInner, E>
{
    fn bind<A, B, F>(fa: Self::F<A>, f: F) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> Self::F<B> + Clone,
    {
        fa.and_then(|ta| {
            let has_none_val_org = Rc::new(Cell::new(false));
            let has_none_val = Rc::clone(&has_none_val_org);
            let stack = TInner::map(
                move |a| match f.clone()(a) {
                    Ok(a) => Ok(a),
                    Err(e) => {
                        has_none_val.set(true);
                        Err(e)
                    }
                },
                ta,
            );

            if has_none_val_org.clone().take() {
                Err(panic!("Not impl."))
            } else {
                Ok(TInner::bind(stack, |a| {
                    a.expect("OptionT bind impl is correct.")
                }))
            }
        })
    }
}
