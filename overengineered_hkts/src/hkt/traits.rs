use core::{
    convert::Infallible,
    fmt::Debug,
    marker::PhantomData,
    ops::ControlFlow,
    panic::{RefUnwindSafe, UnwindSafe},
};

use alloc::sync::Arc;

use dyn_clone::DynClone;
use tap::Pipe as _;

use crate::{
    hkt::{
        hkt_classification::HktClassification,
        id::IdHkt,
        nullary::NullaryHkt,
        one_of::{NotT1Of5, NotT2Of5, NotT3Of5, OneOf5Hkt},
    },
    marker_classification::{AssertBlankOutput, ConstBool, TypeGuard, TyEq},
};

/// Alias for all auto traits
pub trait Marker: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe {}

impl<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe> Marker for T {}

#[derive(Debug, Clone, Copy)]
pub struct PhantomMarker<T>(PhantomData<T>);

unsafe impl<T> Send for PhantomMarker<T> {}
unsafe impl<T> Sync for PhantomMarker<T> {}
impl<T> Unpin for PhantomMarker<T> {}
impl<T> UnwindSafe for PhantomMarker<T> {}
impl<T> RefUnwindSafe for PhantomMarker<T> {}

// fn coerce_rc<'a>(a: &mut std::rc::Rc<&'a str>, x: std::rc::Rc<&'static str>) -> std::rc::Rc<&'a str> {
//    *a = x;
//    let x: std::rc::Rc<&'static str> = std::rc::Rc::new("");
//    x
// }

// fn coerce_unsafe<'a>(a: &mut std::cell::UnsafeCell<&'a str>, x: std::cell::UnsafeCell<&'static str>) -> std::cell::UnsafeCell<&'a str> {
//    // Invariant
//    *a = x;
//    let x: std::cell::UnsafeCell<&'static str> = std::cell::UnsafeCell::new("");
//    // Invariant
//    x
// }

/// A is [Sized], Self::F<'a, A> is ?[Sized]
///
/// `'t` is some arbitrary bound that always outlive every other bound in the hkt so that it can be used as a bound in some places where `'static` is the only other option
pub trait UnsizedHkt<'t>: 't {
    /// Definition: F<'a, A: 't>: 'a (where 't: 'a is logical but maybe unnecessary)
    ///
    /// Requirements:
    /// - Invariant over 'a and A and 't
    type UnsizedF<'a, A: 'a>: 'a + ?Sized
    where
        't: 'a;
}

/// TODO: Whether adding higher ranked lifetime are necessary?
/// F<'a, A: 'a>: 'a => The instance is bound by the same lifetime as the type
/// If F<'a, 'b, A: 'a>: 'b => 'a: 'b or 'a includes 'b
/// Cow<'a, A>: 'a => Cow<'static, A> then A: 'static
const _: () = {};

/// A is [Sized], Self::F<'a, A> is [Sized]
///
/// `'t` is some arbitrary bound that always outlive every other bound in the hkt so that it can be used as a bound in some places where `'static` is the only other option
pub trait Hkt<'t>: UnsizedHkt<'t> {
    type F<'a, A: 'a>: 'a + TyEq<Self::UnsizedF<'a, A>>
    where
        't: 'a;
}

// pub trait ExpHkt {
//     type F<'a, 'b, D: 'b + ExpHkt<'b>, A: 'a>: 'a
//     where
//         't: 'b,
//         'b: 'a;

//     // implicit type equality or TyEq
//     fn as_ref_newtype<'a, 'b, A: 'a>(a: &Self::F<'a, 'b, (), A>) -> &Self::F<'a, 'b, (), A>;
// }

// /// Relic
// #[cfg(false)]
// pub(crate) trait HktTransformer {
//     // Original
//     // type F<'t, TInner: Hkt>: Hkt;

//     // F, TInner: HktTransformer
//     // F::F<TInner::F<()>>::<A> != F::F<IdT::F<TInner::F<()>>>::<A> because of opaque F -> This can be proven with a conversion function
//     // Our injection must work with opaque types

//     // Equality and transformation method:
//     // A: TyEq<B>
//     // (Fa::F<A>) -> Fb::F<A>

