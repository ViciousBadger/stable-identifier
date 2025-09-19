use super::id::StableId;

/// This trait allows a type to act as a domain for identifiers (`StableId`) by providing
/// a concrete backing ID type, along with extra types that specify optional behaviour of this
/// identifier, these can be left as () is the behaviour is not desired.
pub trait IdDomain {
    /// A presentable name for the ID domain, useful in debugging.
    const NAME: &'static str;

    /// The type to use as a concrete ID data structure. `StableId<Self>` will automatically
    /// implement any useful traits implemented by this inner backing type such as `Eq`, `Ord`,
    /// `Hash` and serde's `Serialize`/`Deserialize`.
    ///
    /// For randomly generated IDs, `Uuid` and `Ulid` are great choices as a backing type.
    /// For static IDs used for internals like type identification, a plain `String` can be good enough.
    type Backing: Clone + Eq + std::hash::Hash; // Must be Clone to satisfy bevy's opaque Reflect

    /// An optional type that can be used to generate new identifiers in this domain. Can be
    /// any type that implements either `GenerateIdStateless` or `GenerateIdStateful`.
    ///
    /// For most cases, you can declare an empty struct and use it as a stateless ID generator
    /// by implementing `GenerateIdStateless`. However, in some cases like with `Ulid`, an
    /// unique stateful generator is provided (`ulid::Generator`) that ensures IDs are properly ordered.
    type Generator;

    /// An optional type to use for const representations of the `Backing` type. This is
    /// useful when you want to identify something in your source code, for example types
    /// implementing some common trait using the `StableTypeId` trait. For it to work,
    /// `Self::Backing` must implement `From<Self::ConstRepr>`.
    type ConstRepr;

    fn new_id(from_value: Self::Backing) -> StableId<Self>
    where
        // NOTE: probably unnessecary bound, but how to reove?
        Self: Sized,
    {
        StableId::new(from_value)
    }
}

pub trait IdDomainWithStatelessGenerator<D: IdDomain>: std::marker::Sized {
    fn generate_id() -> StableId<D>;
}

pub trait GenerateIdStateless<D: IdDomain> {
    fn generate_id() -> StableId<D>;
}

impl<D: IdDomain> IdDomainWithStatelessGenerator<D> for D
where
    D::Generator: GenerateIdStateless<D>,
{
    fn generate_id() -> StableId<D> {
        D::Generator::generate_id()
    }
}

pub trait IdDomainWithStatefulGenerator<D: IdDomain>: std::marker::Sized {
    fn generate_id(generator: &mut D::Generator) -> StableId<D>;
}

pub trait GenerateIdStateful<D: IdDomain> {
    fn generate_id(&mut self) -> StableId<D>;
}

impl<D: IdDomain> IdDomainWithStatefulGenerator<D> for D
where
    D::Generator: GenerateIdStateful<D>,
{
    fn generate_id(generator: &mut D::Generator) -> StableId<D> {
        generator.generate_id()
    }
}
