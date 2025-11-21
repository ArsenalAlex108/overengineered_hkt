use std::num::NonZeroUsize;
use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};

struct Generate {
    count: NonZeroUsize,
}

impl Generate {
    fn generate_string(&self, string_fn: impl FnMut(usize) -> String) -> String {
        (1..=self.count.into())
            .map(string_fn)
            .reduce(|sum, next| sum + &next)
            .unwrap()
    }

    fn generate_string_with_head(
        &self,
        head: String,
        string_fn: impl FnMut(usize) -> String,
    ) -> String {
        (2..=self.count.into())
            .map(string_fn)
            .fold(head, |sum, next| sum + &next)
    }

    // fn generate_string_with_seperator(
    //     seperator: String,
    //     string_fn: impl FnMut(i32) -> String,
    // ) -> String {
    //     (1..=count.into())
    //         .map(string_fn)
    //         .intersperse(seperator)
    //         .reduce(|sum, next| sum + &next)
    //         .unwrap()
    // }
}

#[proc_macro]
pub fn generate_one_of(count: TokenStream) -> TokenStream {
    let count_string = count.to_string();
    let count = NonZeroUsize::from_str(&count_string).unwrap();

    let generate = Generate { count };

    let mut buffer = String::new();
    let type_params = generate.generate_string(|i| format!("T{i}, "));
    let type_params_a = generate.generate_string(|i| format!(" T{i}a,"));
    let type_params_b = generate.generate_string(|i| format!(" T{i}b,"));
    let enum_variants = generate.generate_string(|i| format!(" T{i}(T{i}),"));

    let type_params_bounded = generate.generate_string(|i| format!(" T{i}: 'a,"));

    let type_params_bounded_a = generate.generate_string(|i| format!(" T{i}a: 'a,"));

    let type_params_bounded_b = generate.generate_string(|i| format!(" T{i}b: 'b,"));

    let type_params_bounded_clone = generate.generate_string(|i| format!(" T{i}: 'a + Clone,"));

    let type_params_ref = generate.generate_string(|i| format!("&'a T{i},"));

     let type_params_mut = generate.generate_string(|i| format!("&'a mut T{i},"));

    let map_funcs =
        generate.generate_string(|i| format!("map_t{i}: impl 'f + FnOnce(T{i}a) -> T{i}b,\n"));

    let t_s = generate.generate_string(|_| "T, ".to_string());

    let match_t = generate.generate_string_with_head(format!("OneOf{count}::T1(t)"), |i| {
        format!("\n| OneOf{count}::T{i}(t)")
    });

    let repeat_clone = generate.generate_string(|_|"|f| f.clone(),\n".to_string());

    buffer.push_str(
        &format!("
            use std::marker::PhantomData;
            use std::convert::Infallible;
            use std::clone::Clone;

            pub enum OneOf{count}<{type_params}> {{
                {enum_variants}
            }}

            /// Specific pattern indicating actual value is one of the given types.
            /// Types can either map over all types - or eliminate specific types by requiring specific traits proving actual value is not that type.
            pub trait OneOf{count}Hkt<'t> {{
                type OneOf{count}F<'a, {type_params_bounded}>: 'a where 't: 'a;

                /// Converts this object into a matchable enum.
                fn into_one_of_{count}_enum<'a, {type_params_bounded}>(
                    s: Self::OneOf{count}F<'a, {type_params}>
                ) -> OneOf{count}<{type_params}> where 't: 'a;

                /// Converts a mut reference to this object into an owned object with mut references to each variant,
                fn as_mut<'a, {type_params_bounded}>(
                    s: &'a mut Self::OneOf{count}F<'a, {type_params}>
                ) -> Self::OneOf{count}F<'a, {type_params_mut}> where 't: 'a;

                /// Converts a reference to this object into an owned object with references to each variant,
                fn as_ref<'a, {type_params_bounded}>(
                    s: &'a Self::OneOf{count}F<'a, {type_params}>
                ) -> Self::OneOf{count}F<'a, {type_params_ref}> where 't: 'a;

                /// Perform mapping on each type - but only exactly one of the provided functions will execute.
                fn map_one_of_{count}<'a, 'b, 'f,
                    {type_params_bounded_a}
                    {type_params_bounded_b}
                >(
                    s: Self::OneOf{count}F<'a, {type_params_a}>,
                    {map_funcs}
                ) -> Self::OneOf{count}F<'b, {type_params_b}>
                where
                    'a: 'f,
                    't: 'a + 'b;


                /// Unwrap into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_{count}_enum](OneOf{count}Hkt::into_one_of_{count}_enum), but a more efficient implementation may be provided instead.
                fn unwrap_all<'a, T: 'a>(s: Self::OneOf{count}F<'a, {t_s}>) -> T
                where
                    't: 'a,
                {{
                    match Self::into_one_of_{count}_enum(s) {{
                        {match_t} => t,
                    }}
                }}

                /// Create a new object from an existing one to store a different value.
                fn create_from<'a, {type_params_bounded} T: 'a>(
                    old: &Self::OneOf{count}F<'a, {type_params}>,
                    value: T,
                ) -> Self::OneOf{count}F<'a, {t_s}>
                where
                    't: 'a;

                /// Create a new object from an existing one to store a different value.
                fn clone_one_of_{count}<'a, 'b, {type_params_bounded_clone}>(
                    old: &'b Self::OneOf{count}F<'a, {type_params}>,
                ) -> Self::OneOf{count}F<'a, {type_params}>
                where
                    'a: 'b,
                    't: 'a;

            }}
        ")
                // TODO: make this work:
                // {{
                //     Self::map_one_of_{count}(
                //         Self::as_ref(old),
                //         {repeat_clone}
                //     )
                // }}
    );

    buffer.push_str(&generate.generate_string(|i| {
        let type_params_replaced = generate.generate_string(|j| {
            if i == j {
                format!(" T{j}b,")
            } else {
                format!(" T{j}a,")
            }
        });

        format!(
            "
            /// Indicates that the Hkt does not contains an instannce of `T{i}`
            pub trait NotT{i}of{count}<'t>: OneOf{count}Hkt<'t> {{
                /// Set `T{i}` to an arbitrary type parameter `T{i}b`.
                fn arbitrary_t{i}<'a, {type_params_bounded_a} T{i}b: 'a>(
                    s: Self::OneOf{count}F<'a, {type_params_a}>,
                    _type_infer: PhantomData<T{i}b>,
                ) -> Self::OneOf{count}F<'a, {type_params_replaced}>
                where
                    't: 'a;
            }}

        "
        )
    }));

    for i in 1..=count.into() {
        buffer.push_str(
            &format!("
                pub struct T{i}Of{count}Hkt(Infallible);

                impl<'t> OneOf{count}Hkt<'t> for T{i}Of{count}Hkt {{
                    type OneOf{count}F<'a, {type_params_bounded}> = T{i} where 't: 'a;

                    /// Converts this object into a matchable enum.
                    fn into_one_of_{count}_enum<'a, {type_params_bounded}>(
                        s: Self::OneOf{count}F<'a, {type_params}>
                    ) -> OneOf{count}<{type_params}> where 't: 'a {{
                        OneOf{count}::T{i}(s)
                    }}

                    /// Converts a mut reference to this object into an owned object with mut references to each variant,
                    fn as_mut<'a, {type_params_bounded}>(
                        s: &'a mut Self::OneOf{count}F<'a, {type_params}>
                    ) -> Self::OneOf{count}F<'a, {type_params_mut}> where 't: 'a {{ 
                        s
                    }}

                    /// Converts a reference to this object into an owned object with references to each variant,
                    fn as_ref<'a, {type_params_bounded}>(
                        s: &'a Self::OneOf{count}F<'a, {type_params}>
                    ) -> Self::OneOf{count}F<'a, {type_params_ref}> where 't: 'a {{
                        s
                    }}

                    /// Perform mapping on each type - but only exactly one of the provided functions will execute.
                    fn map_one_of_{count}<'a, 'b, 'f,
                        {type_params_bounded_a}
                        {type_params_bounded_b}
                    >(
                        s: Self::OneOf{count}F<'a, {type_params_a}>,
                        {map_funcs}
                    ) -> Self::OneOf{count}F<'b, {type_params_b}>
                    where
                        'a: 'f,
                        't: 'a + 'b {{
                        map_t{i}(s)
                    }}

                    /// Unwrap into `T` when all variants have the same type. The default implementation uses [Self::into_one_of_{count}_enum](OneOf{count}Hkt::into_one_of_{count}_enum), but a more efficient implementation may be provided instead.
                    fn unwrap_all<'a, T: 'a>(s: Self::OneOf{count}F<'a, {t_s}>) -> T
                    where
                        't: 'a,
                    {{
                        s
                    }}

                    /// Create a new object from an existing one to store a different value.
                    fn create_from<'a, {type_params_bounded} T: 'a>(
                        _: &Self::OneOf{count}F<'a, {type_params}>,
                        value: T,
                    ) -> Self::OneOf{count}F<'a, {t_s}>
                    where
                        't: 'a {{
                        value
                    }}

                    /// Create a new object from an existing one to store a different value.
                    fn clone_one_of_{count}<'a, 'b, {type_params_bounded_clone}>(
                        old: &'b Self::OneOf{count}F<'a, {type_params}>,
                    ) -> Self::OneOf{count}F<'a, {type_params}>
                    where
                        'a: 'b,
                        't: 'a {{
                        old.clone()
                    }}
                }}
            ")
        );

        for j in 1..=count.into() {
            if i == j {
                continue;
            }

            let type_params_replaced = generate.generate_string(|k| {
                if k == j {
                    format!(" T{k}b,")
                } else {
                    format!(" T{k}a,")
                }
            });

            buffer.push_str(&format!(
                "
                    impl<'t> NotT{j}of{count}<'t> for T{i}Of{count}Hkt {{
                        /// Set `T{i}` to an arbitrary type parameter `T{i}b`.
                        fn arbitrary_t{j}<'a, {type_params_bounded_a} T{j}b: 'a>(
                            s: Self::OneOf{count}F<'a, {type_params_a}>,
                            _type_infer: PhantomData<T{j}b>,
                        ) -> Self::OneOf{count}F<'a, {type_params_replaced}>
                        where
                            't: 'a {{
                            s
                        }}
                    }}

                "
            ))
        }
    }

    TokenStream::from_str(&buffer).unwrap()
}

