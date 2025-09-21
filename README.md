A small Rust crate that provides a convenient abstraction for working with identifiers of various types.

# Motivation
This library came about as a result of experimenting with ways to implement the "newtype wrappers" for identifiers
in order to make using the wrong identifier type a compiler error. When working with software with
lots of persistent data types, this pattern helps declaring intent using the type system, so that you 
don't have to think too hard about what that one random Uuid really represents - if it's an `Id<Dog>`,
it's clearly something that identifies a dog - no need to even know the backing type in most cases.

---

The main use case of this crate is to define your own "identifier domains" (categories of
things that can be identified) using the `IdDomain` trait. `IdDomain` can be implemented for
almost any type or even just a marker struct if you don't have relevant type available already.

You can use any suitable type as a "backing" type for identifiers, be it `String`, `Uuid`...
the optional feature `tiny_id` provides `TinyId`, a simple fixed-size string-like identifier,
serving as both an example of a backing type as well as a decent option if you don't need anything fancy.

Other minor features include:
- Providing random generation of identifiers in your domain
- Providing a const representation of identifiers in your domain
- Extracting identifiers of types using the `IdentifyAs` trait
- Assigning identifiers to types using the `StableTypeId` trait

## Optional features
//! - `serde` lets you serialize and deserialize [`Id<T>`], as long as the backing type also implements these traits.
//! - `tiny_id` provides a barebones implementation of a concrete backing type that can be used if you just want a quick and easy identifier, with random ID generation using [`nanoid`](https://docs.rs/nanoid).

# Is it production ready?

Likely not, it's just my best shot at this pattern. There is very little code here, you can easily copy it into your own crate
to extend it or change some design decision.

# Contributing

`stable_identifier` is an extemely simple crate and I am open to any contributions or ideas for improvements, as long as they are within the scope of stable identifiers.
One thing in particular is that the `Id<T>` type does not have any trait implementations for other libraries in the ecosystem besides `serde`;
having more of these impls definitely wouldn't hurt and saves the user from having to maintain their own wrapper type.
