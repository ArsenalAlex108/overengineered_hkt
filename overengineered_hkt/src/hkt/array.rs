use std::{cell::RefCell, convert::Infallible, marker::PhantomData, rc::Rc};

use naan::fun::{F1, F2Once};
use tap::Pipe;

use crate::hkt::dependent_wrapper::{DependentWrapper, DependentWrapperT};
use crate::hkt::{FoldWhile, IntoIteratorHkt};
use crate::{
    hkt::{
        Applicative, CloneFreeApplicative, DependentCloneK, Foldable, Functor, Hkt, HktUnsized,
        Monad, Pure, Traversable, id::IdHkt,
    },
    utils::AsFn,
};

pub struct ArrayT<const N: usize, TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'a, TInner: Hkt<'a>, const N: usize> Hkt<'a> for ArrayT<N, TInner> {
    type F<A: 'a> = [TInner::F<A>; N];
}

impl<'a, TInner: HktUnsized<'a>, const N: usize> HktUnsized<'a> for ArrayT<N, TInner> {
    type FUnsized<A: 'a + ?Sized> = [TInner::FUnsized<A>; N];
}

impl<'a, TInner: Functor<'a>, const N: usize> Functor<'a> for ArrayT<N, TInner> {
    fn map<A, B, F: 'a + FnMut(A) -> B + Clone>(f: F, fa: Self::F<A>) -> Self::F<B> {
        fa
            .map(TInner::map.curry().call(f).as_fn())
    }
}

impl<'a, TInner: Pure<'a>> Pure<'a> for ArrayT<1, TInner> {
    fn pure<A: 'a>(a: A) -> Self::F<A> {
        [TInner::pure(a)]
    }
}

impl<'a, TInner: Pure<'a>> Pure<'a> for ArrayT<0, TInner> {
    fn pure<A: 'a>(_: A) -> Self::F<A> {
        []
    }
}

impl<'a, TInner: DependentCloneK<'a>, const N: usize> DependentCloneK<'a> for ArrayT<N, TInner> {
    fn clone<A: 'a + Clone>(a: &Self::F<A>) -> Self::F<A> {
        a.each_ref().map(TInner::clone)
    }
}

impl<'a, TInner: Foldable<'a>, const N: usize> Foldable<'a> for ArrayT<N, TInner> {
    fn fold_while<A, B, F>(f: F, init: B, fb: Self::F<A>) -> FoldWhile<B>
    where
        F: 'a + FnMut(B, A) -> FoldWhile<B> + Clone,
    {
        fb.into_iter()
            .try_fold(init, |b, ka| TInner::fold_while(f.clone(), b, ka))
    }
}

