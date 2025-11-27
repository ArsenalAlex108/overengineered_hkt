use core::{convert::Infallible, marker::PhantomData};

use crate::{hkt::{Hkt, HktUnsized, UnsizedHkt, UnsizedHktUnsized, hkt_classification::{self, HktClassification}, id::IdHkt}, transmute::unsafe_transmute_id};

pub struct RefT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for RefT<TInner> {
    type F<'a, A: 'a> = &'a TInner::UnsizedF<'a, A> where 't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for RefT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized> = &'a TInner::UnsizedFUnsized<'a, A> where 't: 'a;
}

impl<TInner> HktClassification for RefT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

#[cfg(false)]
unsafe impl<'t, TInner: CovariantTransmute<'t>> CovariantTransmute<'t> for RefT<TInner> {
    fn covariant_transmute<'a, 'f, 'b, A, F: CovariantTransmute<'t>>(s: 
        F::F<'f, Self::F<'a, A>>) -> F::F<'f, Self::F<'b, A>>
    where
        A: 'a,
        'a: 'b,
        't: 'a {
        // SAFETY: https://doc.rust-lang.org/reference/subtyping.html#variance
        unsafe { unsafe_transmute_id(s) }
    }
}

pub struct RefMutT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for RefMutT<TInner> {
    type F<'a, A: 'a> = &'a mut TInner::UnsizedF<'a, A> where 't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for RefMutT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized> = &'a mut TInner::UnsizedFUnsized<'a, A> where 't: 'a;
}

impl<TInner> HktClassification for RefMutT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}
