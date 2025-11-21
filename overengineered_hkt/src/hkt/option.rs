use std::{cell::Cell, convert::Infallible, marker::PhantomData, rc::Rc};

use naan::fun::{F1, F2Once};

use crate::{
    hkt::{Applicative, CloneFreeApplicative, Functor, Hkt, HktUnsized, Monad, Pure},
    utils::AsFn,
};

pub struct OptionT<TInner>(Infallible, PhantomData<TInner>);

impl<'a, TInner: Hkt<'a>> Hkt<'a> for OptionT<TInner> {
    type F<A: 'a> = Option<TInner::F<A>>;
}

impl<'a, TInner: HktUnsized<'a>> HktUnsized<'a> for OptionT<TInner> {
    type FUnsized<A: 'a + ?Sized> = Option<TInner::FUnsized<A>>;
}

impl<'a, TInner: Functor<'a>> Functor<'a> for OptionT<TInner> {
    fn map<A, B, F>(f: F, fa: Self::F<A>) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.map(TInner::map.curry().call(f).as_fn())
    }
}

impl<'a, TInner: Pure<'a>> Pure<'a> for OptionT<TInner> {
    fn pure<A>(a: A) -> Self::F<A> {
        Some(TInner::pure(a))
    }
}

impl<'a, TInner: Applicative<'a>> Applicative<'a> for OptionT<TInner> {
    fn apply<A, B, F>(ff: Self::F<F>, fa: Self::F<A>) -> Self::F<B>
    where
        A: 'a + Clone,
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.and_then(|a| ff.map(|f| TInner::apply(f, a)))
    }
}

impl<'a, TInner: CloneFreeApplicative<'a>> CloneFreeApplicative<'a> for OptionT<TInner> {
    fn apply_once<A, B, F>(ff: Self::F<F>, fa: Self::F<A>) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> B + Clone,
    {
        fa.and_then(|a| ff.map(|f| TInner::apply_once(f, a)))
    }
}

impl<'a, TInner: Monad<'a>> Monad<'a> for OptionT<TInner> {
    fn bind<A, B, F>(fa: Self::F<A>, f: F) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> Self::F<B> + Clone,
    {
        fa.and_then(|ta| {
            #[cfg(not(panic = "unwind"))]
            return {
                let has_none_val_org = Rc::new(Cell::new(false));
                let has_none_val = Rc::clone(&has_none_val_org);
                let stack = TInner::map(
                    move |a| match f.clone()(a) {
                        Some(a) => Some(a),
                        None => {
                            has_none_val.set(true);
                            None
                        }
                    },
                    ta,
                );

                if has_none_val_org.clone().take() {
                    None
                } else {
                    Some(TInner::bind(stack, |a| {
                        a.expect("OptionT bind impl is correct.")
                    }))
                }
            };
            #[cfg(panic = "unwind")]
            return {
                use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

                struct Break;

                match catch_unwind(AssertUnwindSafe(|| {
                    TInner::bind(ta, move |a| match f.clone()(a) {
                        None => resume_unwind(Box::new(Break)),
                        Some(a) => a,
                    })
                })) {
                    Err(e) => {
                        if e.is::<Break>() {
                            None
                        } else {
                            resume_unwind(e)
                        }
                    }
                    Ok(a) => Some(a),
                }
            };
        })
    }
}
