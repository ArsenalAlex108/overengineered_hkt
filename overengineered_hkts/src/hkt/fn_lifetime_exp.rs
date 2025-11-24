use std::rc::Rc;

pub(crate) fn clone_fn_from_ref<'ref_lt, 'f_in_lt, 'f_out_lt, 'a, 'b, A: 'a, B: 'b>
(f1: &'ref_lt (impl 'f_in_lt + Fn(A) -> B))
-> impl 'f_out_lt + Fn(A) -> B 
where
// Trivial
'f_in_lt: 'ref_lt,
// Required => 'f_out_lt is bound by 'ref_lt and not 'f_in_lt 
'ref_lt: 'f_out_lt {  
    f1
}

pub(crate) fn clone_fn<'ref_lt, 'f_in_lt, 'f_out_lt, 'a, 'b, A: 'a, B: 'b>
(f1: &'ref_lt (impl 'f_in_lt + Fn(A) -> B + Clone))
-> impl 'f_out_lt + Fn(A) -> B + Clone
where
// Trivial
'f_in_lt: 'ref_lt,
// Required
'f_in_lt: 'f_out_lt {  
    f1.clone()
}

pub(crate) fn clone_fn_once_ref<'ref_lt, 'f_in_lt, 'f_out_lt, 'a, 'b, A: 'a, B: 'b>
(f1: &'ref_lt (impl 'f_in_lt + FnOnce(A) -> B + Clone))
-> impl 'f_out_lt + Fn(A) -> B + Clone
where
// Trivial
'f_in_lt: 'ref_lt,
// Required => 'f_out_lt is bound by 'ref_lt and not 'f_in_lt 
'ref_lt: 'f_out_lt, {  
    move |a| f1.clone()(a)
}

/// Any closure/[FnOnce] that are [Clone] automatically implements [Fn]
pub(crate) fn clone_fn_once<'f_in_lt, 'f_out_lt, 'a, 'b, A: 'a, B: 'b>
(f1: impl 'f_in_lt + FnOnce(A) -> B + Clone)
-> impl 'f_in_lt + Fn(A) -> B + Clone {  
    move |a| f1.clone()(a)
}

// Error without unsafe
// pub(crate) fn rc_ref<'a, A>(rc: Rc<A>) -> &'a A {
    
// }

pub(crate) fn leak_rc<A>(rc: Rc<A>) -> &'static A {
    unsafe {
        &*Rc::into_raw(rc)
    }
}

enum Borrown<'a, T> {
    Borrow(&'a T),
    Own(T)
}

// impl<'a, T> Clone for Borrown<'a, T> {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Borrow(arg0) => Self::Borrow(arg0),
//             // Err
//             Self::Own(arg0) => Self::Borrow(arg0),
//         }
//     }
// }

// pub(crate) fn borrown_ref<'a, A>(val: Borrown<'a, A>) -> &'a A {
//     match val {
//         Borrown::Borrow(r) => r,
//         // Err
//         Borrown::Own(t) => &t,
//     }
// }