//     // Abstract => Implement nothing
//     type F<'t, TNewtype: HktTransformer, TInner: Hkt>: Hkt;

//     // Concrete => implement something
//     fn into_concrete<'t, TNewtype: HktTransformer, TInner: Hkt>(
//         a: Self::F<'t, TNewtype, TInner>,
//     ) -> TNewtype::F<'t, Self::F<'t, (), TInner>, TInner>;
// }

// #[cfg(false)]
// pub(crate) type IdTTransformer = ();

// #[cfg(false)]
// impl HktTransformer for IdTTransformer {
//     type F<'t, TNewtype: HktTransformer, TInner: Hkt> = TInner;

//     fn into_concrete<'t, TNewtype: HktTransformer, TInner: Hkt>(
//         a: Self::F<'t, TNewtype, TInner>,
//     ) -> TNewtype::F<'t, Self::F<'t, (), TInner>, TInner> {
//         todo!()
//     }
// }

// #[cfg(false)]
// fn run<'t, F: HktTransformer, A: 'a>(
//     a: <F::F<'t, (), ()>>::F<'t, A>,
// ) -> <F::F<'t, (), ()>>::F<'t, A> {
//     fn run_s() {
//         let x = run::<(), ()>(());
//     }

//     a
// }

// #[cfg(false)]
// pub struct VecTTransformer(Infallible);

// // Vec<Arc<A>> == (Id)<Vec<(Id)<Arc<A>>>> => D<Fa<D<Fb<A>>>>
// // VecT<ArcT>::<A> == IdT<VecT<IdT<ArcT>>>::<A> => DT<VecT<DT<ArcT>>>::<A>
// // Way to say 2 stacks are transmutable
// #[cfg(false)]
// impl HktTransformer for VecTTransformer {
//     type F<'t, TNewtype: HktTransformer, TInner: Hkt> = VecT<TNewtype::F<'t, (), TInner>>;
// }

/// A is ?[Sized], Self::F<'a, A> is ?[Sized]
///
/// `'t` is some arbitrary bound that always outlive every other bound in the hkt so that it can be used as a bound in some places where `'static` is the only other option
pub trait UnsizedHktUnsized<'t>:
    // TODO:
    UnsizedHkt<'t, UnsizedF<'t, ()> = Self::UnsizedFUnsized<'t, ()>>
{
    type UnsizedFUnsized<'a, A: 'a + ?Sized>: 'a + ?Sized where 't: 'a;
}

/// A is ?[Sized]
///
/// `'t` is some arbitrary bound that always outlive every other bound in the hkt so that it can be used as a bound in some places where `'static` is the only other option
pub trait HktUnsized<'t>:
    UnsizedHktUnsized<'t>
    // TODO:
    + Hkt<'t, F<'t, ()> = Self::FUnsized<'t, ()>>
{
    type FUnsized<'a, A: 'a + ?Sized>: 'a + TyEq<Self::UnsizedFUnsized<'a, A>> where 't: 'a
    ;
}

impl<'t, T: Hkt<'t> + HktClassification<Choice = hkt_classification::OuterHkt>> UnsizedHkt<'t>
    for T
{
    type UnsizedF<'a, A: 'a>
        = T::F<'a, A>
    where
        't: 'a;
}

impl<'t, T: HktUnsized<'t> + HktClassification<Choice = hkt_classification::OuterHkt>>
    UnsizedHktUnsized<'t> for T
{
    type UnsizedFUnsized<'a, A: 'a + ?Sized>
        = T::FUnsized<'a, A>
    where
        't: 'a;
}

// pub struct TryFlowBreak<T, E> {
//     pub value: T,
//     pub error: E,
//     _phantom: PhantomData<()>
// }

// impl<'t, T, E> TryFlowBreak<T, E> {
//     pub fn new(value: T, error: E) -> Self {
//         Self { value, error, _phantom: PhantomData }
//     }
// }

// pub type TryFlow<T, E = ()> = ControlFlow<TryFlowBreak<T, E>, T>;

// pub trait IntoTryResultExt {
//     type T;
//     type E;
//     fn into_try_result(self) -> Result<Self::T, Self::E>;
// }

/// Converts enum with all variants being T into T
pub trait Converge {
    type T;
    fn converge(self) -> Self::T;
}

