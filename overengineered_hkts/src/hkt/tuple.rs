use std::convert::Infallible;

use either::Either::Right;

use crate::hkt::{CloneFnHkt, CloneK, Functor, Hkt, HktUnsized, PhantomMarker, UnsizedHkt, UnsizedHktUnsized, hkt_classification::{self, HktClassification}, id::IdHkt};

impl<'t, Left: Hkt<'t>, Right: Hkt<'t>> Hkt<'t> for (Left, Right) {
    type F<'a, A: 'a> = (Left::F<'a, A>, Right::F<'a, A>)
    where
        't: 'a;
}

impl<'t, Left: HktUnsized<'t>, Right: HktUnsized<'t>> HktUnsized<'t> for (Left, Right) {
    type FUnsized<'a, A: 'a + ?Sized> = (Left::FUnsized<'a, A>, Right::FUnsized<'a, A>) where 't: 'a
    ;
}

impl<Left, Right> HktClassification for (Left, Right) {
    type Choice = hkt_classification::OuterHkt;
}

// impl<'t, T: 't, ReqIn: CloneK<'t, ReqIn> + CloneFnHkt<'t>> CloneK<'t, ReqIn> for UnitHkt<T> {
//     fn clone<'a, 'b, A: 'a + 'b>(requirements: impl 'a + Into<<ReqIn>::F<'a, A>>, a: &Self::F<'a, A>) -> Self::F<'b, A>
//     where
//         't: 'a + 'b {
//         // Expected A got T
//         ReqIn::call_clone(requirements.into(), a)
//     }
// }

// pub type UnitTupleHkt = ();

// impl<'t> UnsizedHkt<'t> for UnitTupleHkt {
//     type UnsizedF<'a, A: 'a> = () where 't: 'a;
// }

// impl<'t> Hkt<'t> for UnitTupleHkt {
//     type F<'a, A: 'a> = () where 't: 'a;
// }

// impl<'t> UnsizedHktUnsized<'t> for UnitTupleHkt {
//     type UnsizedFUnsized<'a, A: 'a + ?Sized> = () where 't: 'a;
// }

// impl HktClassification for UnitTupleHkt {
//     type Choice = hkt_classification::TransparentHkt;
// }

// impl<'t> Functor<'t> for UnitTupleHkt {
//     fn map<'a, A, B, F1>(
//         requirements: impl 'a + Into<(<UnitHkt as Hkt<'t>>::F<'a, A>, <UnitHkt as Hkt<'t>>::F<'a, B>, <UnitHkt as Hkt<'t>>::F<'a, F1>)>,
//         f: F1,
//         fa: Self::F<'a, A>,
//     ) -> Self::F<'a, B>
//     where
//         A: 'a,
//         B: 'a,
//         F1: 'a + FnOnce(A) -> B,
//         't: 'a {
//         fa
//     }
// }

// impl<'t, ReqIn: Hkt<'t>> CloneK<'t, ReqIn> for UnitTupleHkt {
//     fn clone<'a, 'b, A: 'a + 'b>(requirements: impl 'a + Into<<ReqIn>::F<'a, A>>, a: &Self::F<'a, A>) -> Self::F<'b, A>
//     where
//         't: 'a + 'b {
//         a.clone()
//     }
// }


// pub trait GenericFunction<'t, ReqIn: Hkt<'t> = UnitHkt>: Hkt<'t> {
//     fn call<'a, A>(&self, requirements: impl 'a + Into<ReqIn::F<'a, A>>, a: A) -> Self::F<'a, A> where 't: 'a;
// }

// pub trait VariadicFunctor<'t> {
//     type Target<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t>>: 'a where 't: 'a, Self: 'a;
//     fn variadic_map<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t>>(self, requirements: impl 'a + Into<<ReqIn>::F<'a, Self>>, f: F) -> Self::Target<'a, F, ReqIn> where 't: 'a, Self: 'a + Sized;
// }

// impl<'t> VariadicFunctor<'t> for () {
//     type Target<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t>> = () where 't: 'a, Self: 'a;

//     fn variadic_map<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t>>(self: Self, requirements: impl 'a + Into<<ReqIn>::F<'a, Self>>, f: F) -> Self::Target<'a, F, ReqIn> where 't: 'a, Self: 'a + Sized {
//         self
//     }
// }



// impl<'t, A: 't, B: VariadicFunctor<'t>> VariadicFunctor<'t> for (A, B) {
//     type Target<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t>> = (F::F<'a, A>, B::Target<'a, F, ReqIn>) where 't: 'a, Self: 'a;

//     fn variadic_map<'a, F: GenericFunction<'t, ReqIn>, ReqIn: CloneK<'t> + Functor<'t>>(self: Self, requirements: impl 'a + Into<<ReqIn>::F<'a, Self>>, f: F) -> Self::Target<'a, F, ReqIn> where 't: 'a, Self: 'a + Sized {
//         let requirements = requirements.into();
//         let ra = ReqIn::map(
//             |a| a.0,
//             ReqIn::clone((), &requirements),
//         );

//         let rb = ReqIn::map(
//             |a| a.1,
//             ReqIn::clone((), &requirements),
//         );


//         (f.call(ra,self.0), self.1.variadic_map(rb, f))
//     }
// }
