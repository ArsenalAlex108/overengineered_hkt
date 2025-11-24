use core::{
    convert::Infallible,
    fmt::Debug,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{hkt::{
    Hkt, HktUnsized, PhantomMarker, UnsizedHkt, hkt_classification::{self, HktClassification}, id::IdHkt, nullary::NullaryHkt
}, transmute};

mod sealed {
    use core::convert::Infallible;

    /// Sealed base trait
    pub trait BaseSealed {}
    // pub trait SendTypeChoice {}

    /// Sealed associated type base trait
    pub trait AssociatedSealed {}

    /// Key for [AssociatedSealed] - which itself is sealed.
    pub struct AssociatedKey(Infallible);
    impl AssociatedSealed for AssociatedKey {}

    pub trait TyEqSealed {
        type T: ?Sized;
    }

    impl<T: ?Sized> TyEqSealed for T {
        type T = T;
    }
}

pub trait TyEq<T: ?Sized>: sealed::TyEqSealed<T = T> {
    fn transmute_hkt_into<'a, 't, F: HktUnsized<'t>>(a: F::FUnsized<'a, Self>) -> F::FUnsized<'a, T> {
        // SAFETY: `Self` and `T` are the same type,
        // therefore F::FUnsized<'a, Self> and F::FUnsized<'a, T>
        // are the same type, hence transmutation between the two
        // are always safe.
        unsafe { transmute::transmute_unchecked(a) }
    }

    fn transmute_hkt_from<'a, 't, F: HktUnsized<'t>>(a: F::FUnsized<'a, T>) -> F::FUnsized<'a ,Self> {
        // SAFETY: `Self` and `T` are the same type,
        // therefore F::FUnsized<'a, Self> and F::FUnsized<'a, T>
        // are the same type, hence transmutation between the two
        // are always safe.
        unsafe { transmute::transmute_unchecked(a) }
    }
}

impl<T: ?Sized> TyEq<T> for T {}

#[allow(unused)]
/// Implemented by [`ConstBool<true>`](ConstBool) and [`ConstBool<false>`](ConstBool)
pub(crate) trait IsConstBool: sealed::BaseSealed {
    const BOOL: bool;
}

#[derive(Debug, Default, Clone, Copy)]
#[must_use = "Unexpected attempt to call no-op function with blank return type."]
/// Used to replace/block the output of a function to signal that the output type is intentionally empty and calling the function is a no-op. Used as associated types in [TypeGuard] for [`ConstBool<false>`]
pub struct AssertBlankOutput;

/// Error returned calling [TypeGuard::try_create_guarded] when [TypeGuard] is [`ConstBool<true>`]
#[derive(Debug, Default, Clone, Copy)]
pub struct TypeGuardWasConstBoolTrue;

pub struct ConstBool<const VAL: bool>;

impl<const VAL: bool> sealed::BaseSealed for ConstBool<VAL> {}
impl<const VAL: bool> IsConstBool for ConstBool<VAL> {
    const BOOL: bool = VAL;
}

#[allow(unused)]
pub(crate) trait IsSend<Bool: IsConstBool> {
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

#[allow(unused)]
pub(crate) trait IsSync<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsSync<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: Sync> IsSync<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

// pub trait IsClone<Bool: IsConstBool + TypeGuardK> {
//     type Sealed: sealed::AssociatedSealed;

//     fn protected_clone<'t>(a: &Self) -> <<Bool as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
//     where
//         Self: Sized;
// }

#[allow(unused)]
/// (Unused) Higher-kinded version of [TypeGuard]
pub(crate) trait TypeGuardK: sealed::BaseSealed {
    type OutputF<'t>: Hkt<'t>;
}

impl TypeGuardK for ConstBool<false> {
    type OutputF<'t> = NullaryHkt<AssertBlankOutput>;
}

impl TypeGuardK for ConstBool<true> {
    type OutputF<'t> = IdHkt;
}

/// Used to guard output type at type level. Has the same definition as [Hkt] but is sealed and implemented exclusively for [ConstBool].
pub trait TypeGuard<'t>: sealed::BaseSealed {
    type Output<'a, A: 'a>: 'a
    where
        't: 'a;
    type Err<'a>: 'a
    where
        't: 'a;

    /// Match with some runtime overhead due to enum - but at least no unsafe needed.
    fn match_guard<'a, A>(guarded_val: Self::Output<'a, A>) -> Result<A, Self::Err<'a>>
    where
        A: 'a,
        't: 'a;

    /// Converts a value into a guarded output.
    fn into_guarded<'a, A>(val: A) -> Self::Output<'a, A>
    where
        A: 'a,
        't: 'a;

    fn try_create_guarded<'a, A>() -> Result<Self::Output<'a, A>, TypeGuardWasConstBoolTrue>
    where
        A: 'a,
        't: 'a;
}

impl<'t> TypeGuard<'t> for ConstBool<false> {
    type Output<'a, A: 'a>
        = AssertBlankOutput
    where
        't: 'a;

    type Err<'a>
        = AssertBlankOutput
    where
        't: 'a;

