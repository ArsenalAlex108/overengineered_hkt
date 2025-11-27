// TODO: Ability to Add custom bounds like Debug

#[cfg(false)]
pub mod array;
/// TODO
pub mod bind;
/// TODO
pub mod boxed;
#[cfg(false)]
pub mod dependent_wrapper;
#[cfg(false)]
pub mod extensions;
#[cfg(false)]
#[deprecated]
pub mod fn_lifetime_exp;
#[cfg(false)]
pub mod fold_vec;
#[cfg(false)]
pub mod free;
pub mod future;
pub mod id;
pub mod iter;
#[cfg(feature = "laws")]
pub mod laws;
#[cfg(not(feature = "laws"))]
pub(crate) mod laws;
pub mod nullary;
pub mod one_of;
pub mod option;
pub mod reference;
pub mod result;
pub mod slice;
pub mod sync;
pub mod traits;
#[cfg(false)]
pub mod tuple;
pub mod vec;

pub use traits::*;

// let string = "".to_string();
// let func = || &string;
// let func = func.clone();

// /// An enum used for controlling the execution of `fold_while`.
// ///
// /// See [`.fold_while()`](Itertools::fold_while) for more information.
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum FoldWhile<T> {
//     /// Continue folding with this value
//     Continue(T),
//     /// Fold is complete and will return this value
//     Done(T),
// }

// impl<T> From<itertools::FoldWhile<T>> for FoldWhile<T> {
//     fn from(value: itertools::FoldWhile<T>) -> Self {
//         match value {
//             itertools::FoldWhile::Continue(t) => Self::Continue(t),
//             itertools::FoldWhile::Done(t) => Self::Done(t),
//         }
//     }
// }

// impl<T> From<FoldWhile<T>> for itertools::FoldWhile<T> {
//     fn from(value: FoldWhile<T>) -> Self {
//         match value {
//             FoldWhile::Continue(t) => Self::Continue(t),
//             FoldWhile::Done(t) => Self::Done(t),
//         }
//     }
// }

// impl<'a, T: UnsizedTransmuteFunctorUnsized<'a>> UnsizedTransmuteFunctor<'a> for T {
//     type UnsizedTransmuteF<A: 'a> = T::UnsizedTransmuteFUnsized<A>;
// }

// impl<'a, T: TransmuteFunctorUnsized<'a>> TransmuteFunctor<'a> for T {
//     type TransmuteF<A: 'a> = T::TransmuteFUnsized<A>;
// }

// pub trait HktUnsizedTransformer<'a> {
//     type Ft<K: UnsizedHkt<'a>>: Hkt<'a>;
//     type UnsizedFt<K: UnsizedHktUnsized<'a>>: HktUnsized<'a> + TyEq<Self::Ft<K>>;
// }

// pub struct BoxTTransformer(Infallible);

// impl<'a> HktUnsizedTransformer<'a> for BoxTTransformer {
//     type Ft<K: UnsizedHkt<'a>> = BoxT<K>;
//     type UnsizedFt<K: UnsizedHktUnsized<'a>> = BoxT<K>;
// }

// pub unsafe trait SendHkt<'a>: UnsizedHkt<'a> {
//     type __F<A: 'a>: ?Sized + TyEq<Self::UnsizedF<A>> where Self::UnsizedF<A>: Send;
// }

// unsafe impl<'a, TInner: SendHkt<'a>> SendHkt<'a> for ArcT<TInner> {
//     type __F<A: 'a> = Self::UnsizedF<A> where Self::UnsizedF<A>: Send;
// }

// fn send_hkt<'a, A, F: SendHkt<'a> + HktUnsized<'a>>(a: Box<F::UnsizedF<A>>) where F::UnsizedF<A>: std::marker::Send {
//     fn send(_: impl Send) {}
//     send(a);
// }

// Exposed K is forbidden
// impl<'a, K: HktUnsizedTransformer<'a>> Functor<'a> for K::UnsizedFt<DynIteratorT<()>>
// where K::UnsizedFt<DynIteratorT<()>>: IntoIteratorHkt<'a> {
//     fn map<A: 'a, B: 'a, F: 'a + FnMut(A) -> B + Clone>(f: F, fa: Self::F<A>) -> Self::F<B> {
//         fa.pipe(K::UnsizedFt::<DynIteratorT<()>>
//         ::into_iter).map(f).pipe(Box::new) as _
//     }
// }

// Incorrect:
// impl<'a, K: HKTUnsized<'a>> HKT<'a> for IteratorT<K> {
//     type F<A: 'a> = K::FUnsized<dyn Iterator<Item = A>>;
// }
//
// impl<'a, K: UnsizedHKTUnsized<'a>> UnsizedHKT<'a> for IteratorT<K> {
//     type UnsizedF<A: 'a> = K::UnsizedFUnsized<dyn Iterator<Item = A>>;
// }

#[cfg(test)]
mod test {
    use core::ops::{BitAnd, Not};

    use const_random::const_random;

    use crate::{
        hkt::{Monad, id::IdHkt, one_of::T4Of5Hkt, option::OptionT, result::ResultT, vec::VecT},
        marker_classification::DependentClone,
    };

    type Hkt = VecT<VecT>;
    type HktI = Vec<Vec<i32>>;

    fn get_test_base() -> HktI {
        vec![
            vec![const_random!(i32), const_random!(i32), const_random!(i32)],
            vec![const_random!(i32), const_random!(i32), const_random!(i32)],
            vec![const_random!(i32), const_random!(i32), const_random!(i32)],
        ]
    }

    fn rand() -> i32 {
        const_random!(i32)
    }

    fn eq(a: HktI, b: HktI) -> bool {
        a == b
    }

    fn f1(i: i32) -> i32 {
        i.overflowing_mul(23).0
    }
    fn f2(i: i32) -> i32 {
        i.overflowing_mul(37).0
    }

    fn i32_to_option_vec(i: i32) -> Option<Vec<i32>> {
        match i {
            ..=-1 => None,
            0 => Some(Vec::new()),
            1.. => Some(vec![i; i.try_into().expect("i > 0 here")]),
        }
    }

    fn i32_to_vec_option(i: i32) -> Vec<Option<i32>> {
        if i <= 0 {
            Vec::new()
        } else if i % 2 == 0 {
            (1..i)
                .flat_map(|i| [Some(i), None])
                .chain([Some(i)])
                .collect()
        } else {
            (1..=i).map(Some).collect()
        }
    }

    fn i32_to_vec_option_2(i: i32) -> Vec<Option<i32>> {
        match i {
            ..=-1 => vec![],
            0 => vec![Some(0)],
            1.. => core::iter::repeat_n(None, i.try_into().expect("Positive num"))
            .chain([Some(i)])
            .collect()
        }
    }

    /// TODO: proper error collection
    fn validate(results: impl IntoIterator<Item = bool>) {
        let err_indices =
            results
                .into_iter()
                .enumerate()
                .fold(Vec::new(), |mut sum, (index, success)| {
                    if success.not() {
                        sum.push(index)
                    }
                    sum
                });

        if err_indices.is_empty().not() {
            panic!("Validation failed at indices: {:#?}", err_indices);
        }
    }

    #[test]
    fn test_functor_laws() {
        use super::laws::functor;
        validate([
            functor::identity_law::<Hkt>(get_test_base(), eq),
            functor::composite_law::<Hkt>(get_test_base(), f1, f2, eq),
        ])
    }

    type ArbitraryHkt = VecT<OptionT<VecT>>;
    //Can't compare: VecT<ResultT<i32, PinBoxFutureT<OptionT<BoxT<DynIteratorT>>>>>

    // Breaks linking:
    //VecT<OptionT<VecT<ResultT<i32, IdHkt>>>>;
    #[test]
    fn test_applicatve_laws() {
        use super::laws::applicative;
        validate([
            applicative::identity_law::<Hkt>(rand(), eq),
            applicative::composition_law::<Hkt>(rand(), f1, f2, eq),
            applicative::homomorphism_law::<Hkt>(rand(), f1, eq),
            applicative::interchange_law::<Hkt>(rand(), f1, eq),
            applicative::composition_law_k::<Hkt>(get_test_base(), f1, f2, eq),
        ]);

        let eq = |a, b| a == b;

        validate([
            applicative::identity_law::<ArbitraryHkt>(rand(), eq),
            applicative::composition_law::<ArbitraryHkt>(rand(), f1, f2, eq),
            applicative::homomorphism_law::<ArbitraryHkt>(rand(), f1, eq),
            applicative::interchange_law::<ArbitraryHkt>(rand(), f1, eq),
        ]);
    }

    #[test]
    fn test_monad_laws() {
        use super::laws::monad;

        let arbitrary_eq = |a, b| a == b;

        validate([
            monad::left_identity_law::<Hkt>(rand(), eq),
            monad::right_identity_law::<Hkt>(rand(), eq),
            monad::associativity_law::<Hkt>(rand(), f1, f2, eq),
            monad::left_identity_law_k::<Hkt>(get_test_base(), eq),
            monad::associativity_law_k::<Hkt>(get_test_base(), f1, f2, eq),
            monad::left_identity_law::<ArbitraryHkt>(rand(), arbitrary_eq),
            monad::right_identity_law::<ArbitraryHkt>(rand(), arbitrary_eq),
            monad::associativity_law::<ArbitraryHkt>(rand(), f1, f2, arbitrary_eq),
        ]);
    }

    #[test]
    fn traversable_laws() {}

    #[test]
    fn test_optiont_and_vect_bind() {
        let input = Some(vec![-1, 0, 1, 2]);
        let input = Some(vec![1, 2]);
        let expected = Some(vec![1, 2, 2]);

        assert_eq!(
            <OptionT<VecT> as Monad<DependentClone, DependentClone, T4Of5Hkt>>::bind::<
                _,
                _,
                fn(i32) -> Option<Vec<i32>>,
                fn(i32) -> Option<Vec<i32>>,
                fn(i32) -> Option<Vec<i32>>,
                _,
                fn(i32) -> Option<Vec<i32>>,
            >(|i| *i, |i| *i, input, i32_to_option_vec),
            expected
        );
    }

    #[test]
    fn test_vect_and_optiont_bind() {
        let input = vec![None, Some(0), Some(1), Some(2)];
        // let input = Some(vec![2]);
        let expected = vec![None, Some(0), None, Some(1), None, None, Some(2)];

        // let input = vec![Some(-1), Some(0), Some(1), Some(2)];
        assert_eq!(
            <VecT<OptionT> as Monad<DependentClone, DependentClone, T4Of5Hkt>>::bind::<
                _,
                _,
                fn(i32) -> Vec<Option<i32>>,
                fn(i32) -> Vec<Option<i32>>,
                fn(i32) -> Vec<Option<i32>>,
                _,
                fn(i32) -> Vec<Option<i32>>,
            >(|i| *i, |i| *i, input, i32_to_vec_option_2),
            expected
        );
    }

    #[test]
    fn test_vect_id_bind() {
        let input = vec![-1, 0, 1, 2];
        let expected = vec![1, 2, 2];

        // let input = vec![Some(-1), Some(0), Some(1), Some(2)];
        assert_eq!(
            <VecT as Monad<DependentClone, DependentClone, T4Of5Hkt>>::bind::<
                _,
                _,
                fn(i32) -> Vec<i32>,
                fn(i32) -> Vec<i32>,
                fn(i32) -> Vec<i32>,
                _,
                fn(i32) -> Vec<i32>,
            >(
                |i| *i,
                |i| *i,
                input,
                |i| i32_to_option_vec(i).unwrap_or_default()
            ),
            expected
        );
    }

    #[test]
    fn test_optiont_id_bind() {
        let input = Some(Some(1));
        let expected = Some(Some(3));

        // let input = vec![Some(-1), Some(0), Some(1), Some(2)];
        assert_eq!(
            <OptionT<OptionT> as Monad<DependentClone, DependentClone, T4Of5Hkt>>::bind::<
                _,
                _,
                fn(i32) -> Option<Option<i32>>,
                fn(i32) -> Option<Option<i32>>,
                fn(i32) -> Option<Option<i32>>,
                _,
                fn(i32) -> Option<Option<i32>>,
            >(|i| *i, |i| *i, input, |i| Some(Some(i + 2))),
            expected
        );

        let expected = Some(None);

        // let input = vec![Some(-1), Some(0), Some(1), Some(2)];
        assert_eq!(
            <OptionT<OptionT> as Monad<DependentClone, DependentClone, T4Of5Hkt>>::bind::<
                _,
                _,
                fn(i32) -> Option<Option<i32>>,
                fn(i32) -> Option<Option<i32>>,
                fn(i32) -> Option<Option<i32>>,
                _,
                fn(i32) -> Option<Option<i32>>,
            >(|i| *i, |i| *i, input, |i| Some(None)),
            expected
        );
    }
}
