# Overengineered Higher-kinded Types

![Static Badge](https://img.shields.io/badge/maintenance-expiremental-yellow)

Expiremental crate of higher-kinded types integrated with lifetimes and matching function types and optional cloning and supporting no_std.

## Higher-kinded types base traits over lifetimes

There 4 base Hkts with the base root as an example:

```
/// `'t` is some arbitrary bound that always outlive every other bound in the hkt so that it can be used as a bound in some places where `'static` is the only other option
pub trait UnsizedHkt<'t>: 't {
    /// Definition: F<'a, A: 't>: 'a (where 't: 'a is logical but maybe unnecessary)
    ///
    /// Requirements:
    /// - Invariant over 'a and A and 't
    type UnsizedF<'a, A: 'a>: 'a + ?Sized
    where
        't: 'a;
}
```

Each can be succintly described as:

- UnsizedHkt: A is Sized, Self::F<'a, A> is ?Sized
- Hkt: A is Sized, Self::F<'a, A> is Sized, derives UnsizedHkt
- UnsizedHktUnsized: A is ?Sized, Self::F<'a, A> is ?Sized, derives UnsizedHkt
- HktUnsized: A is ?Sized, Self::F<'a, A> is Sized, derives UnsizedHktUnsizedHkt + Hkt

Beware that although all type implementing these traits but share the same Self::F<'a, A> between implementations, that invariant is currently not enforced at compile time between hkts where A is ?Sized and ones where A is Sized.

Out of the 4 traits, `Hkt` receives the most attention for obvious reasons.

Here's an example of a type implementing all traits:

```rust
pub struct BoxT<TInner = IdHkt>(Infallible, PhantomData<TInner>);

impl<'t, TInner: UnsizedHkt<'t>> Hkt<'t> for BoxT<TInner> {
    type F<'a, A: 'a>
        = Box<TInner::UnsizedF<'a, A>>
    where
        't: 'a;
}

impl<'t, TInner: UnsizedHktUnsized<'t>> HktUnsized<'t> for BoxT<TInner> {
    type FUnsized<'a, A: 'a + ?Sized>
        = Box<TInner::UnsizedFUnsized<'a, A>>
    where
        't: 'a;
}

impl<TInner> HktClassification for BoxT<TInner> {
    type Choice = hkt_classification::OuterHkt;
}
```

The hkt being generic over the inner hkt allows hkt stacking to describe pretty any types as `Self::F<'a, A>`. Implementing HktClassification with `Choice = hkt_classification::OuterHkt` will add blanket implementations of `UnsizedHkt` and `UnsizedHktUnsized` too - though the other `Choice` types are currently unused.

## Multi function types matching (OneOfN)

**TODO**

## Transmuting Hkts

Due to how Rust memory layout is implemented, transmuting between Self::F<'a, A> and Self::F<'a, B> is not safe even if A can be transmuted to B due to different layout monomorphisation. However transmutation is always safe if it is known at compile time that A == B, since objects of the same `Sized` type obviously have the same memory layout.

## Clone guarded matching

**TODO**

## Known issues

- Currently there is no known way of enforcing type equality between hkt traits receiving Sized and hkt traits receiving ?Sized
- Ergonomics is bad - *really* bad, and you can check on the laws implementations to see how unergonomic it is - and it's still being worked on.
- The compiler is significantly less helpful at type inference since requirements are placed first and args and are prioritize in type inference; so the compiler is likely to offer misleading lints if the wrong requirements are used.
- No way to specify your Hkt does not require cloning at all - currently only whether passing a clone func is necessary to clone, and you still need to specify TCloneableOf5 and CloneK even if no cloning is needed - still being worked on

## Disclosure

This library was inspired by the `C#` library [`language-ext`](https://github.com/louthy/language-ext). Some implementations were transcribed from their `C#` implementation since I can't read Haskell.

## TODO

- Implement/Fix Bind\<Traversable\> for Iter & Future - and see if ReqIn and ReqOut can be different types for previous impls
- Implement optional Clone => 3 clone options: DependentClone, IndependentClone and NoClone => INSTEAD: independent cloning only make sense for nullary and is otherwise not useful.
- Implement DerefK and hkt traits for Arc
- Optimize FoldWhile and Rfoldwhile including (fold refs and muts) using as_mut()
- Better ergonomics: Add type-inferred methods and single-layer wrapper types (since raw hkt objects don't encode type information)
- Add no-alloc support
- Documentation
- Benchmark
- Readd support for any requirements rether than just cloning (closures won't be supported)
- Add more hkts
