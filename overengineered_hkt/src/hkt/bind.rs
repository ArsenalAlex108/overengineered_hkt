use std::{convert::Infallible, marker::PhantomData};

use tap::Pipe as _;

use crate::hkt::{
    CloneK, CovariantK, Functor, Hkt, HktUnsized, PureMapInner, TCloneableOf5, UnsizedHkt, UnsizedHktUnsized, id::IdHkt, nullary::NullaryHkt, one_of::{NotT1of5, NotT2of5, NotT3of5, OneOf5Hkt, T5Of5Hkt}
};

/// [BindT] is required to implement [Functor], [DependentCloneK], [DependentExpandLifetimeK], [CovariantK] so try implementing these for `TOuter::F<()>` first... It is not possible for [BindT] to implement [Applicative][Applicative], [Monad][Monad]...
///
/// [Applicative]: crate::hkt::Applicative
/// [Monad]: crate::hkt::Monad
pub struct BindT<TOuter = IdHkt, TInner = IdHkt>(Infallible, PhantomData<(TOuter, TInner)>);

// impl<TOuter: UnsizedHktUnsized, TInner: UnsizedHkt> UnsizedHkt for BindT<TOuter, TInner> {
//     type UnsizedF<'a, A: 'a> = TOuter::UnsizedFUnsized<'a, TInner::UnsizedF<'t, A>>
//     where
//         't: 'a;
// }

// Conflict with above - there are 16 possible combinations
// impls prioritizing Sized inputs bounds
impl<'t, TOuter: UnsizedHkt<'t>, TInner: Hkt<'t>> UnsizedHkt<'t> for BindT<TOuter, TInner> {
    type UnsizedF<'a, A: 'a>
        = TOuter::UnsizedF<'a, TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: Hkt<'t>, TInner: Hkt<'t>> Hkt<'t> for BindT<TOuter, TInner> {
    type F<'a, A: 'a>
        = TOuter::F<'a, TInner::F<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: UnsizedHkt<'t>, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t>
    for BindT<TOuter, TInner>
{
    type UnsizedFUnsized<'a, A: ?Sized + 'a>
        = TOuter::UnsizedF<'a, TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

impl<'t, TOuter: Hkt<'t>, TInner: HktUnsized<'t>> HktUnsized<'t> for BindT<TOuter, TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = TOuter::F<'a, TInner::FUnsized<'a, A>>
    where
        't: 'a;
}

// pub struct BindUnsizedT<TOuter = IdHkt, TInner = IdHkt>(Infallible, PhantomData<(TOuter, TInner)>);

// impl<'t, TOuter: UnsizedHkt<'t>, TInner: Hkt<'t>> UnsizedHkt<'t> for BindUnsizedT<TOuter, TInner> {
//     type UnsizedF<'a, A: 'a>
//         = TOuter::UnsizedF<'a, TInner::F<'a, A>>
//     where
//         't: 'a;
// }

// impl<'t, TOuter: UnsizedHktUnsized<'t>, TInner: HktUnsized<'t>> UnsizedHktUnsized<'t> for BindUnsizedT<TOuter, TInner> {
//     type UnsizedFUnsized<'a, A: ?Sized + 'a> = TOuter::UnsizedFUnsized<'a, TInner::FUnsized<'a, A>>
//     where
//         't: 'a;
// }

// impl<'t, TOuter: HktUnsized<'t>, TInner: HktUnsized<'t>> HktUnsized<'t> for BindUnsizedT<TOuter, TInner> {
//     type FUnsized<'a, A: ?Sized + 'a> = TOuter::FUnsized<'a, TInner::FUnsized<'a, A>>
//     where
//         't: 'a;
// }

impl<
    't,
    ReqIn: Hkt<'t> + CloneK<'t>,
    ReqOut: Hkt<'t> + CloneK<'t>,
    ReqF1: OneOf5Hkt<'t> + TCloneableOf5<'t>,
    TOuter: Functor<'t, ReqIn, ReqOut, ReqF1>,
    TInner: Functor<'t, ReqIn, ReqOut, ReqF1>,
> Functor<'t, ReqIn, ReqOut, ReqF1> for BindT<TOuter, TInner>
{
    fn map<'a, A, B, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>(
        in_requirement: ReqIn::F<'a, A>,
        out_requirement: ReqOut::F<'a, B>,
        f: <ReqF1>::OneOf5F<'a, F1Once, F1Mut, F1Fn, F1Clone, F1Copy>,
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
        't: 'a,
    {
        // F needs to be Cloneable
        TOuter::map(
            ReqIn::clone((), &in_requirement),
            ReqOut::clone((), &out_requirement),
            ReqF1::create_from(&f, move |ka| {
                TInner::map(
                    ReqIn::clone((), &in_requirement),
                    ReqOut::clone((), &out_requirement),
                    // TODO: The mirror lies
                    f,
                    ka,
                )
            }),
            fa,
        )
    }
}

// impl<'t, TOuter: DependentCloneK<'t>, TInner: DependentCloneK<'t>> DependentCloneK<'t> for BindT<TOuter, TInner> {
//     fn clone_f<'a, 'b, F: DependentCloneK<'t>, A: 'a + Clone>(a: &Self::F<'a, F::F<'a, A>>) -> Self::F<'b, F::F<'b, A>> where 't: 'a {
//         TOuter::clone_f::<BindT<TInner, F>, _>(a)
//     }
// }

impl<
    't,
    TOuter: CovariantK<'t> + Functor<'t, NullaryHkt, NullaryHkt, T5Of5Hkt>,
    TInner: CovariantK<'t>,
> CovariantK<'t> for BindT<TOuter, TInner>
{
    fn covariant_convert<'a, 'b, A>(
        a: Self::F<'a, A>,
    ) -> Self::F<'b, A>
    where
        A: 'a,
        'a: 'b,
        't: 'a + 'b,
    {
        TOuter::map(
            (),
            (),
            |a| TInner::covariant_convert::<'a, 'b>(a),
            TOuter::covariant_convert::<'a, 'b>(a),
        )
    }
}