    fn match_guard<'a, A>(guarded_val: Self::Output<'a, A>) -> Result<A, Self::Err<'a>>
    where
        A: 'a,
        't: 'a,
    {
        Err(guarded_val)
    }

    fn into_guarded<'a, A>(val: A) -> Self::Output<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        AssertBlankOutput
    }

    fn try_create_guarded<'a, A>() -> Result<Self::Output<'a, A>, TypeGuardWasConstBoolTrue>
    where
        A: 'a,
        't: 'a,
    {
        Ok(AssertBlankOutput)
    }
}

impl<'t> TypeGuard<'t> for ConstBool<true> {
    type Output<'a, A: 'a>
        = A
    where
        't: 'a;

    type Err<'a>
        = Infallible
    where
        't: 'a;

    fn match_guard<'a, A>(guarded_val: Self::Output<'a, A>) -> Result<A, Self::Err<'a>>
    where
        A: 'a,
        't: 'a,
    {
        Ok(guarded_val)
    }

    fn into_guarded<'a, A>(val: A) -> Self::Output<'a, A>
    where
        A: 'a,
        't: 'a,
    {
        val
    }

    fn try_create_guarded<'a, A>() -> Result<Self::Output<'a, A>, TypeGuardWasConstBoolTrue>
    where
        A: 'a,
        't: 'a,
    {
        Err(TypeGuardWasConstBoolTrue)
    }
}

// impl<T> IsClone<ConstBool<false>> for T {
//     type Sealed = sealed::AssociatedKey;

//     #[inline]
//     #[cold]
//     fn protected_clone<'t>(
//         a: &Self,
//     ) -> <<ConstBool<false> as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
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
//     ) -> <<ConstBool<true> as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Self>
//     where
//         Self: Sized,
//     {
//         a.clone()
//     }
// }

#[allow(unused)]
#[deprecated = "Unused"]
pub(crate) trait IsUnpin<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsUnpin<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: Unpin> IsUnpin<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

#[allow(unused)]
#[deprecated = "Unused"]
pub(crate) trait IsUnwindSafe<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsUnwindSafe<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: UnwindSafe> IsUnwindSafe<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

#[allow(unused)]
#[deprecated = "Unused"]
pub(crate) trait IsRefUnwindSafe<Bool: IsConstBool> {
    type Sealed: sealed::AssociatedSealed;
}

impl<T> IsRefUnwindSafe<ConstBool<false>> for T {
    type Sealed = sealed::AssociatedKey;
}
impl<T: RefUnwindSafe> IsRefUnwindSafe<ConstBool<true>> for T {
    type Sealed = sealed::AssociatedKey;
}

// pub trait IsDebug<Bool: IsConstBool + TypeGuardK> {
//     type Sealed: sealed::AssociatedSealed;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<Bool as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>>;
// }

// impl<T> IsDebug<ConstBool<false>> for T {
//     type Sealed = sealed::AssociatedKey;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<ConstBool<false> as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>> {
//         MissingBoundAtCompileTimeError
//     }
// }

// impl<T: std::fmt::Debug> IsDebug<ConstBool<true>> for T {
//     type Sealed = sealed::AssociatedKey;

//     fn as_debug<'t>(
//         &'t self,
//     ) -> <<ConstBool<true> as TypeGuardK>::OutputF<'t> as Hkt<'t>>::F<'t, Wrapper<&'t Self>> {
//         Wrapper(self)
//     }
// }

#[deprecated = "Unused"]
pub(crate) struct Wrapper<T>(T);

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
pub(crate) trait TraitRequirements {
    type IsSend: IsConstBool;
    type IsSync: IsConstBool;
    type IsUnpin: IsConstBool;
    type IsUnwindSafe: IsConstBool;
    type IsRefUnwindSafe: IsConstBool;
}

#[deprecated = "Unused"]
/// Compound trait to simplify bounds
pub(crate) trait TraitRequirementsTarget<Req: TraitRequirements>:
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

#[deprecated = "Unused"]
/// Marker type to signal that none of [TraitRequirements] are required.
pub(crate) struct RequireNone<ActualHkt = NullaryHkt>(Infallible, PhantomMarker<ActualHkt>);

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

#[allow(unused)]
#[repr(transparent)]
#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct UnsafeAuto<T>(T);

#[allow(unused)]
impl<T> UnsafeAuto<T> {
    pub(crate) unsafe fn assert_all_safe(t: T) -> Self {
        Self(t)
    }

    pub(crate) fn assert_markers_implemented(
        _: impl Send + Sync + Unpin + UnwindSafe + RefUnwindSafe,
    ) {
    }

    pub(crate) fn into_inner(self) -> T {
        self.0
    }
}

unsafe impl<T> Send for UnsafeAuto<T> {}
unsafe impl<T> Sync for UnsafeAuto<T> {}
impl<T> Unpin for UnsafeAuto<T> {}
impl<T> UnwindSafe for UnsafeAuto<T> {}
impl<T> RefUnwindSafe for UnsafeAuto<T> {}