// impl<'t, T, E> IntoTryResultExt for TryFlow<T, E> {
//     type T = T;
//     type E = E;
//     fn into_try_result(self) -> Result<T, E> {
//         match self {
//             std::ops::ControlFlow::Continue(t) => Ok(t),
//             std::ops::ControlFlow::Break(e) => Err(e.error),
//         }
//     }
// }

// impl<'t, T, E> UnwrapEither for TryFlow<T, E> {
//     type T = T;
//     fn unwrap_either(self) -> T {
//         match self {
//             ControlFlow::Continue(t) => t,
//             ControlFlow::Break(t) => t.value,
//         }
//     }
// }

// impl<'t, T, E> IntoTryResultExt for Result<T, E> {
//     type T = T;
//     type E = E;
//     fn into_try_result(self) -> Result<T, E> {
//         self
//     }
// }

impl<T> Converge for FoldWhile<T> {
    type T = T;

    fn converge(self) -> Self::T {
        match self {
            ControlFlow::Continue(t) => t,
            ControlFlow::Break(t) => t,
        }
    }
}

impl<T> Converge for Result<T, T> {
    type T = T;
    fn converge(self) -> T {
        match self {
            Ok(t) => t,
            Err(t) => t,
        }
    }
}

// Variance:
// Hkt: A <|- B
// ReqIn: bound of the stack is the sum(?) of bounds of the whole TBD
// Does being bound makes them more or less derived?
// Invariance:
// FnOnce: Accept: { FnOnce... Fn... } CanGet { FnOnce }
// Fn: Accept: { Fn... } CanGet { FnOnce..Fn }
//

// TODO: Should our functions have this signature?
// Requirements:
// - Functions must be sharable and can be called multiple times
// - F<A> must be given a choice to own the functions (so that their lifetime may extend beyond 'a - but is that even definable?)
// - We would need Hkt over lifetimes too.
// TODO: define map_while/try_map methods => Result<F<B>, E>
// DO NOT DO THE ABOVE

// Accept FnOnce: TInner is required to also only accept FnOnce (since FnOnce is base)
//  However since type don't need any functionality from FnOnce's derived traits they can accept any input and work with the corresponding TInner
// Be careful when dealing with TInner though, e.g. if you currently has FnMut then TInner can't be Fn Only...

/// Usecases: Eager & Lazy evaluation.
///
/// `F1` usually needs cloning.
pub trait Functor<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>:
    Hkt<'t>
{
    // Clone-agnostic => clone transitivity proof:
    // If not Clone => Use cloneless
    // If Clone => Use Clone or cloneless
    // Op1: CloneFn method:
    // If not Clone => call produces trash
    // If Clone => call produce cloned
    // If unknown => match
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        f: ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
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
        't: 'a;

    // /// Perform map but short-circuits if break is returned.
    // /// What short-circuiting means depends on the implementing type:
    // /// - For eagerly evaluated types, [None] can be returned instead of an incomplete value.
    // /// - For lazily evaluated types, always return [Some()].
    // /// - If the type has a variable number of elements, it can choose to return less elements either by filtering or breaking when encountering [ControlFlow::Break]. Consult the implementing type's documentation for its exact behavior.
    // /// - Implementors may also choose to ignore [ControlFlow] and behave exactly like [Functor::map]
    // /// - If the mapping closure returns exclusively [ControlFlow::Continue] then try_map is **required** to behave exactly like map and obey [Functor] laws.
    // /// Bind Option<TInner<Option<TInner<>>>> Impossible/Maybe for types that are Foldable.
    // /// Bind TInner<Option<TInner<Option<>>>> => Option<A> + (A) => TInner<Option<B>> {
    // ///     Some(a) => f(a)
    // ///     None => TInner::pure(None)
    // /// }
    // /// Bind TInner<Future<TInner<Future<>>>> => Future<A> + (A) => TInner<Future<B>>
    // /// Future<A>.map(f) => Future<TInner<Future<B>>
    // /// Bind Future<TInner<Future<TInner>>>> => TInner<A> + (A) => Future<TInner<B>> => Future<TInner<Future<TInner>>>>
    // fn try_map<'a, 'b, A, B, F1>(f: F1, fa: Self::F<'a, A>) -> Option</* ? */ Self::F</*?*/'b, B>>
    // where
    //     A: 'a,
    //     B: 'b,
    //     F1: Fn(A) -> FoldWhile<B> + Clone;
}

