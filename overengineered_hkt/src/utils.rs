use std::mem::needs_drop;

#[cfg(false)]
pub(crate) fn compose_fn_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C,
{
    |a| g(f(a))
}

pub(crate) fn placehold() -> ! {
    unimplemented!()
}

pub(crate) trait IntoFnOnce<A> {
    type Ret;
    fn into_fn(self) -> impl FnOnce(A) -> Self::Ret;
}

pub(crate) trait AsFnMut<A> {
    type Ret;
    fn as_mut_fn(&mut self) -> impl FnMut(A) -> Self::Ret;
}

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
pub struct CloneWrapper<T, F>(pub T, pub F);

impl<T, F: Fn(&T) -> T + Clone> Clone for CloneWrapper<T, F> {
    fn clone(&self) -> Self {
        Self(self.1(&self.0), self.1.clone())
    }
}


#[cfg(test)]
mod test {
    use std::panic::resume_unwind;


    #[test]
    fn scratch() {

        // Only valid if unwinding is enabled
        #[cfg(panic = "unwind")]
        fn f() -> i32 {

            let some_result = Err(());

            // Struct to mark the break signal specific to this code
            struct Break;

            // Catch all panics
            match std::panic::catch_unwind(||
                some_result.unwrap_or_else(|_| {
                    // Trigger panicking with the break signal as payload without invoking the global panic hook.
                    std::panic::resume_unwind(Box::new(Break))
                })
            ) {
                Ok(i) => i,
                // Inspect the payload if panic was thrown. If you are certain nothing inside std::panic::catch_unwind other than your signal can panic, you can use panic!() or std::panic::resume_unwind(Box::new(())) instead, skip checking the type of the payload and just return the default value.
                Err(e) =>
                    // If its the specified signal, return a default value
                    if e.is::<Break>() { 1 }
                    // Else resume panicking with that payload
                    else { resume_unwind(e) },
            }
        }
    }
}
