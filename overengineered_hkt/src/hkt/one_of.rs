use core::clone::Clone;
use core::convert::Infallible;
use core::marker::PhantomData;
pub enum OneOf5<T1, T2, T3, T4, T5> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
}
#[doc = " Specific pattern indicating actual value is one of the given types."]
#[doc = " Types can either map over all types - or eliminate specific types by requiring specific traits proving actual value is not that type."]
pub trait OneOf5Hkt<'t> {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>: 'a
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a;

    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b;

    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b;

    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        'b: 'f,
        't: 'a + 'b;

    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        match Self::into_one_of_5_enum(s) {
            OneOf5::T1(t) | OneOf5::T2(t) | OneOf5::T3(t) | OneOf5::T4(t) | OneOf5::T5(t) => t,
        }
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b;

    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a;
}
#[doc = " Indicates that the Hkt does not contains an instannce of `T1`"]
pub trait NotT1of5<'t>: OneOf5Hkt<'t> {
    #[doc = " Set `T1` to an arbitrary type parameter `T1b`."]
    fn arbitrary_t1<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T1b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T1b>,
    ) -> Self::OneOf5F<'a, T1b, T2a, T3a, T4a, T5a>
    where
        't: 'a;
}
#[doc = " Indicates that the Hkt does not contains an instannce of `T2`"]
pub trait NotT2of5<'t>: OneOf5Hkt<'t> {
    #[doc = " Set `T2` to an arbitrary type parameter `T2b`."]
    fn arbitrary_t2<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T2b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T2b>,
    ) -> Self::OneOf5F<'a, T1a, T2b, T3a, T4a, T5a>
    where
        't: 'a;
}
#[doc = " Indicates that the Hkt does not contains an instannce of `T3`"]
pub trait NotT3of5<'t>: OneOf5Hkt<'t> {
    #[doc = " Set `T3` to an arbitrary type parameter `T3b`."]
    fn arbitrary_t3<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T3b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T3b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3b, T4a, T5a>
    where
        't: 'a;
}
#[doc = " Indicates that the Hkt does not contains an instannce of `T4`"]
pub trait NotT4of5<'t>: OneOf5Hkt<'t> {
    #[doc = " Set `T4` to an arbitrary type parameter `T4b`."]
    fn arbitrary_t4<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T4b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T4b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4b, T5a>
    where
        't: 'a;
}
#[doc = " Indicates that the Hkt does not contains an instannce of `T5`"]
pub trait NotT5of5<'t>: OneOf5Hkt<'t> {
    #[doc = " Set `T5` to an arbitrary type parameter `T5b`."]
    fn arbitrary_t5<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T5b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T5b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5b>
    where
        't: 'a;
}
pub struct T1Of5Hkt(Infallible);