// Trait bound not satisfied: somehow not exhaustive
// fn may_clone<'a, 't, const CLONE: bool, A: 'a>(a: A, a_fn: impl Fn(&A) -> <ConstBool<CLONE> as TypeGuard<'t>>::Output<'a, A>) -> (A, Option<A>) {
//     (a, None)
// }

/// `F1` usually needs cloning.
pub trait Cofunctor<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>:
    Hkt<'t>
{
    fn comap<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        f: ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        fa: Self::F<'a, B>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        B: 'a,
        F1Once: 'a + FnOnce(A) -> B,
        F1Mut: 'a + FnMut(A) -> B,
        F1Fn: 'a + Fn(A) -> B,
        F1Clone: 'a + Fn(A) -> B + Clone,
        F1Copy: 'a + Fn(A) -> B + Copy,
        't: 'a;
}

/// TODO: Need traits for transformer, transmutation is a necessity.
pub mod hkt_classification {
    use core::convert::Infallible;

    mod sealed {
        pub trait HktClassificationType {}
    }

    pub trait HktClassification {
        type Choice: sealed::HktClassificationType;
    }

    pub struct OuterHkt(Infallible);
    pub struct InnerHkt(Infallible);
    pub struct TransparentHkt(Infallible);
    pub struct OptedOutHkt(Infallible);

    use sealed::HktClassificationType;
    impl_trait_for!(HktClassificationType => OuterHkt, InnerHkt, TransparentHkt, OptedOutHkt);
}

pub trait DerefHkt<'t>: UnsizedHktUnsized<'t> {
    fn deref<'a, 'b: 'a, A: 'a + ?Sized>(fa: &'a Self::UnsizedFUnsized<'b, A>) -> &'a A
    where
        't: 'b + 'a;
}

pub trait DerefMutHkt<'t>: UnsizedHktUnsized<'t> {
    fn deref_mut<'a, 'b: 'a, A: 'a + ?Sized>(fa: &'a mut Self::UnsizedFUnsized<'b, A>) -> &'a mut A
    where
        't: 'b + 'a;
}

pub type FoldWhile<T> = ControlFlow<T, T>;

/// Eagerly evaluated, higher kinded version of [Iterator].
/// This trait can be trivial implemented for any type by just returning `init`, but that is not recommended.
///
/// `F1` usually needs cloning.
pub trait Foldable<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>, InHkt: Hkt<'t> = IdHkt>:
    Hkt<'t>
{
    fn fold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: InHkt::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b;

    /*
    fn extend<'a, 'b, 'f, A, F, E>(
        in_requirement: ReqIn::F<'a, A>,
        // TODO: Fix lifetime?
        out_requirement: ReqOut::F<'a, F>,
        tag: ReqF1::OneOf5F<'f, impl Sized, impl Sized, impl Sized, impl Sized, impl Sized>,
        collection: F,
        fa: Self::F<'a, A>,
    ) -> F
    where
        A: 'a,
        F: 'b + DerefMut<Target = E>,
        E: 'b + Extend<A>,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b,
    {
        let tag = ReqF1::create_from(&tag, |mut sum: F, a: A| {
            sum.extend(std::iter::once(a));
            ControlFlow::Continue(sum)
        });

        Self::fold_while(in_requirement, out_requirement, tag, collection, fa).unwrap_either()
    }
     */

    /// Extend a collection implementing [Extend] with elements of this [Foldable].
    fn extend<'a, 'e, 'f, A, E>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        tag: ReqF1::OneOf5F<'f, impl Sized, impl Sized, impl Sized, impl Sized, impl Sized>,
        collection: &'e mut E,
        fa: InHkt::F<'a, Self::F<'a, A>>,
    ) -> &'e mut E
    where
        Self: Foldable<'t, ReqIn, ConstBool<false>, ReqF1, InHkt>,
        A: 'a,
        E: 'e + Extend<InHkt::F<'a, A>>,
        'a: 'f,
        'e: 'f,
        't: 'a + 'e,
    {
        let tag = ReqF1::create_from(&tag, |sum: &'e mut E, a: InHkt::F<'a, A>| {
            sum.extend(core::iter::once(a));
            FoldWhile::Continue(sum)
        });

        <Self as Foldable<'t, ReqIn, ConstBool<false>, ReqF1, InHkt>>::fold_while::<
            A,
            &'e mut E,
            _,
            _,
            _,
            _,
            _,
        >(clone_a, |_| AssertBlankOutput, tag, collection, fa)
        .converge()
    }

    /// Hkt version of [Iterator::size_hint]
    #[allow(unused_variables)]
    fn size_hint<'a, A>(s: &Self::F<'a, A>) -> (usize, Option<usize>)
    where
        't: 'a,
    {
        (0, None)
    }
}

