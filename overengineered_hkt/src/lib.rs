use std::{
    convert::Infallible,
    fmt::{Debug, Error},
    marker::PhantomData,
    panic::RefUnwindSafe,
    sync::Arc,
};

use tap::{Conv, Pipe as _};

use crate::{dyn_sealed::DynTyEqSealed, hkt::HktUnsized};

#[macro_use]
mod macros;

pub mod hkt;
pub mod marker_classification;
#[cfg(feature = "transmute")]
pub mod transmute;
#[cfg(not(feature = "transmute"))]
pub(crate) mod transmute;
pub(crate) mod utils;

/// Re-export of [either::Either]
pub use either::Either;

struct Foo;

#[::negative::negative_impl]
impl !Unpin for Foo {}

trait Demo {}

impl<T: Unpin> Demo for T {}

impl Demo for Foo {} // <- does not overlap!

pub struct UnitTarget;

// #[derive(Debug, Clone, Copy, Default)]
// struct PhantomData<T>([T;0]);

#[derive(Clone, Default)]
pub struct Mod<T>(PhantomData<Arc<T>>);

//impl PubTrait for Mod<UnitTarget> {}

// unsafe impl<T: Debug> Send for Mod<T> {}

trait IAmNeverImplemented {}

// This never works because where clause is never satisfied.
// #[::negative::negative_impl]
// unsafe impl<T: IAmNeverImplemented> !Send for Mod<T> {}

// This implements !Send for every T.
#[::negative::negative_impl]
unsafe impl !Send for Mod<UnitTarget> {}

// const _: () = {
//     pub struct __Wrapper<'a, T: ?Sized>(::core::marker::PhantomData<&'a ()>, T);
//     unsafe impl<T: ?Sized> ::core::marker::Send for __Wrapper<'_, T>
//     where
//         T: ::core::marker::Send,
//     {}
//     #[allow(clippy::non_send_fields_in_send_ty)]
//     #[doc(hidden)]
//     unsafe impl<'__wrapper> ::core::marker::Send for Mod<UnitTarget>
//     where
//         for<'a> __Wrapper<'__wrapper, *const ()>: ::core::marker::Send,
//     {}
// };

// unsafe impl Send for Mod<UnitTarget>
// where for <'a> [()] : Sealed
// {

// }

// trait ModMarker {}
// impl ModMarker for Mod<UnitTarget> {}
// impl<T: RefUnwindSafe> ModMarker for Mod<T> {}

// Doesn't work:
// trait C {}
// impl C for ()
// where for<'a> [()]: Sized {}
// impl<T> C for T {}

// #[::negative::negative_impl]
// impl !RefUnwindSafe for UnitTarget {}

pub trait Marker {}

impl<T> Marker for T where Mod<T>: RefUnwindSafe {}

// Does not work - The interface is unimplement for every T since compiler believes Mod<T>: !Send for every T
// impl<T> From<T> for UnitTarget
// where for<'a> Mod<T>: Send {
//     fn from(_: T) -> Self {
//         Self
//     }
// }

// Same issue
// impl<T> From<T> for UnitTarget
// where T: Marker {
//     fn from(_: T) -> Self {
//         Self
//     }
// }

// Works but doesn't make sense
// impl<T: Send> From<T> for UnitTarget {
//     fn from(_: T) -> Self {
//         Self
//     }
// }

// Works and at least this makes more sense
// impl<T: Debug> From<T> for UnitTarget {
//     fn from(_: T) -> Self {
//         Self
//     }
// }

trait Sealed {}
pub trait PubTrait {}

trait TraitBool<const B: bool> {}

impl TraitBool<false> for () {}
impl TraitBool<true> for () {}

// From<Self> is implemented for every sized type
pub struct BoxNewType<T: ?Sized>(Box<T>);

pub struct Infallible2(Infallible);

pub trait MapBoxFrom<T: ?Sized> {
    fn map_box_from(value: Box<T>) -> Box<Self>;
}

pub trait MapBoxInto<T: ?Sized> {
    fn map_box_into(self: Box<Self>) -> Box<T>;
}

impl<T: ?Sized, R: ?Sized> MapBoxInto<R> for T
where
    R: MapBoxFrom<T>,
{
    fn map_box_into(self: Box<Self>) -> Box<R> {
        R::map_box_from(self)
    }
}

impl<T, R> MapBoxFrom<T> for R
where
    R: From<T>,
{
    fn map_box_from(value: Box<T>) -> Box<Self> {
        value.pipe(|i| *i).conv::<R>().into()
    }
}

pub trait TryMapBoxFrom<T: ?Sized> {
    type Error;
    fn try_map_box_from(value: Box<T>) -> Result<Box<Self>, Self::Error>;
}

pub trait TryMapBoxInto<T: ?Sized> {
    type Error;
    fn try_map_box_into(self: Box<Self>) -> Result<Box<T>, Self::Error>;
}

impl<T: ?Sized, R: ?Sized> TryMapBoxInto<R> for T
where
    R: TryMapBoxFrom<T>,
{
    type Error = R::Error;
    fn try_map_box_into(self: Box<Self>) -> Result<Box<R>, Self::Error> {
        R::try_map_box_from(self)
    }
}
// // Why this API will never be stabilised: Conflict between this
// impl<T: ?Sized, R: ?Sized> TryMapBoxFrom<T> for R
// // Leave (Sized) TryFrom impls naked
// where R: MapBoxFrom<T>
// {
//     type Error = Infallible;
//     fn try_map_box_from(value: Box<T>) -> Result<Box<Self>, Self::Error>{
//         value
//         .pipe(R::map_box_from)
//         .pipe(Ok)
//     }
// }
// // And
// impl<T, R> TryMapBoxFrom<T> for R
// // Leave (Unsized) MapBoxFrom impls naked
// where R: TryFrom<T>
// {
//     type Error = R::Error;
//     fn try_map_box_from(value: Box<T>) -> Result<Box<Self>, Self::Error>{
//         value
//         .pipe(|i| *i)
//         .pipe(R::try_from)
//         .map(Box::new)
//     }
// }

