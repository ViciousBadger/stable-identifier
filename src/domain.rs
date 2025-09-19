use crate::{GenerateIdStateful, GenerateIdStateless, Id};

/// Defines a "domain" of identifiable entities.
///
/// ## Purpose
///
/// This allows any concrete type to act as a generic argument for identifiers ([`Id`]) by providing
/// a concrete backing ID type, along with extra types that specify optional behaviour of this
/// identifier, these can be left as () is the behaviour is not desired.
pub trait IdDomain {
    /// A presentable name for the ID domain, useful in debugging.
    const NAME: &'static str;

    /// The type to use as a concrete data structure for identifiers in this domain.
    ///
    /// `Id<Self>` will automatically implement the following traits if they are implemented by this inner backing type:
    /// - [`Debug`]
    /// - [`std::fmt::Display`]
    /// - [`Clone`]
    /// - [`Copy`]
    /// - [`PartialEq`]
    /// - [`Eq`]
    /// - [`Hash`]
    /// - [`PartialOrd`]
    /// - [`Ord`]
    /// - [`serde::Serialize`]/[`serde::Deserialize`] (if `serde` feature is enabled).
    ///
    /// Some examples widely used backing types are [`Uuid`](https://docs.rs/uuid) and [`Ulid`](https://docs.rs/ulid) (sortable).
    ///
    /// You can of course just use [`String`], although it lacks the convenience of
    /// stack-allocated, [`Copy`]-able ID types.
    type Backing;

    /// A type that can be used to generate new identifiers in this domain. Can be
    /// any type that implements either [`GenerateIdStateless`] or [`GenerateIdStateful`].
    ///
    /// Can also be `()` if random ID generation is not desired.
    ///
    /// For most cases, you can declare an empty struct and use your algorithm of choice
    /// by implementing [`GenerateIdStateless`]. However, in some cases like with [`Ulid`](https://docs.rs/ulid), a
    /// stateful generator is provided (See [`ulid::Generator`](https://docs.rs/ulid/latest/ulid/struct.Generator.html)) that ensures IDs are properly ordered.
    type Generator;

    /// A type to use for `const` representations of the [`IdDomain::Backing`] type.
    ///
    /// Can be `()` if `const` representations are not deisred.
    ///
    /// Used by the [`crate::identify::StableTypeId`] trait to provide type identifiers.
    type ConstRepr;

    /// Construct a new identifier from a backing value.
    fn new_id(from_value: impl Into<Self::Backing>) -> Id<Self>
    where
        // NOTE: Self does not really have to be Sized as we are not constructing Self but
        // Self::Backing, however I am not sure how to remove this bound.
        Self: Sized,
    {
        Id::new(from_value.into())
    }

    /// Generate a new identifier.
    fn generate_id() -> Id<Self>
    where
        Self: Sized,
        Self::Generator: GenerateIdStateless<Self>,
    {
        Self::Generator::generate_id()
    }

    /// Generate an identifier using a given stateful generator.
    fn generate_id_stateful(generator: &mut Self::Generator) -> Id<Self>
    where
        Self: Sized,
        Self::Generator: GenerateIdStateful<Self>,
    {
        generator.generate_id_stateful()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_bounds() {
        struct Dog;
        impl IdDomain for Dog {
            const NAME: &'static str = "Dog";
            type Backing = String;
            type Generator = ();
            type ConstRepr = ();
        }
        struct Cat;
        impl IdDomain for Cat {
            const NAME: &'static str = "Cat";
            type Backing = String;
            type Generator = ();
            type ConstRepr = ();
        }
        let dog_id = Dog::new_id("hans".to_string());
        let cat_id = Cat::new_id("hans".to_string());
        assert_eq!(dog_id, dog_id);
        assert_eq!(cat_id, cat_id);

        // compiler does not allow eq because Id<Dog> and Id<Cat> are different types :)
        // assert_eq!(dog_id, cat_id);
    }
}
