//! Unsafe transmute functions for shared types confirmed to be transmute-possible while avoid exposing public APIs, as well as functions for transmuting entire Hkt stacks.

use core::{
    mem::ManuallyDrop,
};

use tap::Pipe;

use crate::marker_classification::TyEq;

union TransmuteUnion<A, B> {
    a: ManuallyDrop<A>,
    b: ManuallyDrop<B>,
}

/// Convert the inner type into the wrapper type.
/// Transmutation doesn't cause data races because it either copies a pointer or moved data, which references to are proved not to exist.
#[inline]
pub(crate) unsafe fn unsafe_transmute<T, U>(s: U) -> T
where
    T: Sized,
    U: Sized,
{
    assert!(size_of::<U>() == size_of::<T>());
    assert!(align_of::<U>() == align_of::<T>());
    // SAFETY: Both types must have the exact same memory representation at runtime.
    unsafe {
        TransmuteUnion::<U, T> {
            a: ManuallyDrop::new(s),
        }
        .b
        .pipe(ManuallyDrop::into_inner)
    }
}

/// Like [unsafe_transmute] but also checks the [TypeId](std::any::TypeId) of `T` and `U` for equality as if they were bound by `'static` using [typeid::of].
/// Use this to unsafely assert at compile time that `T` and `U` are exactly the same type - including lifetimes.
/// 
/// # Safety
/// This function is `unsafe` because undefined behavior related to lifetimes can still occur since `T` and `U` are not bounded by `'static`. If the [TypeId](std::any::TypeId) check were sufficient, any undefined behavior would have been stopped at runtime by a [panic](assert_eq). This function is only guaranteed to be safe if `T` and `U` are exactly the same type - of course including lifetimes.
#[inline]
#[allow(unused)]
pub(crate) unsafe fn unsafe_transmute_id<T, U>(s: U) -> T
where
    T: Sized,
    U: Sized,
{
    assert_eq!(typeid::of::<T>(), typeid::of::<U>());

    // SAFETY: if the above is true, then both types must have the exact same memory representation at runtime.
    unsafe { transmute_unchecked(s) }
}

/// Convert the inner type into the wrapper type.
#[inline]
pub(crate) unsafe fn transmute_unchecked<T, U>(s: U) -> T
where
    T: Sized,
    U: Sized,
{
    debug_assert!(size_of::<U>() == size_of::<T>());
    debug_assert!(align_of::<U>() == align_of::<T>());

    // SAFETY: Both types must have the exact same memory representation at runtime.
    unsafe {
        TransmuteUnion::<U, T> {
            a: ManuallyDrop::new(s),
        }
        .b
        .pipe(ManuallyDrop::into_inner)
    }
}

#[allow(unused)]
/// # Safety
///  See [std::mem::transmute]
pub unsafe trait Transmutable<Target: ?Sized> {}

unsafe impl<T: ?Sized + TyEq<R>, R: ?Sized> Transmutable<R> for T {}

#[allow(unused)]
/// # Safety
///  Only safe when T: Transmutable<R> and Self::F<'a, T> has the exact same memory layout as Self::F<'a, R> and do not depend on TypeIds.
pub unsafe trait TransmutableHkt {}

// unsafe impl<'a, TInner: Hkt<'a>, A: 'a> Transmutable<TInner::F<A>> for DependentWrapper<'a, TInner, A> {}

const _: () = {
    use crate::hkt::id::IdHkt;
    impl_trait_for!(unsafe TransmutableHkt => IdHkt);
    // impl_trait_for_wrapper!(unsafe TransmutableHkt: T => 
    // );
};

pub fn transmute_hkt<'a, 't, A: Transmutable<B>, B, F: TransmutableHkt + Hkt<'t>>(
    a: F::F<'a, A>,
) -> F::F<'a, B> {
    debug_assert!(size_of::<A>() == size_of::<B>());
    debug_assert!(align_of::<A>() == align_of::<B>());

    unsafe { transmute_unchecked(a) }
}

#[allow(unused)]
/// # Safety
///  Same as [std::mem::transmute]
pub(crate) unsafe fn unsafe_transmute_hkt<'a, 't, A, B, F: TransmutableHkt + Hkt<'t>>(a: F::F<'a, A>) -> F::F<'a, B> {
    assert!(size_of::<A>() == size_of::<B>());
    assert!(align_of::<A>() == align_of::<B>());

    unsafe { transmute_unchecked(a) }
}

use crate::hkt::{Hkt, HktUnsized};

#[allow(unused)]
pub(crate) fn transmute_hkt_unsized<
    'a, 't,
    A: ?Sized + Transmutable<B>,
    B: ?Sized,
    F: TransmutableHkt + HktUnsized<'t>,
>(
    a: F::FUnsized<'a, A>,
) -> F::FUnsized<'a, B> {
    debug_assert!(size_of::<*const A>() == size_of::<*const B>());

    unsafe { transmute_unchecked(a) }
}

// pub fn wrap_hkt_3_unsized<
//     'a, 't,
//     A,
//     F: TransmutableHkt + Hkt<'t>,
//     Fb: TransmutableHkt + Hkt<'t>,
//     Fc: TransmutableHkt + Hkt<'t>,
// >(
//     a: F::F<'a, Fb::F<'a, Fc::F<'a, A>>>,
// ) -> DependentWrapper<'a, 't, (), F::F<'a, DependentWrapper<'a, 't, (), Fb::F<'a, DependentWrapper<'a, 't, (), Fc::F<'a, DependentWrapper<'a, 't, (), A>>>>>>> {
//     unsafe { transmute_unchecked(a) }
// }


// /// # Safety
// ///  Same as [std::mem::transmute]
// pub unsafe fn unsafe_transmute_hkt_unsized<
//     'a,
//     A: ?Sized,
//     B: ?Sized,
//     F: TransmutableHkt + HktUnsized<'a>,
// >(
//     a: F::FUnsized<A>,
// ) -> F::FUnsized<B> {
//     assert!(size_of::<*const A>() == size_of::<*const B>());

//     unsafe { transmute_unchecked(a) }
// }

// #[repr(transparent)]
// struct RefStr<'a>(&'a str);

// unsafe impl<'a> Transmutable<RefStr<'a>> for &'a str {}

// #[repr(transparent)]
// struct Str(str);

// unsafe impl Transmutable<Str> for str {}

// fn sample() {
//     let string = "".to_string();
//     let binding = string.as_str();
//     let array = Arc::new(Mutex::new(vec![&binding]));

//     let result = transmute_hkt::<_, RefStr<'_>, IdT<ArcT<MutexT<VecT<RefT>>>>>(array.clone());

//     let resource = result.clone();

//     let lock = resource.lock().unwrap();
//     let content_ref = &*lock;

//     let result_arc = resource.clone();

//     let result = transmute_hkt_unsized::<_, Str, ArcT<MutexT<VecT<RefT<RefT>>>>>(array.clone());

//     let resource = result.clone();

//     let lock = resource.lock().unwrap();
//     let content_ref = &*lock;

//     let result_arc = resource.clone();
// }