/// `F1` usually needs cloning.
pub trait Rfoldable<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>, InHkt: Hkt<'t> = IdHkt>:
    Hkt<'t>
{
    fn rfold_while<'a, 'b, 'f, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'f + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'f + Fn(&B) -> ReqOut::Output<'b, B> + Clone,
        f: ReqF1::OneOf5F<'f, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
        init: B,
        fa: InHkt::F<'a, Self::F<'a, A>>,
    ) -> FoldWhile<B>
    where
        A: 'a,
        B: 'b,
        F1Once: 'f + FnOnce(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Mut: 'f + FnMut(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Fn: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B>,
        F1Clone: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Clone,
        F1Copy: 'f + Fn(B, InHkt::F<'a, A>) -> FoldWhile<B> + Copy,
        'a: 'f,
        'b: 'f,
        't: 'a + 'b;
}

pub trait Pure<'t, ReqIn: TypeGuard<'t>>: Hkt<'t> {
    fn pure<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: A,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a;
}

/// `A` and `F1` usually needs cloning.
pub trait Applicative<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>:
    Functor<'t, ReqIn, ReqOut, ReqF1> + Pure<'t, ReqIn>
{
    fn apply<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        ff: Self::F<'a, ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>>,
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
        't: 'a;
}

/// `B` and `F1` usually needs cloning.
pub trait Monad<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>:
    Applicative<'t, ReqIn, ReqOut, ReqF1>
{
    fn bind<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        clone_b: impl 'a + Fn(&B) -> ReqOut::Output<'a, B> + Clone,
        fa: Self::F<'a, A>,
        f: ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
    ) -> Self::F<'a, B>
    where
        A: 'a,
        B: 'a,
        F1Once: 'a + FnOnce(A) -> Self::F<'a, B>,
        F1Mut: 'a + FnMut(A) -> Self::F<'a, B>,
        F1Fn: 'a + Fn(A) -> Self::F<'a, B>,
        F1Clone: 'a + Fn(A) -> Self::F<'a, B> + Clone,
        F1Copy: 'a + Fn(A) -> Self::F<'a, B> + Copy,
        't: 'a;
}

#[allow(unused)]
/// TODO
pub(crate) trait MonadT<
    't,
    M: Monad<'t, ReqIn, ReqOut, ReqF1>,
    ReqIn: TypeGuard<'t>,
    ReqOut: TypeGuard<'t>,
    ReqF1: OneOf5Hkt<'t>,
>: Monad<'t, ReqIn, ReqOut, ReqF1>
{
    fn lift<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        ma: M::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a;
}

/// `B`, `F1` and `F` usually needs cloning.
pub trait Traversable<'t, ReqIn: TypeGuard<'t>, ReqOut: TypeGuard<'t>, ReqF1: OneOf5Hkt<'t>>:
    Functor<'t, ReqIn, ReqOut, ReqF1> + Foldable<'t, ReqIn, ReqOut, ReqF1>
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
        't: 'a;
}

#[cfg(false)]
pub(crate) trait SemigroupK<'t, Req: TypeGuard<'t>>: Hkt<'t> {
    fn combine<'a, A: 'a>(
        a: Self::F<'a, A>,
        b: Self::F<'a, A>,
        clone_a: impl 'a + Fn(&A) -> Req::Output<'a, A> + Clone
    ) -> Self::F<'a, A>
    where
        't: 'a;
}

