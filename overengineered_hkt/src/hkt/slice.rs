use std::{convert::Infallible, marker::PhantomData};

use crate::hkt::{Hkt, HktUnsized, UnsizedHkt, UnsizedHktUnsized, hkt_classification::{self, HktClassification}, id::IdHkt};

pub struct SliceT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: Hkt<'t>> UnsizedHkt<'t> for SliceT<TInner> {
    type UnsizedF<'a, A: 'a> = [TInner::F<'a, A>] where 't: 'a;
}

impl<'t, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t> for SliceT<TInner> {
    type UnsizedFUnsized<'a, A: 'a + ?Sized> = [TInner::FUnsized<'a, A>] where 't: 'a;
}

impl<TInner> HktClassification for SliceT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}
