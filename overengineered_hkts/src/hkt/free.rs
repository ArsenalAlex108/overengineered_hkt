use std::{convert::Infallible, marker::PhantomData};

use crate::hkt::{Applicative, DependentCloneK, Functor, Hkt, Monad, Pure};

pub struct FreeHkt<F>(Infallible, PhantomData<F>);

pub enum Free<'a, F: Hkt<'a>, A: 'a> {
    Pure(A),
    Bind(F::F<Free<'a, F, A>>),
}

impl<'a, F: Hkt<'a>> Hkt<'a> for FreeHkt<F> {
    type F<A: 'a> = Free<'a, F, A>;
}

impl<'a, F: DependentCloneK<'a>, A: Clone> Clone for Free<'a, F, A> {
    fn clone(&self) -> Self {
        match self {
            Self::Pure(arg0) => Self::Pure(arg0.clone()),
            Self::Bind(arg0) => Self::Bind(F::clone(arg0)),
        }
    }
}

impl<'a, K: Functor<'a>> Functor<'a> for FreeHkt<K> {
    fn map<A, B, F>(mut f: F, fa: Self::F<A>) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> B + Clone,
    {
        match fa {
            Free::Pure(a) => Free::Pure(f(a)),
            Free::Bind(fa) => Free::Bind(K::map(move |a| Self::map(f.clone(), a), fa)),
        }
    }
}

impl<'a, K: Functor<'a>> Pure<'a> for FreeHkt<K> {
    fn pure<A: 'a>(a: A) -> Self::F<A> {
        Free::Pure(a)
    }
}

impl<'a, K: Functor<'a> + DependentCloneK<'a>> Applicative<'a> for FreeHkt<K> {
    fn apply<A, B, F>(ff: Self::F<F>, fa: Self::F<A>) -> Self::F<B>
    where
        A: 'a + Clone,
        F: 'a + FnMut(A) -> B + Clone,
    {
        match (ff, fa) {
            (Free::Pure(mut f), Free::Pure(a)) => Free::Pure(f(a)),
            (Free::Pure(f), Free::Bind(a)) => {
                Free::Bind(K::map(move |i| Self::map(f.clone(), i), a))
            }
            // (Free::Bind(f), Free::Pure(a)) => {
            //     let func = move |i| Self::apply(i, Free::Pure(a.clone()));
            //     Free::Bind(K::map(func, f))
            // },
            // (Free::Bind(f), Free::Bind(a)) => {
            //     let a = DependentWrapperT::<K>::wrap(a);
            //     let func = move |i| Self::apply(i, Free::Bind(a.clone().into_inner()));
            //     Free::Bind(K::map(func, f))
            // }
            (Free::Bind(f), a) => {
                let func = move |i| Self::apply(i, a.clone());
                Free::Bind(K::map(func, f))
            }
        }
    }
}

impl<'a, K: Functor<'a> + DependentCloneK<'a>> Monad<'a> for FreeHkt<K> {
    fn bind<A, B, F>(fa: Self::F<A>, mut f: F) -> Self::F<B>
    where
        F: 'a + FnMut(A) -> Self::F<B> + Clone,
    {
        match fa {
            Free::Pure(a) => f(a),
            Free::Bind(m) => Free::Bind(K::map(move |a| Self::bind(a, f.clone()), m)),
        }
    }
}
