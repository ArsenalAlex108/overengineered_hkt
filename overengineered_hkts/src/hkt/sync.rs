use core::{convert::Infallible, marker::PhantomData};

use alloc::sync::Arc;

use crate::hkt::{
    DerefHkt, Hkt, HktUnsized, UnsizedHkt, UnsizedHktUnsized,
    hkt_classification::{self, HktClassification},
    id::IdHkt,
};

pub struct ArcT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for ArcT<TInner> {
    type F<'a, A: 'a>
        = Arc<TInner::UnsizedF<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for ArcT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Arc<TInner::UnsizedFUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for ArcT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}

impl<'t, TInner: DerefHkt<'t>> DerefHkt<'t> for ArcT<TInner> {
    fn deref<'a, 'b: 'a, A: 'a + ?Sized>(fa: &'a Self::UnsizedFUnsized<'b, A>) -> &'a A
    where
        't: 'b + 'a,
    {
        TInner::deref(fa)
    }
}

// // Doesn't work:
// pub struct ArcLT<'a, TInner>(Infallible, PhantomData<&'a TInner>);

// impl<'a, TInner: UnsizedTransmuteHKT<'a>> TransmuteHKT<'a> for ArcLT<'a, TInner> {
//     type Transmutable<A: 'a> = Arc<TInner::UnsizedTransmutable<A>>;
// }

// type ArcAL<TInner> = ArcLT<'static, TInner>;

#[cfg(not(feature = "no-std"))]
pub use use_std::*;

#[cfg(not(feature = "no-std"))]
mod use_std {
    use core::{convert::Infallible, marker::PhantomData};

    use std::sync::Mutex;

    use crate::hkt::{
        Hkt, HktUnsized, hkt_classification::{self, HktClassification},
        id::IdHkt,
    };

    ///
    /// # Safety
    ///
    /// Transmutation doesn't cause data races because it either copies a pointer or moved data, which references to are proved not to exist.
    pub struct MutexT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

    impl<'t, TInner: Hkt<'t>> Hkt<'t> for MutexT<TInner> {
        type F<'a, A: 'a>
            = Mutex<TInner::F<'a, A>>
        where
            't: 'a;
    }

    impl<'t, TInner: HktUnsized<'t>> HktUnsized<'t> for MutexT<TInner> {
        type FUnsized<'a, A: 'a + ?Sized>
            = Mutex<TInner::FUnsized<'a, A>>
        where
            't: 'a;
    }

    impl<TInner> HktClassification for MutexT<TInner> {
        type Choice = hkt_classification::OuterHkt;
    }
}