// /// TODO
// pub(crate) trait ChoiceK<'t>: Applicative<'t> + SemigroupK<'t> {
//     fn choose<'a, A: 'a>(a: Self::F<'a, A>, b: Self::F<'a, A>) -> Self::F<'a, A>
//     where
//         't: 'a;
// }

// /// TODO
// pub(crate) trait AlternativeK<'t>: ChoiceK<'t> + MonoidK<'t> {}

#[allow(unused)]
/// TODO
pub(crate) trait DefaultK<'t>: Hkt<'t> {
    fn default<'a, A: 'a>() -> Self::F<'a, A>
    where
        't: 'a;
}

#[allow(unused)]
#[cfg(false)]
/// TODO
pub(crate) trait MonoidK<'t>: SemigroupK<'t> + DefaultK<'t> {}

#[cfg(false)]
impl<'t, K: SemigroupK + DefaultK> MonoidK for K {}

// pub trait DependentDuplicateK: Hkt {
//     fn duplicate<'a, A: 'a + Clone>(a: Self::F<'a, A>) -> Cycle<impl Iterator<Item = Self::F<'a, A>>> {
//         Self::clone_f::<(), _>(a)
//     }

//     fn clone_f<'a, F: DependentCloneK, A: 'a + Clone>(a: &Self::F<'a, F::F<'t, A>>) -> Self::F<'a, F::F<'t, A>>;
// }

// pub trait TransformK<'t, TIn: Hkt<'t>, TOut: Hkt<'t>, E>: Hkt<'t> {
//     fn transform<'a, 'b, A: 'a>(a: TIn::F<'a, Self::F<'a, A>>) -> TOut::F<'b, Self::F<'b, A>>
//     where
//         E: EncodeLifetime<'a, 'b>;
// }

// pub trait EncodeLifetime<'a, 'b> {}
// pub struct EquivalentLifetime(Infallible);
// impl<'a> EncodeLifetime<'a, 'a> for EquivalentLifetime {}

// pub struct SeperateLifetime(Infallible);
// impl<'a, 'b> EncodeLifetime<'a, 'b> for SeperateLifetime {}

// impl<'t> TransformK<'t, (), (), EquivalentLifetime> for CowT {
//     fn transform<'a, 'b, A: 'a>(a: <() as Hkt<'t>>::F<'a, Self::F<'a, A>>) -> <() as Hkt<'t>>::F<'b, Self::F<'b, A>>
//     where EquivalentLifetime: EncodeLifetime<'a, 'b> {
//         a
//     }
// }

/// Convert reference into ownership with the same lifetime bound:
/// - The bound could be from a reference to some shared resource
/// - Other owned data could be cloned, including A
pub trait CloneK<
    't,
    ReqIn: TypeGuard<'t> = ConstBool<false>,
    Output: TypeGuard<'t> = ConstBool<true>,
>: Hkt<'t>
{
    /// = (&Self::F<'a, A>) => Self::F<'a, A> (Clone) + (Self::F<'a, A>) => Self::F<'t, A> (IntoOwned) + (Self::F<'t, A>) => Self::F<'a, A> (Covariance)
    fn clone<'a, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a;
}

// TODO
#[cfg(false)]
impl<'t, ReqIn: TypeGuard<'t>, T: Hkt<'t>> CloneK<'t, ReqIn, ConstBool<false>> for T {}

/// Relic
// pub trait CloneApplicativeFn<'t, ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>>: Hkt<'t> {
//     fn clone_applicative_func<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
//         fa: &Self::F<'a, ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>>,
//     ) -> Self::F<'a, ReqF1::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>>
//     where
//         A: 'a,
//         B: 'a,
//         F1Once: 'a + FnOnce(A) -> B,
//         F1Mut: 'a + FnMut(A) -> B,
//         F1Fn: 'a + Fn(A) -> B,
//         F1Clone: 'a + Fn(A) -> B + Clone,
//         F1Copy: 'a + Fn(A) -> B + Copy,
//         't: 'a;
// }

#[cfg(feature = "unstable")]
/// Similar to [CloneK] but can be set to arbitrary new lifetime
pub trait CloneOwnedK<
    't,
    ReqIn: TypeGuard<'t> = ConstBool<false>,
    Output: TypeGuard<'t> = ConstBool<true>,
