use super::{domain::IdDomain, id::Id};

/// This trait lets you define stable identifiers of a specified [`IdDomain`] for types.
/// The implementor gets to choose their own identifier that is assumed to be unique.
///
/// ## Use case
/// Like with [`std::any::TypeId`], implementing this trait allows you to identify a certain type.
/// Unlike `TypeId`, the provided identifier is "stable"; types can be safely moved and renamed,
/// system architecture can change and so on, but the identifier remains the same until you change it!
///
/// ## Requirements
///
/// `IdDomain` **must** provide a `ConstRepr` such that `D::Backing` implements `From<D::ConstRepr>`.
pub trait StableTypeId<D>
where
    D: IdDomain,
    D::Backing: From<D::ConstRepr>,
{
    /// An constant ID representation of this type.
    /// It is assumed that each type identifer is unique within their domain.
    const STABLE_TYPE_ID: D::ConstRepr;

    /// Get the type ID associated with this concrete type.
    fn stable_type_id() -> Id<D> {
        Id::new(Self::STABLE_TYPE_ID.into())
    }
}

/// Used to identify `Self` by some identifier in domain `D`.
pub trait IdentifyAs<D: IdDomain> {
    /// Gets a stable identifier that can be used to identify this value.
    /// In case the type has multiple identities, the domain will be inferred by pattern matching.
    fn identify_as(&self) -> Id<D>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_of_types() {
        struct Tool;
        impl IdDomain for Tool {
            const NAME: &'static str = "Tool";
            type Backing = String;
            type Generator = ();
            type ConstRepr = &'static str;
        }

        struct Saw;
        impl StableTypeId<Tool> for Saw {
            const STABLE_TYPE_ID: &'static str = "saw";
        }

        struct Hammer;
        impl StableTypeId<Tool> for Hammer {
            const STABLE_TYPE_ID: &'static str = "hammer";
        }

        struct Shovel;
        impl StableTypeId<Tool> for Shovel {
            const STABLE_TYPE_ID: &'static str = "shovel";
        }

        let saw_id = Saw::stable_type_id();
        let hammer_id = Hammer::stable_type_id();
        let shovel_id = Shovel::stable_type_id();

        assert_ne!(saw_id, hammer_id);
        assert_ne!(saw_id, shovel_id);
        assert_ne!(hammer_id, shovel_id);
    }
}