impl<'t> OneOf5Hkt<'t> for T1Of5Hkt {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>
        = T1
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a,
    {
        OneOf5::T1(s)
    }
    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        't: 'a + 'b,
    {
        map_t1(s)
    }
    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        s
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b,
    {
        value
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a,
    {
        old.clone()
    }
}
impl<'t> NotT2of5<'t> for T1Of5Hkt {
    #[doc = " Set `T1` to an arbitrary type parameter `T1b`."]
    fn arbitrary_t2<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T2b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T2b>,
    ) -> Self::OneOf5F<'a, T1a, T2b, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT3of5<'t> for T1Of5Hkt {
    #[doc = " Set `T1` to an arbitrary type parameter `T1b`."]
    fn arbitrary_t3<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T3b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T3b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3b, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT4of5<'t> for T1Of5Hkt {
    #[doc = " Set `T1` to an arbitrary type parameter `T1b`."]
    fn arbitrary_t4<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T4b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T4b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4b, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT5of5<'t> for T1Of5Hkt {
    #[doc = " Set `T1` to an arbitrary type parameter `T1b`."]
    fn arbitrary_t5<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T5b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T5b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5b>
    where
        't: 'a,
    {
        s
    }
}
pub struct T2Of5Hkt(Infallible);

impl<'t> OneOf5Hkt<'t> for T2Of5Hkt {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>
        = T2
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a,
    {
        OneOf5::T2(s)
    }
    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        't: 'a + 'b,
    {
        map_t2(s)
    }
    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        s
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b,
    {
        value
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a,
    {
        old.clone()
    }
}
impl<'t> NotT1of5<'t> for T2Of5Hkt {
    #[doc = " Set `T2` to an arbitrary type parameter `T2b`."]
    fn arbitrary_t1<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T1b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T1b>,
    ) -> Self::OneOf5F<'a, T1b, T2a, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT3of5<'t> for T2Of5Hkt {
    #[doc = " Set `T2` to an arbitrary type parameter `T2b`."]
    fn arbitrary_t3<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T3b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T3b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3b, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT4of5<'t> for T2Of5Hkt {
    #[doc = " Set `T2` to an arbitrary type parameter `T2b`."]
    fn arbitrary_t4<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T4b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T4b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4b, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT5of5<'t> for T2Of5Hkt {
    #[doc = " Set `T2` to an arbitrary type parameter `T2b`."]
    fn arbitrary_t5<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T5b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T5b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5b>
    where
        't: 'a,
    {
        s
    }
}
pub struct T3Of5Hkt(Infallible);

impl<'t> OneOf5Hkt<'t> for T3Of5Hkt {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>
        = T3
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a,
    {
        OneOf5::T3(s)
    }
    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        't: 'a + 'b,
    {
        map_t3(s)
    }
    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        s
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b,
    {
        value
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a,
    {
        old.clone()
    }
}
impl<'t> NotT1of5<'t> for T3Of5Hkt {
    #[doc = " Set `T3` to an arbitrary type parameter `T3b`."]
    fn arbitrary_t1<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T1b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T1b>,
    ) -> Self::OneOf5F<'a, T1b, T2a, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT2of5<'t> for T3Of5Hkt {
    #[doc = " Set `T3` to an arbitrary type parameter `T3b`."]
    fn arbitrary_t2<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T2b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T2b>,
    ) -> Self::OneOf5F<'a, T1a, T2b, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT4of5<'t> for T3Of5Hkt {
    #[doc = " Set `T3` to an arbitrary type parameter `T3b`."]
    fn arbitrary_t4<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T4b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T4b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4b, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT5of5<'t> for T3Of5Hkt {
    #[doc = " Set `T3` to an arbitrary type parameter `T3b`."]
    fn arbitrary_t5<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T5b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T5b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5b>
    where
        't: 'a,
    {
        s
    }
}
pub struct T4Of5Hkt(Infallible);

impl<'t> OneOf5Hkt<'t> for T4Of5Hkt {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>
        = T4
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a,
    {
        OneOf5::T4(s)
    }
    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        't: 'a + 'b,
    {
        map_t4(s)
    }
    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        s
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b,
    {
        value
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a,
    {
        old.clone()
    }
}
impl<'t> NotT1of5<'t> for T4Of5Hkt {
    #[doc = " Set `T4` to an arbitrary type parameter `T4b`."]
    fn arbitrary_t1<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T1b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T1b>,
    ) -> Self::OneOf5F<'a, T1b, T2a, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT2of5<'t> for T4Of5Hkt {
    #[doc = " Set `T4` to an arbitrary type parameter `T4b`."]
    fn arbitrary_t2<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T2b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T2b>,
    ) -> Self::OneOf5F<'a, T1a, T2b, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT3of5<'t> for T4Of5Hkt {
    #[doc = " Set `T4` to an arbitrary type parameter `T4b`."]
    fn arbitrary_t3<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T3b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T3b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3b, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT5of5<'t> for T4Of5Hkt {
    #[doc = " Set `T4` to an arbitrary type parameter `T4b`."]
    fn arbitrary_t5<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T5b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T5b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5b>
    where
        't: 'a,
    {
        s
    }
}
pub struct T5Of5Hkt(Infallible);

impl<'t> OneOf5Hkt<'t> for T5Of5Hkt {
    type OneOf5F<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>
        = T5
    where
        't: 'a;
    #[doc = " Converts this object into a matchable enum."]
    fn into_one_of_5_enum<'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a>(
        s: Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> OneOf5<T1, T2, T3, T4, T5>
    where
        't: 'a,
    {
        OneOf5::T5(s)
    }
    #[doc = " Converts a mut reference to this object into an owned object with mut references to each variant,"]
    fn as_mut<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a mut Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Converts a reference to this object into an owned object with references to each variant,"]
    fn as_ref<'a, 'b, T1: 'b, T2: 'b, T3: 'b, T4: 'b, T5: 'b>(
        s: &'a Self::OneOf5F<'b, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5>
    where
        'b: 'a,
        't: 'b,
    {
        s
    }
    #[doc = " Perform mapping on each type - but only exactly one of the provided functions will execute."]
    fn map_one_of_5<
        'a,
        'b,
        'f,
        T1a: 'a,
        T2a: 'a,
        T3a: 'a,
        T4a: 'a,
        T5a: 'a,
        T1b: 'b,
        T2b: 'b,
        T3b: 'b,
        T4b: 'b,
        T5b: 'b,
    >(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        map_t1: impl 'f + FnOnce(T1a) -> T1b,
        map_t2: impl 'f + FnOnce(T2a) -> T2b,
        map_t3: impl 'f + FnOnce(T3a) -> T3b,
        map_t4: impl 'f + FnOnce(T4a) -> T4b,
        map_t5: impl 'f + FnOnce(T5a) -> T5b,
    ) -> Self::OneOf5F<'b, T1b, T2b, T3b, T4b, T5b>
    where
        'a: 'f,
        't: 'a + 'b,
    {
        map_t5(s)
    }
    #[doc = " Converts into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_5_enum](OneOf5Hkt::into_one_of_5_enum), but a more efficient implementation may be provided instead."]
    fn into_one_of_5<'a, T: 'a>(s: Self::OneOf5F<'a, T, T, T, T, T>) -> T
    where
        't: 'a,
    {
        s
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn create_from<'a, 'b, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T: 'b>(
        _: &Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
        value: T,
    ) -> Self::OneOf5F<'b, T, T, T, T, T>
    where
        't: 'a + 'b,
    {
        value
    }
    #[doc = " Create a new object from an existing one to store a different value."]
    fn clone_one_of_5<
        'a,
        'b,
        T1: 'a + Clone,
        T2: 'a + Clone,
        T3: 'a + Clone,
        T4: 'a + Clone,
        T5: 'a + Clone,
    >(
        old: &'b Self::OneOf5F<'a, T1, T2, T3, T4, T5>,
    ) -> Self::OneOf5F<'a, T1, T2, T3, T4, T5>
    where
        'a: 'b,
        't: 'a,
    {
        old.clone()
    }
}
impl<'t> NotT1of5<'t> for T5Of5Hkt {
    #[doc = " Set `T5` to an arbitrary type parameter `T5b`."]
    fn arbitrary_t1<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T1b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T1b>,
    ) -> Self::OneOf5F<'a, T1b, T2a, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT2of5<'t> for T5Of5Hkt {
    #[doc = " Set `T5` to an arbitrary type parameter `T5b`."]
    fn arbitrary_t2<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T2b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T2b>,
    ) -> Self::OneOf5F<'a, T1a, T2b, T3a, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT3of5<'t> for T5Of5Hkt {
    #[doc = " Set `T5` to an arbitrary type parameter `T5b`."]
    fn arbitrary_t3<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T3b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T3b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3b, T4a, T5a>
    where
        't: 'a,
    {
        s
    }
}
impl<'t> NotT4of5<'t> for T5Of5Hkt {
    #[doc = " Set `T5` to an arbitrary type parameter `T5b`."]
    fn arbitrary_t4<'a, T1a: 'a, T2a: 'a, T3a: 'a, T4a: 'a, T5a: 'a, T4b: 'a>(
        s: Self::OneOf5F<'a, T1a, T2a, T3a, T4a, T5a>,
        _type_infer: PhantomData<T4b>,
    ) -> Self::OneOf5F<'a, T1a, T2a, T3a, T4b, T5a>
    where
        't: 'a,
    {
        s
    }
}

// use std::{convert::Infallible, marker::PhantomData};

// use tap::Pipe as _;

// pub enum OneOfSix<A, B, C, D, E, F> {
//     A(A),
//     B(B),
//     C(C),
//     D(D),
//     E(E),
//     F(F),
// }

// /// Specific pattern indicating actual value is one of the given types.
// /// Types can either map over all types - or eliminate specific types by requiring specific traits proving actual value is not that type.
// pub trait OneOfSixHkt<'t> {
//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>: 'a
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a;

//     /// Perform mapping on each type.
//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b;

//     /// Unwrap into `T` when all variants have the same type. The default implementation uses [Self::into_enum](OneOfSixHkt::into_enum), but a more efficient implementation may be provided instead.
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         match Self::into_enum(s) {
//             OneOfSix::A(t)
//             | OneOfSix::B(t)
//             | OneOfSix::C(t)
//             | OneOfSix::D(t)
//             | OneOfSix::E(t)
//             | OneOfSix::F(t) => t,
//         }
//     }

//     /// Create a new object from an existing one to store a different value.
//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         ld: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a;
// }

// /// Indicates that the Hkt does not contains an instannce of `A`
// pub trait NotAOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `A` to an arbitrary type parameter.
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a;
// }

// /// Indicates that the Hkt does not contains an instannce of `B`
// pub trait NotBOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `B` to an arbitrary type parameter.
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a;
// }

// /// Indicates that the Hkt does not contains an instannce of `C`
// pub trait NotCOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `C` to an arbitrary type parameter.
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a;
// }

// /// Indicates that the Hkt does not contains an instannce of `D`
// pub trait NotDOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `D` to an arbitrary type parameter.
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a;
// }

// /// Indicates that the Hkt does not contains an instannce of `E`
// pub trait NotEOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `E` to an arbitrary type parameter.
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a;
// }
// /// Indicates that the Hkt does not contains an instannce of `A`
// pub trait NotFOfSixHkt<'t>: OneOfSixHkt<'t> {
//     /// Set `A` to an arbitrary type parameter.
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a;
// }

// pub struct AOfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for AOfSixHkt {
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = A
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::A(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_a(s)
//     }

//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotBOfSixHkt<'t> for AOfSixHkt {
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotCOfSixHkt<'t> for AOfSixHkt {
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotDOfSixHkt<'t> for AOfSixHkt {
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotEOfSixHkt<'t> for AOfSixHkt {
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotFOfSixHkt<'t> for AOfSixHkt {
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// pub struct BOfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for BOfSixHkt {
//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = B
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::B(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_b(s)
//     }

//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotAOfSixHkt<'t> for BOfSixHkt {
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotCOfSixHkt<'t> for BOfSixHkt {
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotDOfSixHkt<'t> for BOfSixHkt {
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotEOfSixHkt<'t> for BOfSixHkt {
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotFOfSixHkt<'t> for BOfSixHkt {
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// pub struct COfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for COfSixHkt {
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = C
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::C(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_c(s)
//     }

//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotAOfSixHkt<'t> for COfSixHkt {
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotBOfSixHkt<'t> for COfSixHkt {
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotDOfSixHkt<'t> for COfSixHkt {
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotEOfSixHkt<'t> for COfSixHkt {
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotFOfSixHkt<'t> for COfSixHkt {
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// pub struct DOfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for DOfSixHkt {
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = D
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::D(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_d(s)
//     }
//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotAOfSixHkt<'t> for DOfSixHkt {
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotBOfSixHkt<'t> for DOfSixHkt {
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotCOfSixHkt<'t> for DOfSixHkt {
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotEOfSixHkt<'t> for DOfSixHkt {
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotFOfSixHkt<'t> for DOfSixHkt {
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// pub struct EOfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for EOfSixHkt {
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = E
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::E(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_e(s)
//     }

//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotAOfSixHkt<'t> for EOfSixHkt {
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotBOfSixHkt<'t> for EOfSixHkt {
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotCOfSixHkt<'t> for EOfSixHkt {
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotDOfSixHkt<'t> for EOfSixHkt {
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotFOfSixHkt<'t> for EOfSixHkt {
//     fn arbitrary_f<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, F2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<F2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F2>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// pub struct FOfSixHkt(Infallible);

// impl<'t> OneOfSixHkt<'t> for FOfSixHkt {
//     fn unwrap_all<'a, T: 'a>(s: Self::OneOfSixF<'a, T, T, T, T, T, T>) -> T
//     where
//         't: 'a,
//     {
//         s
//     }

//     type OneOfSixF<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>
//         = F
//     where
//         't: 'a;

//     fn into_enum<'a, A: 'a, B: 'a, C: 'a, D: 'a, E: 'a, F: 'a>(
//         s: Self::OneOfSixF<'a, A, B, C, D, E, F>,
//     ) -> OneOfSix<A, B, C, D, E, F>
//     where
//         't: 'a,
//     {
//         OneOfSix::F(s)
//     }

//     fn map_one_of_six<
//         'a,
//         'b,
//         'f,
//         A1: 'a,
//         B1: 'a,
//         C1: 'a,
//         D1: 'a,
//         E1: 'a,
//         F1: 'a,
//         A2: 'b,
//         B2: 'b,
//         C2: 'b,
//         D2: 'b,
//         E2: 'b,
//         F2: 'b,
//     >(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         map_a: impl 'f + FnOnce(A1) -> A2,
//         map_b: impl 'f + FnOnce(B1) -> B2,
//         map_c: impl 'f + FnOnce(C1) -> C2,
//         map_d: impl 'f + FnOnce(D1) -> D2,
//         map_e: impl 'f + FnOnce(E1) -> E2,
//         map_f: impl 'f + FnOnce(F1) -> F2,
//     ) -> Self::OneOfSixF<'b, A2, B2, C2, D2, E2, F2>
//     where
//         'a: 'f,
//         't: 'a + 'b,
//     {
//         map_f(s)
//     }

//     fn create_from<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, T: 'a>(
//         _: &Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         value: T,
//     ) -> Self::OneOfSixF<'a, T, T, T, T, T, T>
//     where
//         't: 'a,
//     {
//         value
//     }
// }

// impl<'t> NotAOfSixHkt<'t> for FOfSixHkt {
//     fn arbitrary_a<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, A2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<A2>,
//     ) -> Self::OneOfSixF<'a, A2, B1, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotBOfSixHkt<'t> for FOfSixHkt {
//     fn arbitrary_b<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, B2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<B2>,
//     ) -> Self::OneOfSixF<'a, A1, B2, C1, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotCOfSixHkt<'t> for FOfSixHkt {
//     fn arbitrary_c<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, C2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<C2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C2, D1, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotDOfSixHkt<'t> for FOfSixHkt {
//     fn arbitrary_d<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, D2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<D2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D2, E1, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// impl<'t> NotEOfSixHkt<'t> for FOfSixHkt {
//     fn arbitrary_e<'a, A1: 'a, B1: 'a, C1: 'a, D1: 'a, E1: 'a, F1: 'a, E2: 'a>(
//         s: Self::OneOfSixF<'a, A1, B1, C1, D1, E1, F1>,
//         _: PhantomData<E2>,
//     ) -> Self::OneOfSixF<'a, A1, B1, C1, D1, E2, F1>
//     where
//         't: 'a,
//     {
//         s
//     }
// }

// /// Can contain (X O O O)..(X X X X)
// /// So (A B C D) can map to (X O O O)..(X X X X)
// /// Transitivity
// pub struct MinFnOnceHkt(Infallible);

// impl<'t> FnTypeHkt<'t> for MinFnOnceHkt {
//     type FnTypeEnum<'a, TFnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>
//         = MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
//     where
//         't: 'a;

//     fn map_fn<
//         'a,
//         'b,
//         TFnOnce1: 'a,
//         TFnMut1: 'a,
//         TFn1: 'a,
//         TFnClone1: 'a,
//         TFnOnce2: 'b,
//         TFnMut2: 'b,
//         TFn2: 'b,
//         TFnClone2: 'b,
//     >(
//         s: Self::FnTypeEnum<'a, TFnOnce1, TFnMut1, TFn1, TFnClone1>,
//         map_fn_once: impl FnOnce(TFnOnce1) -> TFnOnce2,
//         map_fn_mut: impl FnOnce(TFnMut1) -> TFnMut2,
//         map_fn: impl FnOnce(TFn1) -> TFn2,
//         map_fn_clone: impl FnOnce(TFnClone1) -> TFnClone2,
//     ) -> Self::FnTypeEnum<'b, TFnOnce2, TFnMut2, TFn2, TFnClone2>
//     where
//         't: 'a + 'b,
//     {
//         match s.0 {
//             FnTypes::FnOnce(a) => map_fn_once(a).pipe(FnTypes::FnOnce),
//             FnTypes::FnMut(b) => map_fn_mut(b).pipe(FnTypes::FnMut),
//             FnTypes::Fn(c) => map_fn(c).pipe(FnTypes::Fn),
//             FnTypes::FnClone(d) => map_fn_clone(d).pipe(FnTypes::FnClone),
//         }.pipe(MinFnOnce)
//     }

//     fn into_enum<'a, TFnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>(s: Self::FnTypeEnum<'a, TFnOnce, TFnMut, TFn, TFnClone>) -> FnTypes<TFnOnce, TFnMut, TFn, TFnClone> where 't: 'a {
//         s.0
//     }
// }

// pub struct MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>(
//     pub(super) FnTypes<TFnOnce, TFnMut, TFn, TFnClone>,
// );

// pub enum FnTypes<TFnOnce, TFnMut, TFn, TFnClone> {
//     FnOnce(TFnOnce),
//     FnMut(TFnMut),
//     Fn(TFn),
//     FnClone(TFnClone),
// }

// impl<T> FnTypes<T, T, T, T> {
//     fn unwrap_all(self) -> T {
//         match self {
//             FnTypes::FnOnce(t) | FnTypes::FnMut(t) | FnTypes::Fn(t) | FnTypes::FnClone(t) => t,
//         }
//     }
// }

// impl<'t> FromFnCloneHkt<'t> for MinFnOnceHkt {
//     fn from_fn_clone<'a, X: 'a>(f: X) -> Self::FnTypeEnum<'a, X, X, X, X>
//     where
//         't: 'a,
//     {
//         MinFnOnce(FnTypes::FnClone(f))
//     }
// }

// impl<'t> FromFnHkt<'t> for MinFnOnceHkt {
//     fn from_fn<'a, X, O>(f: X) -> Self::FnTypeEnum<'a, X, X, X, O>
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         MinFnOnce(FnTypes::Fn(f))
//     }
// }

// impl<'t> FromFnMutHkt<'t> for MinFnOnceHkt {
//     fn from_fn_mut<'a, X, O>(f: X) -> Self::FnTypeEnum<'a, X, X, O, O>
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         MinFnOnce(FnTypes::FnMut(f))
//     }
// }

// impl<'t> FromFnOnceHkt<'t> for MinFnOnceHkt {
//     fn from_fn_once<'a, X, O>(f: X) -> Self::FnTypeEnum<'a, X, O, O, O>
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         MinFnOnce(FnTypes::FnOnce(f))
//     }
// }

// impl<'t> IntoFnOnceHkt<'t> for MinFnOnceHkt {
//     fn into_fn_once<'a, X, O>(s: Self::FnTypeEnum<'a, X, O, O, O>) -> X
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         match s.0 {
//             FnTypes::FnOnce(x) => x,
//             FnTypes::FnMut(o) | FnTypes::Fn(o) | FnTypes::FnClone(o) => {
//                 todo!()
//             }
//         }
//     }
// }

// impl<'t> AnyIntoFnOnceHkt<'t> for MinFnOnceHkt {
//     fn any_into_fn_once<'a, A, B, C, D>(s: Self::FnTypeEnum<'a, A, B, C, D>) -> A
//     where
//         A: 'a,
//         B: 'a,
//         C: 'a,
//         D: 'a,
//         't: 'a,
//     {
//         // SAFETY: variants ensured by sealing the `FnTypeHkt`
//         unsafe {
//             match s.0 {
//                 FnTypes::FnOnce(a) => a,
//                 FnTypes::FnMut(b) => unsafe_transmute_id(b),
//                 FnTypes::Fn(c) => unsafe_transmute_id(c),
//                 FnTypes::FnClone(d) => unsafe_transmute_id(d),
//             }
//         }
//     }
// }

// impl<'t> IntoFnMutHkt<'t> for MinFnOnceHkt {
//     fn into_fn_mut<'a, X, O>(s: Self::FnTypeEnum<'a, X, X, O, O>) -> X
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         match s.0 {
//             FnTypes::FnOnce(x) | FnTypes::FnMut(x) => x,
//             FnTypes::Fn(o) | FnTypes::FnClone(o) => todo!(),
//         }
//     }
// }

// impl<'t> IntoFnHkt<'t> for MinFnOnceHkt {
//     fn into_fn<'a, X, O>(s: Self::FnTypeEnum<'a, X, X, X, O>) -> X
//     where
//         X: 'a,
//         O: 'a,
//         't: 'a,
//     {
//         match s.0 {
//             FnTypes::FnOnce(x) | FnTypes::FnMut(x) | FnTypes::Fn(x) => x,
//             FnTypes::FnClone(o) => todo!(),
//         }
//     }
// }

// impl<'t> IntoFnCloneHkt<'t> for MinFnOnceHkt {
//     fn into_fn_clone<'a, X>(s: Self::FnTypeEnum<'a, X, X, X, X>) -> X
//     where
//         X: 'a,
//         't: 'a,
//     {
//         match s.0 {
//             FnTypes::FnOnce(x)
//             | FnTypes::FnMut(x)
//             | FnTypes::Fn(x)
//             | FnTypes::FnClone(x) => x,
//         }
//     }
// }

// // impl<TFnOnce, TFnMut, TFn, TFnClone> FromFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
// //     for MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
// // {
// //     fn from_fn_once(f: TFnOnce) -> Self {
// //         Self::FnOnce(f)
// //     }
// // }

// // impl<TFnOnce, TFnMut, TFn, TFnClone> FromFnMut<TFnOnce, TFnMut, TFn, TFnClone>
// //     for MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
// // {
// //     fn from_fn_mut(f: TFnMut) -> Self {
// //         Self::FnMut(f)
// //     }
// // }

// // impl<TFnOnce, TFnMut, TFn, TFnClone> FromFn<TFnOnce, TFnMut, TFn, TFnClone>
// //     for MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
// // {
// //     fn from_fn(f: TFn) -> Self {
// //         Self::Fn(f)
// //     }
// // }

// // impl<TFnOnce, TFnMut, TFn, TFnClone> FromFnClone<TFnOnce, TFnMut, TFn, TFnClone>
// //     for MinFnOnce<TFnOnce, TFnMut, TFn, TFnClone>
// // {
// //     fn from_fn_clone(f: TFnClone) -> Self {
// //         Self::FnClone(f)
// //     }
// // }

// // impl<F> IntoFnOnce<F, F, F, F>
// //     for MinFnOnce<F, F, F, F>
// // {
// //     fn into_fn_once(self) -> F {
// //         match self {
// //             MinFnOnce::FnOnce(f)
// //             | MinFnOnce::FnMut(f)
// //             | MinFnOnce::Fn(f)
// //             | MinFnOnce::FnClone(f) => f,
// //         }
// //     }
// // }

// // impl<F, T> IntoFnMut<T, F, F, F>
// //     for MinFnOnce<T, F, F, F>
// // {
// //     fn into_fn_mut(self) -> F {
// //         todo!()
// //     }
// // }

// pub struct MinFnMutHkt(Infallible);

// impl<'t> FnTypeHkt<'t> for MinFnMutHkt {
//     type FnTypeEnum<'a, _FnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>
//         = MinFnMut<TFnMut, TFn, TFnClone>
//     where
//         't: 'a;

//     fn map_fn<
//         'a,
//         'b,
//         TFnOnce1: 'a,
//         TFnMut1: 'a,
//         TFn1: 'a,
//         TFnClone1: 'a,
//         TFnOnce2: 'b,
//         TFnMut2: 'b,
//         TFn2: 'b,
//         TFnClone2: 'b,
//     >(
//         s: Self::FnTypeEnum<'a, TFnOnce1, TFnMut1, TFn1, TFnClone1>,
//         map_fn_once: impl FnOnce(TFnOnce1) -> TFnOnce2,
//         map_fn_mut: impl FnOnce(TFnMut1) -> TFnMut2,
//         map_fn: impl FnOnce(TFn1) -> TFn2,
//         map_fn_clone: impl FnOnce(TFnClone1) -> TFnClone2,
//     ) -> Self::FnTypeEnum<'b, TFnOnce2, TFnMut2, TFn2, TFnClone2>
//     where
//         't: 'a + 'b,
//     {
//         todo!()
//     }

//     fn into_enum<'a, TFnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>(s: Self::FnTypeEnum<'a, TFnOnce, TFnMut, TFn, TFnClone>) -> FnTypes<TFnOnce, TFnMut, TFn, TFnClone> where 't: 'a {
//         todo!()
//     }
// }

// pub enum MinFnMut<TFnMut, TFn, TFnClone> {
//     FnMut(TFnMut),
//     Fn(TFn),
//     FnClone(TFnClone),
// }

// pub struct MinFnHkt(Infallible);

// impl<'t> FnTypeHkt<'t> for MinFnHkt {
//     type FnTypeEnum<'a, _FnOnce: 'a, _FnMut: 'a, TFn: 'a, TFnClone: 'a>
//         = MinFn<TFn, TFnClone>
//     where
//         't: 'a;

//     fn map_fn<
//         'a,
//         'b,
//         TFnOnce1: 'a,
//         TFnMut1: 'a,
//         TFn1: 'a,
//         TFnClone1: 'a,
//         TFnOnce2: 'b,
//         TFnMut2: 'b,
//         TFn2: 'b,
//         TFnClone2: 'b,
//     >(
//         s: Self::FnTypeEnum<'a, TFnOnce1, TFnMut1, TFn1, TFnClone1>,
//         map_fn_once: impl FnOnce(TFnOnce1) -> TFnOnce2,
//         map_fn_mut: impl FnOnce(TFnMut1) -> TFnMut2,
//         map_fn: impl FnOnce(TFn1) -> TFn2,
//         map_fn_clone: impl FnOnce(TFnClone1) -> TFnClone2,
//     ) -> Self::FnTypeEnum<'b, TFnOnce2, TFnMut2, TFn2, TFnClone2>
//     where
//         't: 'a + 'b,
//     {
//         todo!()
//     }

//     fn into_enum<'a, TFnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>(s: Self::FnTypeEnum<'a, TFnOnce, TFnMut, TFn, TFnClone>) -> FnTypes<TFnOnce, TFnMut, TFn, TFnClone> where 't: 'a {
//         todo!()
//     }
// }

// pub enum MinFn<TFn, TFnClone> {
//     Fn(TFn),
//     FnClone(TFnClone),
// }

// pub struct MinFnCloneHkt(Infallible);

// impl<'t> FnTypeHkt<'t> for MinFnCloneHkt {
//     type FnTypeEnum<'a, _FnOnce: 'a, _FnMut: 'a, _Fn: 'a, TFnClone: 'a>
//         = TFnClone
//     where
//         't: 'a;

//     fn map_fn<
//         'a,
//         'b,
//         TFnOnce1: 'a,
//         TFnMut1: 'a,
//         TFn1: 'a,
//         TFnClone1: 'a,
//         TFnOnce2: 'b,
//         TFnMut2: 'b,
//         TFn2: 'b,
//         TFnClone2: 'b,
//     >(
//         s: Self::FnTypeEnum<'a, TFnOnce1, TFnMut1, TFn1, TFnClone1>,
//         map_fn_once: impl FnOnce(TFnOnce1) -> TFnOnce2,
//         map_fn_mut: impl FnOnce(TFnMut1) -> TFnMut2,
//         map_fn: impl FnOnce(TFn1) -> TFn2,
//         map_fn_clone: impl FnOnce(TFnClone1) -> TFnClone2,
//     ) -> Self::FnTypeEnum<'b, TFnOnce2, TFnMut2, TFn2, TFnClone2>
//     where
//         't: 'a + 'b,
//     {
//         todo!()
//     }

//     fn into_enum<'a, TFnOnce: 'a, TFnMut: 'a, TFn: 'a, TFnClone: 'a>(s: Self::FnTypeEnum<'a, TFnOnce, TFnMut, TFn, TFnClone>) -> FnTypes<TFnOnce, TFnMut, TFn, TFnClone> where 't: 'a {
//         todo!()
//     }
// }
