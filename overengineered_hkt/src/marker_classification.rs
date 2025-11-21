use std::{
    convert::Infallible,
    fmt::Debug,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::hkt::{
    Hkt, PhantomMarker, UnsizedHkt,
    hkt_classification::{self, HktClassification},
    id::IdHkt,
    nullary::NullaryHkt,
};

mod sealed {
    use std::convert::Infallible;

    /// Sealed base trait
    pub trait BaseSealed {}
    // pub trait SendTypeChoice {}

    /// Sealed associated type base trait
    pub trait AssociatedSealed {}

    /// Key for [AssociatedSealed] - which itself is sealed.
    pub struct AssociatedKey(Infallible);
    impl AssociatedSealed for AssociatedKey {}
}

/// Implemented by [`ConstBool<true>`](ConstBool) and [`ConstBool<false>`](ConstBool)
pub trait IsConstBool: sealed::BaseSealed {
    const BOOL: bool;
}

#[derive(Debug)]
#[must_use = "Unexpected attempt to call function with missing bounds on target type"]
/// Used to block replace the output of a function to signal that the called type is missing bounds needed to call this function.
pub struct MissingBoundAtCompileTimeError;

pub struct ConstBool<const VAL: bool>;

impl<const VAL: bool> sealed::BaseSealed for ConstBool<VAL> {}
impl<const VAL: bool> IsConstBool for ConstBool<VAL> {
    const BOOL: bool = VAL;
}

pub trait IsSend<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

// pub struct IsSend(Infallible);
// pub struct MayNotSend(Infallible);
// impl_trait_for!(sealed::SendTypeChoice => IsSend, MayNotSend);

impl<T> IsSend<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: Send> IsSend<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

pub trait IsSync<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsSync<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: Sync> IsSync<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

// pub trait IsClone<Bool: IsConstBool + TypeGuard> {
//     type Sealed: sealed::AssociatedSealed;

//     fn protected_clone<'t>(a: &Self) -> <<Bool as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
//     where
//         Self: Sized;
// }

#[deprecated = "Unused for now."]
pub trait TypeGuard: sealed::BaseSealed {
    type OutputF<'t>: Hkt<'t>;
}

impl TypeGuard for ConstBool<false> {
    type OutputF<'t> = NullaryHkt<MissingBoundAtCompileTimeError>;
}

impl TypeGuard for ConstBool<true> {
    type OutputF<'t> = IdHkt;
}

// impl<T> IsClone<ConstBool<false>> for T {
//     type Sealed = sealed::AssociatedKey;

//     #[inline]
//     #[cold]
//     fn protected_clone<'t>(
//         a: &Self,
//     ) -> <<ConstBool<false> as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
//     where
//         Self: Sized,
//     {
//         MissingBoundAtCompileTimeError
//     }
// }

// impl<T: Clone> IsClone<ConstBool<true>> for T {
//     type Sealed = sealed::AssociatedKey;

//     fn protected_clone<'t>(
//         a: &Self,
//     ) -> <<ConstBool<true> as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
//     where
//         Self: Sized,
//     {
//         a.clone()
//     }
// }

pub trait IsUnpin<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsUnpin<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: Unpin> IsUnpin<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

pub trait IsUnwindSafe<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsUnwindSafe<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: UnwindSafe> IsUnwindSafe<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

pub trait IsRefUnwindSafe<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsRefUnwindSafe<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: RefUnwindSafe> IsRefUnwindSafe<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

// pub trait IsDebug<Bool: IsConstBool + TypeGuard> {
//     type Sealed: sealed::AssociatedSealed;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<Bool as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>>;
// }

// impl<T> IsDebug<ConstBool<false>> for T {
//     type Sealed = sealed::AssociatedKey;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<ConstBool<false> as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>> {
//         MissingBoundAtCompileTimeError
//     }
// }

// impl<T: std::fmt::Debug> IsDebug<ConstBool<true>> for T {
//     type Sealed = sealed::AssociatedKey;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<ConstBool<true> as TypeGuard>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>> {
//         Wrapper(self)
//     }
// }

pub struct Wrapper<T>(T);

// impl<T: IsDebug<ConstBool<true>>> Debug for Wrapper<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("Wrapper").field(&self.0).finish()
//     }
// }

// impl<T: IsClone<ConstBool<true>>> Clone for Wrapper<T> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

/// SAFETY: IsX<true> is only implemented for T: X
unsafe impl<T: IsSend<ConstBool<true>>> Send for Wrapper<T> {}
unsafe impl<T: IsSync<ConstBool<true>>> Sync for Wrapper<T> {}
impl<T: IsUnpin<ConstBool<true>>> Unpin for Wrapper<T> {}
impl<T: IsUnwindSafe<ConstBool<true>>> UnwindSafe for Wrapper<T> {}
impl<T: IsRefUnwindSafe<ConstBool<true>>> RefUnwindSafe for Wrapper<T> {}

#[deprecated = "The compiler cannot prove exhaustiveness"]
/// Trait listing whether a type is required to implement [Clone], [Debug](std::fmt::Debug) and other marker traits.
pub trait TraitRequirements {
    type IsSend: IsConstBool;
    type IsSync: IsConstBool;
    type IsUnpin: IsConstBool;
    type IsUnwindSafe: IsConstBool;
    type IsRefUnwindSafe: IsConstBool;
}

/// Compound trait to simplify bounds
pub trait TraitRequirementsTarget<Req: TraitRequirements>:
    IsSend<Req::IsSend>
    + IsSync<Req::IsSync>
    + IsUnpin<Req::IsUnpin>
    + IsRefUnwindSafe<Req::IsRefUnwindSafe>
{
}

impl<
    Req: TraitRequirements,
    T: IsSend<Req::IsSend>
        + IsSync<Req::IsSync>
        + IsUnpin<Req::IsUnpin>
        + IsRefUnwindSafe<Req::IsRefUnwindSafe>,
> TraitRequirementsTarget<Req> for T
{
}

/// Marker type to signal that none of [TraitRequirements] are required.
pub struct RequireNone<ActualHkt = NullaryHkt>(Infallible, PhantomMarker<ActualHkt>);

impl<K> TraitRequirements for RequireNone<K> {

    type IsSend = ConstBool<false>;

    type IsSync = ConstBool<false>;

    type IsUnpin = ConstBool<false>;

    type IsUnwindSafe = ConstBool<false>;

    type IsRefUnwindSafe = ConstBool<false>;
}

impl<K> HktClassification for RequireNone<K> {
    type Choice = hkt_classification::TransparentHkt;
}

impl<'t, K: UnsizedHkt<'t>> UnsizedHkt<'t> for RequireNone<K> {
    type UnsizedF<'a, A: 'a>
        = K::UnsizedF<'a, A>
    where
        't: 'a;
}

impl<'t, K: Hkt<'t>> Hkt<'t> for RequireNone<K> {
    type F<'a, A: 'a>
        = K::F<'a, A>
    where
        't: 'a;
}

#[repr(transparent)]
#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct UnsafeAuto<T>(T);

impl<T> UnsafeAuto<T> {
    pub(crate) unsafe fn assert_all_safe(t: T) -> Self {
        Self(t)
    }

    pub(crate) fn assert_markers_implemented(_: impl Send + Sync + Unpin + UnwindSafe + RefUnwindSafe) {}

    pub(crate) fn into_inner(self) -> T {
        self.0
    }
}

unsafe impl<T> Send for UnsafeAuto<T> {}
unsafe impl<T> Sync for UnsafeAuto<T> {}
impl<T> Unpin for UnsafeAuto<T> {}
impl<T> UnwindSafe for UnsafeAuto<T> {}
impl<T> RefUnwindSafe for UnsafeAuto<T> {}
