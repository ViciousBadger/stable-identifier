use super::{domain::IdDomain, id::StableId};

/// This trait lets you define stable identifiers of a specified `IdDomain` for types.
/// The implementors get to choose their own identifier that is assumed to be unique.
/// The ID has similar use cases as `std::any::TypeId` but works in a more reliable and controlled manner.
pub trait StableTypeId<D>
where
    D: IdDomain,
    D::Backing: From<D::ConstRepr>,
{
    /// An constant ID representation of this type.
    /// It is assumed that each type identifer is unique within their domain.
    const STABLE_TYPE_ID: D::ConstRepr;

    /// Get the type ID associated with this concrete type.
    fn stable_type_id() -> StableId<D> {
        StableId::new(Self::STABLE_TYPE_ID.into())
    }
}

/// Used to identify `Self` by some identifier in domain `D`.
pub trait IdentifyAs<D: IdDomain> {
    /// Gets a stable identifier that can be used to identify this value.
    /// In case the type has multiple identities, the domain will be inferred by pattern matching.
    ///
    /// # Example
    /// ```
    /// let thing_id: StableId<Thing> = some_thing.identify_as();
    /// ```
    fn identify_as(&self) -> StableId<D>;
}
