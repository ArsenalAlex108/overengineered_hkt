use core::convert::Infallible;


use crate::hkt::CloneOwnedK;
use crate::hkt::CovariantK;
use crate::hkt::CovariantRefK;
use crate::hkt::Rfoldable;
use crate::hkt::UnsizedHkt;
use crate::hkt::UnsizedHktUnsized;

use crate::hkt::FoldWhile;
use crate::hkt::hkt_classification;
use crate::hkt::hkt_classification::HktClassification;
use crate::hkt::one_of::OneOf5;
use crate::hkt::one_of::OneOf5Hkt;
use crate::hkt::reference::RefMutT;
use crate::hkt::reference::RefT;
use crate::hkt::{Applicative, CloneK, Foldable, Functor, Hkt, Monad, Pure, Traversable};
use crate::marker_classification::ConstBool;
use crate::marker_classification::TypeGuard;

// error[E0309]: the parameter type `A` may not live long enough
//   --> src\hkt\id.rs:15:25
//    |
// 15 |     type F<'a, A: 'a> = Vec<A>;
//    |            --           ^^^^^^ ...so that the type `std::vec::Vec<A>` will meet its required lifetime bounds...
//    |            |
//    |            the parameter type `A` must be valid for the lifetime `'a` as defined here...
//    |
// note: ...that is required by this bound
//   --> src\hkt\traits.rs:47:24
//    |
// 47 |     type F<'a, A: 'a>: 'a + TyEq<Self::UnsizedF<'a, A>>;
//    |                        ^^
// help: consider adding an explicit lifetime bound
//    |
// 15 |     type F<'a, A: 'a + 'a> = Vec<A>;
//    |                      ++++

/// Marker type for IdentityHkt
pub struct IdHkt(Infallible);

impl<'t> UnsizedHkt<'t> for IdHkt {
    type UnsizedF<'a, A: 'a>
        = A
    where
        't: 'a;
}

impl<'t> Hkt<'t> for IdHkt {
    type F<'a, A: 'a>
        = A
    where
        't: 'a;
}

impl<'t> UnsizedHktUnsized<'t> for IdHkt {
    type UnsizedFUnsized<'a, A: 'a + ?Sized>
        = A
    where
        't: 'a;
}

impl HktClassification for IdHkt {
    type Choice = hkt_classification::TransparentHkt;
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Functor<'t, ReqIn, ReqOut, ReqF1> for IdHkt
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
        // ReqF1::map_one_of_six(
        //     f,
        //     |f| f(fa),
        //     |mut f| f(fa),
        //     |f| f(fa),
        //     |f| f(fa),
        //     |f| f(fa),
        //     |(f, into_ptr)| f(fa)
        // ).pipe(ReqF1::into_all)
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Foldable<'t, ReqIn, ReqOut, ReqF1> for IdHkt
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
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
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fb))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Foldable<'t, ReqIn, ReqOut, ReqF1, RefT> for IdHkt
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b {
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Foldable<'t, ReqIn, ReqOut, ReqF1, RefMutT> for IdHkt
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefMutT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b {
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Rfoldable<'t, ReqIn, ReqOut, ReqF1> for IdHkt
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
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
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fb))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefT> for IdHkt
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b {
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Rfoldable<'t, ReqIn, ReqOut, ReqF1, RefMutT> for IdHkt
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'f + Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        _clone_b: impl 'f + Fn(&B) -> <ReqOut>::Output<'b, B> + Clone,
        f: <ReqF1>::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: <RefMutT as Hkt<'t>>::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, <RefMutT as Hkt<'t>>::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b {
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(init, fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>> Pure<'t, ReqIn> for IdHkt {
    fn pure<'a, A>(_clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone, a: A) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        a
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Applicative<'t, ReqIn, ReqOut, ReqF1> for IdHkt
{
    fn apply<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
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
        match_one_of_5!(ReqF1::into_one_of_5_enum(ff), f, f(fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Monad<'t, ReqIn, ReqOut, ReqF1> for IdHkt
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
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(fa))
    }
}

impl<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>
    Traversable<'t, ReqIn, ReqOut, ReqF1> for IdHkt
{
    fn traverse<'a, A, B, F, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        _clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        _clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        f: <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
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
        F: Applicative<'t, ReqIn, ReqOut, ReqF1>,
        't: 'a,
    {
        match_one_of_5!(ReqF1::into_one_of_5_enum(f), f, f(fa))
    }
}

impl<'t> CloneOwnedK<'t, ConstBool<true>> for IdHkt {
    fn clone_owned<'a, 'b, A>(
        clone_a: impl Fn(&A) -> <ConstBool<true> as TypeGuard<'t>>::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b,
    {
        clone_a(a)
    }
}

impl<'t> CloneK<'t, ConstBool<true>> for IdHkt {
    fn clone<'a, A>(
        clone_a: impl Fn(&A) -> <ConstBool<true> as TypeGuard<'t>>::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        clone_a(a)
    }
}

impl<'t> CovariantK<'t> for IdHkt {
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        a
    }
}

impl<'t> CovariantRefK<'t> for IdHkt {
    fn covariant_ref_cast<'r, 'a, 'b, A: 'a>(a: &'r Self::F<'a, A>) -> &'r Self::F<'b, A>
    where
        'a: 'b,
        't: 'r + 'a + 'b,
    {
        a
    }
}

#[cfg(false)]
unsafe impl<'t> CovariantTransmute<'t> for IdHkt {
    fn covariant_transmute<'a, 'f, 'b, A, F: CovariantTransmute<'t>>(s: 
        F::F<'f, Self::F<'a, A>>) -> F::F<'f, Self::F<'b, A>>
    where
        A: 'a,
        'a: 'b,
        't: 'a {
        s
    }
}

// /// There should have 4 impl<'t> blocks, but we assume that only Sized transformer stacks are needed to avoid conflict with the blanket impl<'t>s.
// pub struct IdT<TInner = IdHkt>(Infallible, PhantomData<&'a TInner>);

// impl<'t><'a, TInner: Hkt> Hkt for IdT<'a, TInner> {
//     type F<A: 'a> = TInner::F<A>;
// }

// impl<'t><'a, TInner: HktUnsized> HktUnsized for IdT<'a, TInner> {
//     type FUnsized<A: 'a + ?Sized> = TInner::FUnsized<A>;
// }

// pub struct UnsizedIdT<'a, TInner = IdHkt>(Infallible, PhantomData<&'a TInner>);

// impl<'t><'a, TInner: UnsizedHkt> UnsizedHkt for UnsizedIdT<'a, TInner> {
//     type UnsizedF<A: 'a> = TInner::UnsizedF<A>;
// }

// impl<'t><'a, TInner: UnsizedHktUnsized> UnsizedHktUnsized for UnsizedIdT<'a, TInner> {
//     type UnsizedFUnsized<A: 'a + ?Sized> = TInner::UnsizedFUnsized<A>;
// }