#[proc_macro]
pub fn generate_one_of_macros(count: TokenStream) -> TokenStream {
    use std::num::NonZeroUsize;
    use std::str::FromStr;

    let count_string = count.to_string();
    let count = NonZeroUsize::from_str(&count_string).unwrap();

    let generate = Generate { count };

    let mut buffer = String::new();

    buffer.push_str(&
    format!("
        /// Generate a match expression, create an identifier for each branch and copies the expression to each.
        macro_rules! match_one_of_{count} {{
            ($enum:expr, $f:ident, $expression:expr) => {{
                match $enum {{
                    {}
                }}
            }};
        }}
    ", generate.generate_string(|i| format!("
    #[allow(unused_mut)]\n
    OneOf{count}::T{i}(mut $f) => $expression,\n"))));

    TokenStream::from_str(&buffer).unwrap()
}

#[proc_macro]
/// Expected input format: !(count, expression)
pub fn repeat_expr(input: TokenStream) -> TokenStream {
    use std::num::NonZeroUsize;
    use std::str::FromStr;

    let mut token_stream = TokenStream::new();
    let mut input_iter = input.into_iter();

    if let Some(TokenTree::Literal(i)) = input_iter.next() {
        token_stream.extend(input_iter.skip(1));
        let count_string = i.to_string();
        let count = NonZeroUsize::from_str(&count_string).unwrap();

        let generate = Generate { count };

        let mut buffer = generate.generate_string(|_| token_stream.clone().to_string() + ",\n");

        TokenStream::from_str(&buffer).unwrap()
    } else {
        panic!("Invalid format, expected input format: !(count, expression)")
    }
}
