
macro_rules! impl_trait_for {
    (unsafe $trait:path => $($type:ty),+ $(,)?) => {
        $(unsafe impl $trait for $type {

        }
        )+
    };
    ($trait:path => $($type:ty),+ $(,)?) => {
        $(impl $trait for $type {

        }
        )+
    };
}

#[allow(unused)]
macro_rules! impl_hkt_classification_for {
    ($choice:ty: $tinner:ident => $($type:ty),+ $(,)?) => {
        $(impl<$tinner> crate::hkt::traits::hkt_classification::HktClassification for $type {
            type Choice = $choice;
        }
        )+
    };
}

#[allow(unused)]
macro_rules! impl_trait_for_wrapper {
    (unsafe $trait:path : $tinner:ident => $($type:ty),+ $(,)?) => {
        $(unsafe impl<$tinner : $trait> $trait for $type {

        }
        )+
    };
    ($trait:path : $tinner:ident => $($type:ty),+ $(,)?) => {
        $(impl<$tinner : $trait> $trait for $type {

        }
        )+
    };
}

/// Generate a match expression, create an identifier for each branch and copies the expression to each with #[allow(unused_mut)] applied.
macro_rules! match_one_of_5 {
    ($enum:expr, $f:ident, $expression:expr) => {
        match $enum {
            #[allow(unused_mut)]
            OneOf5::T1(mut $f) => $expression,
            #[allow(unused_mut)]
            OneOf5::T2(mut $f) => $expression,
            #[allow(unused_mut)]
            OneOf5::T3(mut $f) => $expression,
            #[allow(unused_mut)]
            OneOf5::T4(mut $f) => $expression,
            #[allow(unused_mut)]
            OneOf5::T5(mut $f) => $expression,
        }
    };
}

/// Call map_one_5_with with given type and first expression and copies the second expression to each branch with #[allow(unused_mut)] applied.
macro_rules! map_one_of_5_with {
    ($type:ty, $f:expr, $expression:expr) => {
        <$type>::map_one_of_5(
            $f,
            #[allow(unused_mut)]
            $expression,
            #[allow(unused_mut)]
            $expression,
            #[allow(unused_mut)]
            $expression,
            #[allow(unused_mut)]
            $expression,
            #[allow(unused_mut)]
            $expression,
        )
    };
}