>: Hkt<'t>
{
    fn clone_owned<'a, 'b, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b;
}

#[allow(unused)]
#[cfg(not(feature = "unstable"))]
/// Similar to [CloneK] but can be set to arbitrary new lifetime
pub(crate) trait CloneOwnedK<
    't,
    ReqIn: TypeGuard<'t> = ConstBool<false>,
    Output: TypeGuard<'t> = ConstBool<true>,
>: Hkt<'t>
{
    fn clone_owned<'a, 'b, A>(
        clone_a: impl 'a + Fn(&A) -> ReqIn::Output<'b, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a + 'b,
        't: 'a + 'b;
}

// TODO
#[cfg(false)]
impl<'t, ReqIn: TypeGuard<'t>, T: Hkt<'t>> CloneOwnedK<'t, ReqIn, ConstBool<false>> for T {}

/// A trait alias to declare a better intention that the variant must be cloneable - along with a helper function.
pub trait TCloneableOf5<'t>: NotT1Of5<'t> + NotT2Of5<'t> + NotT3Of5<'t> {
    /// Helper function to eliminate uncloneable variants.
    fn arbitrary_uncloneable<'a, T1a, T2a, T3a, T4a, T5a, Tb>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<Tb>,
    ) -> Self::OneOf5F<'a, Tb, Tb, Tb, T4a, T5a>
    where
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        Tb: 'a,
        't: 'a,
    {
        s.pipe(|s| Self::arbitrary_t1(s, PhantomData::<Tb>))
            .pipe(|s| Self::arbitrary_t2(s, PhantomData::<Tb>))
            .pipe(|s| Self::arbitrary_t3(s, PhantomData::<Tb>))
    }
}

impl<'t, T: NotT1Of5<'t> + NotT2Of5<'t> + NotT3Of5<'t>> TCloneableOf5<'t> for T {}

#[deprecated = "Unused"]
#[allow(unused)]
pub(crate) trait PureMapInner<'t, ReqIn: Hkt<'t> = NullaryHkt, F: Hkt<'t> = IdHkt>:
    Hkt<'t>
{
    fn pure_map_inner<'a, A>(
        in_requirement: ReqIn::F<'a, A>,
        s: Self::F<'a, A>,
    ) -> Self::F<'a, F::F<'a, A>>
    where
        A: 'a,
        't: 'a;
}

pub(crate) trait CovariantK<'t>: Hkt<'t> {
    /// &'a TInner<'a, A> => &'b TInner<'b, A>/
    /// A stack can have as many lifetimes as layers
    /// Safe but only works on owned types
    fn covariant_convert<'a, 'b, A>(a: Self::F<'a, A>) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b;
}

#[deprecated = "Unused"]
#[allow(unused)]
pub(crate) trait CovariantRefK<'t>: CovariantK<'t> {
    fn covariant_ref_cast<'r, 'a, 'b, A: 'a>(a: &'r Self::F<'a, A>) -> &'r Self::F<'b, A>
    where
        'a: 'b,
        't: 'r + 'a + 'b;
}

#[cfg(false)]
/// # Safety
/// Only implement this for types that are both covariant in itself and the inner type. See https://doc.rust-lang.org/reference/subtyping.html#variance.
pub unsafe trait CovariantTransmute<'t>: Hkt<'t> {
    fn covariant_transmute<'a, 'f, 'b, A, F: CovariantTransmute<'t>>(s: 
        F::F<'f, Self::F<'a, A>>) -> F::F<'f, Self::F<'b, A>>
    where
        A: 'a,
        'a: 'b,
        't: 'a; 
} 

#[deprecated = "Unused"]
#[allow(unused)]
pub(crate) trait CloneFnHkt<'t>: Hkt<'t> {
    fn call_clone<'a, A: 'a>(f: &Self::F<'a, A>, a: &A) -> A
    where
        't: 'a;
}

#[deprecated = "Unused"]
#[allow(unused)]
pub(crate) struct ClonePtrHkt(Infallible);

impl<'t> Hkt<'t> for ClonePtrHkt {
    type F<'a, A: 'a>
        = fn(&A) -> A
    where
        't: 'a;
}

impl HktClassification for ClonePtrHkt {
    type Choice = hkt_classification::OuterHkt;
}

impl<'t, ReqIn: TypeGuard<'t>> CloneK<'t, ReqIn> for ClonePtrHkt {
    fn clone<'a, A>(
        _clone_a: impl Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        *a
    }
}

