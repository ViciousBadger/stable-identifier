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
    fn identify_as(&self) -> StableId<D>;
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
