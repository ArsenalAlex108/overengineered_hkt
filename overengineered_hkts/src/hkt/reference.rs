use core::{convert::Infallible, marker::PhantomData};

use crate::hkt::{Hkt, HktUnsized, UnsizedHkt, UnsizedHktUnsized, hkt_classification::{self, HktClassification}, id::IdHkt};

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

pub struct RefMutT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for RefMutT<TInner> {
    type F<'a, A: 'a> = &'a TInner::UnsizedF<'a, A> where 't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for RefMutT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized> = &'a TInner::UnsizedFUnsized<'a, A> where 't: 'a;
}

impl<TInner> HktClassification for RefMutT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}
