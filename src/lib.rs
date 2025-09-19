//! Provides a convenient abstraction for working with identifiers of various types.
//!
//! The main use case of this crate is to define your own "identifier domains" (categories of
//! things that can be identified) using the [`IdDomain`] trait. `IdDomain` can be implemented for
//! almost any type or even just a marker struct if you don't have relevant type available already.
//!
//! To implement [`IdDomain`] you provide a `Backing` type that will be used to store the actual
//! identifier type of your choice. You can then use [`Id<T>`] to construct identifiers of that
//! type, scoped to the domain you have defined. The generic argument acts as a compile-time check
//! that you are not mixing up identifiable domains and helps clarify the intent between
//! identifiers with minimal boilerplate.
//!
//! Other minor features include:
//! - Providing random generation of identifiers in your domain
//! - Providing a const representation of identifiers in your domain
//! - Extracting identifiers of types using the [`IdentifyAs`] trait
//! - Assigning identifiers to types using the [`StableTypeId`] trait
//!
//! ## Optional features
//! - `serde` lets you serialize and deserialize [`Id<T>`], as long as the backing type also implements these traits.
//! - `tiny_id` provides a barebones implementation of a concrete backing type that can be used if you just want a quick and easy identifier, with random ID generation using [`nanoid`](https://docs.rs/nanoid).

// It's a small crate so might as well flatten the module hierachy.
pub use {
    domain::IdDomain,
    generate::{GenerateIdStateful, GenerateIdStateless},
    id::Id,
    identify::{IdentifyAs, StableTypeId},
};

mod domain;
mod generate;
mod id;
mod identify;

#[cfg(feature = "tiny_id")]
pub mod tiny_id;
