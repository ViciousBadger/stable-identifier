use std::{fmt::Display, hash::Hash};

use super::domain::IdDomain;

/// A container type for an unique identifier of an object in domain `D`.
/// The type will automatically inherit most useful traits of the inner backing ID type determined by the domain.
/// If you want, you can create a convenient type alias for identifiers in your domain:
/// ```
/// use stable_identifier::prelude::*;
///
/// struct Dog;
/// impl IdDomain for Dog {
///     const NAME: &'static str = "Dog";
///     type Backing = String;
///     type Generator = ();
///     type ConstRepr = ();
/// }
/// type DogId = StableId<Dog>;
/// ```
/// If using serde, `StableId` will serialize directly as the inner type without any alteration.
pub struct StableId<D: IdDomain> {
    backing: D::Backing,
}

impl<D: IdDomain> StableId<D> {
    pub fn backing(&self) -> &D::Backing {
        &self.backing
    }

    pub fn into_backing(self) -> D::Backing {
        self.backing
    }
}

impl<D: IdDomain> AsRef<D::Backing> for StableId<D> {
    fn as_ref(&self) -> &D::Backing {
        self.backing()
    }
}

impl<D: IdDomain> std::fmt::Debug for StableId<D>
where
    D::Backing: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StableId")
            .field("domain", &D::NAME)
            .field("id", &self.backing)
            .finish()
    }
}

impl<D: IdDomain> PartialEq for StableId<D>
where
    D::Backing: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.backing == other.backing
    }
}

impl<D: IdDomain> Eq for StableId<D> where D::Backing: Eq {}

impl<D: IdDomain> PartialOrd for StableId<D>
where
    D::Backing: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.backing.partial_cmp(&other.backing)
    }
}

impl<D: IdDomain> Ord for StableId<D>
where
    D::Backing: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.backing.cmp(&other.backing)
    }
}

impl<D: IdDomain> Hash for StableId<D>
where
    D::Backing: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.backing.hash(state);
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::{domain::IdDomain, id::StableId};
    use serde::{Deserialize, Serialize};

    impl<D: IdDomain> Serialize for StableId<D>
    where
        D::Backing: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.backing.serialize(serializer)
        }
    }

    impl<'de, D: IdDomain> Deserialize<'de> for StableId<D>
    where
        D::Backing: Deserialize<'de>,
    {
        fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
        where
            De: serde::Deserializer<'de>,
        {
            Ok(StableId {
                backing: D::Backing::deserialize(deserializer)?,
            })
        }
    }
}

impl<D: IdDomain> Clone for StableId<D>
where
    D::Backing: Clone,
{
    fn clone(&self) -> Self {
        StableId {
            backing: self.backing.clone(),
        }
    }
}

impl<D: IdDomain> Copy for StableId<D> where D::Backing: Copy {}

impl<D: IdDomain> Display for StableId<D>
where
    D::Backing: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", D::NAME, self.backing)
    }
}

impl<D: IdDomain> StableId<D> {
    pub const fn new(value: D::Backing) -> Self {
        Self { backing: value }
    }
}