// Sacraficial Targets: !Debug !Unpin(Futures)

// static USELESS_ARRAY: [(); 0] = [];

//pub type NoTarget = [()];
pub trait NoTargetTrait {}
impl NoTargetTrait for () {}
pub type NoTarget = dyn NoTargetTrait;
//pub struct NoTarget<T: ?Sized = [()]>(T);

impl<T> MapBoxFrom<T> for NoTarget {
    fn map_box_from(_: Box<T>) -> Box<Self> {
        //Box::from(&USELESS_ARRAY[..])
        //Box::new(NoTarget(USELESS_ARRAY)) as _
        Box::new(()) as _
    }
}

// Conflict with Box::from(T), can't constraint T to Unsized
// impl<T: ?Sized> BoxFrom<T> for Box<T> {

// }

struct Ass<T>(T);

// Example not allowed - T must be wrapped in MyStruct<T> in trait parameter but not in Self:
// // Not okay
// impl<T> AsRef<dyn FnOnce() -> T> for dyn FnOnce() -> T {
//     fn as_ref(&self) -> &T {
//         todo!()
//     }
// }
// // Okay
// impl<T> AsRef<Ass<T>> for dyn FnOnce() -> T {
//     fn as_ref(&self) -> &T {
//         todo!()
//     }
// }

// Equality using Sealed traits
mod dyn_sealed {

    pub trait DynTyEqSealed<T: ?Sized> {}

    impl<T: ?Sized> DynTyEqSealed<T> for T {}
}

pub trait DynTyEq<T: ?Sized>: dyn_sealed::DynTyEqSealed<T> {}

// Equality using Sealed traits
mod sealed {
    use crate::transmute::Transmutable;

    pub trait TyEqSealed<T: ?Sized> {}

    impl<T: ?Sized> TyEqSealed<T> for T {}
}

pub trait TyEq<T: ?Sized>: sealed::TyEqSealed<T> {
    fn transmute_hkt_into<'a, 't, F: HktUnsized<'t>>(
        a: F::FUnsized<'a, Self>,
    ) -> F::FUnsized<'a, T> {
        assert_eq!(typeid::of::<T>(), typeid::of::<Self>());

        // SAFETY: `Self` and `T` are the same type,
        // therefore F::FUnsized<'a, Self> and F::FUnsized<'a, T>
        // are the same type, hence transmutation between the two
        // are always safe.
        unsafe { transmute::transmute_unchecked(a) }
    }

    fn transmute_hkt_from<'a, 't, F: HktUnsized<'t>>(
        a: F::FUnsized<'a, T>,
    ) -> F::FUnsized<'a, Self> {
        assert_eq!(typeid::of::<T>(), typeid::of::<Self>());

        // SAFETY: `Self` and `T` are the same type,
        // therefore F::FUnsized<'a, Self> and F::FUnsized<'a, T>
        // are the same type, hence transmutation between the two
        // are always safe.
        unsafe { transmute::transmute_unchecked(a) }
    }
}

impl<T: ?Sized> TyEq<T> for T {}

pub trait DynSelf<T: ?Sized>: DynTyEqSealed<T> {
    // fn into_sized(self: Box<Self>) -> T;
}

#[repr(transparent)]
pub struct A;

pub type AsUnsized<T> = dyn DynSelf<T>;

impl<T: ?Sized> DynSelf<T> for T {}

pub trait DynImpl<T> {
    fn into_sized(self) -> T;
}

impl<T> DynImpl<T> for Box<dyn DynSelf<T>> {
    fn into_sized(self: Box<dyn DynSelf<T>>) -> T {
        unsafe {
            let ptr = Box::into_raw(self);
            *Box::from_raw(ptr as *mut T)
        }
    }
}

// mod sealed {
//     pub trait TyEq
//     where
//         Self: From<Self::Type> + Into<Self::Type>,
//         Self::Type: From<Self> + Into<Self>,
//     {
//         type Type;
//     }

//     impl<T> TyEq for T {
//         type Type = T;
//     }
// }
// pub trait SelfImpl<T>
// where Box<Self> : sealed::TyEq<Type = Box<T>> {
//     fn into_sized(self: Box<Self>) -> T;
// }

// impl<T> SelfImpl<T> for T {
//     fn into_sized(self: Box<Self>) -> T {
//         *self
//     }
// }

// // Ok
// impl<T, R> AsRef<dyn SelfImpl<T = R>> for dyn SelfImpl<T = T> {
//     fn as_ref(&self) -> &(dyn SelfImpl<T = R> + 'static) {
//         todo!()
//     }
// }

impl<T> MapBoxFrom<dyn DynSelf<T>> for dyn DynSelf<T> {
    fn map_box_from(value: Box<dyn DynSelf<T>>) -> Box<Self> {
        value
    }
}

impl<T, R> MapBoxFrom<AsUnsized<T>> for R
where
    R: From<T>,
{
    fn map_box_from(value: Box<AsUnsized<T>>) -> Box<Self> {
        value.into_sized().conv::<R>().into()
    }
}