impl<'t> CloneFnHkt<'t> for ClonePtrHkt {
    fn call_clone<'a, A: 'a>(f: &Self::F<'a, A>, a: &A) -> A
    where
        't: 'a,
    {
        f(a)
    }
}

pub struct DynCloneFnHkt(Infallible);

pub trait DynCloneFn<A>: Fn(&A) -> A + DynClone {}

impl<A, T: Fn(&A) -> A + Clone> DynCloneFn<A> for T {}

impl<'t> Hkt<'t> for DynCloneFnHkt {
    type F<'a, A: 'a>
        = Arc<dyn 'a + DynCloneFn<A>>
    where
        't: 'a;
}

impl HktClassification for DynCloneFnHkt {
    type Choice = hkt_classification::OuterHkt;
}

impl<'t, ReqIn: TypeGuard<'t>> CloneK<'t, ReqIn> for DynCloneFnHkt {
    fn clone<'a, A>(
        _clone_a: impl Fn(&A) -> <ReqIn>::Output<'a, A> + Clone,
        a: &Self::F<'a, A>,
    ) -> Self::F<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        Arc::clone(a)
    }
}

/// Needs Deref & Pure: ((&A) -> A) -> (&F<A>) -> F<A>
#[cfg(false)]
impl<'t, ReqIn: Hkt<'t> + CloneK<'t>, F: DerefHkt<'t> + Pure<'t, ReqIn>> PureMapInner<'t, ReqIn, F>
    for DynCloneFnHkt
{
    fn pure_map_inner<'a, A>(
        in_requirement: <ReqIn>::F<'a, A>,
        s: Self::F<'a, A>,
    ) -> Self::F<'a, <F>::F<'a, A>>
    where
        A: 'a,
        't: 'a,
    {
        let requirements = CloneWrapper(in_requirement, |a: &_| ReqIn::clone((), a));

        Arc::new(move |a: &<F as Hkt<'t>>::F<'a, A>| {
            a.pipe(|a| unsafe {
                unsafe_transmute_id::<&<F as UnsizedHktUnsized<'t>>::UnsizedFUnsized<'a, A>, _>(a)
            })
            .pipe(|a| F::deref::<A>(a))
            .pipe(|a| s.clone()(a))
            .pipe(|a| F::pure(requirements.clone().0, a))
        })
    }
}

impl<'t> CloneFnHkt<'t> for DynCloneFnHkt {
    fn call_clone<'a, A: 'a>(f: &Self::F<'a, A>, a: &A) -> A
    where
        't: 'a,
    {
        f(a)
    }
}

#[allow(unused)]
/// Example type
pub(crate) struct CowT<K = IdHkt>(Infallible, PhantomData<K>);

#[allow(unused)]
pub(crate) enum Borrown<'a, T> {
    Borrow(&'a T),
    Own(T),
}

impl<'t, K: Hkt<'t>> Hkt<'t> for CowT<K> {
    type F<'a, A: 'a>
        = Borrown<'a, K::F<'a, A>>
    where
        't: 'a;
}

impl<K> HktClassification for CowT<K> {
    type Choice = hkt_classification::OuterHkt;
}

// impl<'t, K: CovariantRefK<'t>> CovariantK<'t> for CowT<K> {
//     fn covariant_convert<'a, 'b, A: 'a>(a: Self::F<'a, A>) -> Self::F<'b, A>
//     where
//         'a: 'b,
//         't: 'a + 'b,
//     {
//         match a {
//             Borrown::Borrow(k) => Borrown::Borrow(K::covariant_ref_cast(k)),
//             Borrown::Own(k) => Borrown::Own(K::covariant_convert(k)),
//         }
//     }
// }

#[allow(unused)]
pub(crate) trait IntoIteratorHkt<'t>: Hkt<'t> {
    fn into_iter<'a, A: 'a>(iter: Self::F<'a, A>) -> impl Iterator<Item = A>;
}
