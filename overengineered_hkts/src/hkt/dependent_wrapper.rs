use std::{convert::Infallible, marker::PhantomData};

use tap::Pipe as _;

use crate::hkt::{
    Applicative, CloneFreeApplicative, CovariantK, DependentCloneK, Functor, Hkt, Pure, UnsizedHkt, hkt_classification::{self, HktClassification}, id::IdHkt
};

pub struct DependentWrapperT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

// TODO: Wrapper, Maybe custom hkt to transform stack to wrapper-free stack
/// Strengths:
/// - Associated with specific HKT as opposed to bare object
/// - Can implement traits
///
/// Weakness:
/// - Is a different type, may clog the return type
/// - Has to be converted from/to original

#[derive(
    Debug, derive_more::AsRef, derive_more::AsMut, derive_more::Deref, derive_more::DerefMut,
)]
#[repr(transparent)]
pub struct DependentWrapper<'a, 't, TInner: Hkt<'t>, A: 't>(pub TInner::F<'a, A>) where 't: 'a;

impl<'t, TInner: Hkt<'t>> DependentWrapperT<TInner> {
    pub fn wrap<'a, A: 't>(a: TInner::F<'a, A>) -> DependentWrapper<'a, 't, TInner, A> {
        DependentWrapper::<TInner, _>(a)
    }
}

impl<'a, 't: 'a, TInner: Hkt<'t>, A: 't> DependentWrapper<'a, 't, TInner, A> {
    pub fn into_inner(self) -> TInner::F<'a, A> {
        self.0
    }
}

impl<'t, TInner: UnsizedHkt<'t>> UnsizedHkt<'t> for DependentWrapperT<TInner> {
    type UnsizedF<'a, A: 't> = DependentWrapper<'a, 't, TInner, A>
    where
        't: 'a;
}

impl<'t, TInner: Hkt<'t>> Hkt<'t> for DependentWrapperT<TInner> {
    type F<'a, A: 't> = DependentWrapper<'a, 't, TInner, A> where 't: 'a;
}

impl<'t, TInner: CovariantK<'t>> CovariantK<'t> for DependentWrapperT<TInner> {
    fn covariant_convert<'a, 'b, A: 't>(a: Self::F<'a, A>) -> Self::F<'b, A> where 
    'a: 'b,
    't: 'a + 'b {
        DependentWrapperT::wrap(a.into_inner().pipe(TInner::covariant_convert))
    }
}

impl<'a, 't:'a, TInner: DependentCloneK<'t> + CovariantK<'t>, A: 't + Clone> Clone for DependentWrapper<'a, 't, TInner, A> {
    fn clone(&self) -> Self {
        Self(TInner::clone(&self.0).pipe(TInner::covariant_convert))
    }
}

impl<'t, TInner: DependentCloneK<'t> + CovariantK<'t>> DependentCloneK<'t> for DependentWrapperT<TInner> {
    fn clone<'a, A: 't + Clone>(a: &Self::F<'a, A>) -> Self::F<'t, A> where 't: 'a {
        DependentWrapperT::wrap(TInner::clone(&a.0).pipe(TInner::covariant_convert))
    }
}

impl<'t, TInner: Functor<'t>> Functor<'t> for DependentWrapperT<TInner> {
    fn map<'a, A, B, F1>(f: F1, fa: Self::F<'a, A>) -> Self::F<'a, B>
    where
        A: 't,
        B: 't,
        F1: 'a + Fn(A) -> B + Clone,
        't: 'a {
        DependentWrapperT::wrap(TInner::map(f.clone(), fa.into_inner()))
    }
}

impl<'t, TInner: Pure<'t>> Pure<'t> for DependentWrapperT<TInner> {
    fn pure<'a, A: 't>(a: A) -> Self::F<'a, A> where 't: 'a {
        DependentWrapperT::wrap(TInner::pure(a))
    }
}

impl<'t, TInner: Applicative<'t>> Applicative<'t> for DependentWrapperT<TInner> {
    fn apply<'a, A, B, F1>(ff: Self::F<'a, F1>, fa: Self::F<'a, A>) -> Self::F<'a, B>
    where
        A: 't + Clone,
        B: 't,
        F1: 't + Fn(A) -> B + Clone,
        't: 'a {
        DependentWrapperT::wrap(TInner::apply(ff.into_inner(), fa.into_inner()))
    }
}

impl<'t, TInner: CloneFreeApplicative<'t>> CloneFreeApplicative<'t> for DependentWrapperT<TInner> {
    fn apply_once<'a, A, B, F1>(ff: Self::F<'a, F1>, fa: Self::F<'a, A>) -> Self::F<'a, B>
    where
        A: 't,
        B: 't,
        F1: 't + Fn(A) -> B + Clone,
        't: 'a {
        DependentWrapperT::wrap(TInner::apply_once(ff.into_inner(), fa.into_inner()))
    }
}

