#[cfg(false)]
pub(crate) fn compose_fn_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C,
{
    |a| g(f(a))
}

#[allow(unused)]
pub(crate) fn placehold() -> ! {
    unimplemented!()
}

#[allow(unused)]
pub(crate) trait IntoFnOnce<A> {
    type Ret;
    fn into_fn(self) -> impl FnOnce(A) -> Self::Ret;
}

#[allow(unused)]
pub(crate) trait AsFnMut<A> {
    type Ret;
    fn as_mut_fn(&mut self) -> impl FnMut(A) -> Self::Ret;
}

#[allow(unused)]
pub(crate) trait AsFn<A> {
    type Ret;
    fn as_fn(&self) -> impl Fn(A) -> Self::Ret;
}

impl<T: naan::fun::F1Once<A>, A> IntoFnOnce<A> for T {
    type Ret = T::Ret;

    fn into_fn(self) -> impl FnOnce(A) -> Self::Ret {
        |a| self.call1(a)
    }
}

impl<T: naan::fun::F1<A>, A> AsFn<A> for T {
    type Ret = T::Ret;

    fn as_fn(&self) -> impl Fn(A) -> Self::Ret {
        |a| self.call(a)
    }
}

// fn copy<T>(t: &T) -> Option<T> {
//     match needs_drop::<T>() {
//         true => None,
//         false => Some(
//             unsafe {
//                 // UB: Pointer aliasing rule if T contains &mut
//                 std::ptr::read(t)
//             }
//         )
//     }
// }

/// Wraps a value and a cloning function. Useful for moving them into closures and cloning later.
/// # Example:
/// ```
/// use overengineered_hkt::utils::CloneWrapper;
/// 
/// // Suppose this is an opaque struct not implementing Clone
/// struct Val(i32);
/// 
/// let val = Val(1);
/// 
/// let func = move || val;
/// // func does not implements Clone
/// // func.clone();
/// 
/// // But does with the wrapper:
/// 
/// let wrapper = CloneWrapper(Val(1), |i: &Val| Val(i.0));
/// 
/// let func = move || wrapper.clone().0;
/// // Now func does implement Clone
/// func.clone();
/// 
/// ```
pub struct CloneWrapper<T, F>(pub T, pub F);

impl<T, F> CloneWrapper<T, F> {
    /// Obtain a reference to T using this method to force the closure to move the wrapper struct instead of just T.
    /// 
    /// Instead of: `&wrapper.clone().0`
    /// 
    /// Consider this if you only want the reference while forcing the closure to move the wrapper struct: `wrapper.get_ref_t()`
    pub fn get_ref_t(&self) -> &T {
        &self.0
    }
}

impl<T, F: Fn(&T) -> T + Clone> Clone for CloneWrapper<T, F> {
    fn clone(&self) -> Self {
        Self(self.1(&self.0), self.1.clone())
    }
}
