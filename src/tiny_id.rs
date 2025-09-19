//! Tiny, simple stack-allocated string that can be used as a backing identifier.
//!
//! Good enough for simple use cases, cheap and ergonomic to copy.
//! Comes with a just-as-tiny ID generator, free of charge!
use std::{convert::Infallible, str::FromStr};

use crate::{GenerateIdStateless, Id, IdDomain};

/// Constant-size backing type for string-based identifiers.
/// The fixed size makes it allocation-free and cheap to copy.
///
/// It is expected that the bytes always make up a valid UTF8 string.
///
/// The length of the identifier can be changed with the `N` associated constant
/// and defaults to 21 bytes/characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TinyId<const N: usize = 21> {
    text: [u8; N],
}

impl<const N: usize> TinyId<N> {
    /// Build ID from a byte slice, assumed to be valid utf-8.
    ///
    /// Will be trunctated to at most N bytes.
    /// If there are fewer bytes than the length of the array,
    /// the rest is set to `0u8` (null character).
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0u8; N];
        let copy_len = bytes.len().min(N);
        array[..copy_len].copy_from_slice(&bytes[..copy_len]);
        Self { text: array }
    }

    /// Byte representation of this ID.
    pub fn as_bytes(&self) -> &[u8] {
        &self.text
    }

    /// String representation of this ID. Will panic if the internal bytes do not make up valid utf-8!
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes())
            .expect("TinyId must not be created from invalid utf-8 !")
            .trim_end_matches('\0')
    }

    /// Counts up until the first `0u8` (null character).
    pub fn len(&self) -> usize {
        self.text.iter().take_while(|byte| byte != &&0).count()
    }

    /// Considered empty if the very first byte is `0u8`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> FromStr for TinyId<N> {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TinyId::from_bytes(s.as_bytes()))
    }
}

impl<const N: usize> std::fmt::Display for TinyId<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a, const N: usize> From<&'a str> for TinyId<N> {
    fn from(value: &'a str) -> Self {
        TinyId::from_str(value).unwrap()
    }
}

impl<const N: usize> AsRef<str> for TinyId<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use serde::{Deserialize, Serialize};

    use super::*;

    impl<const N: usize> Serialize for TinyId<N> {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de, const N: usize> Deserialize<'de> for TinyId<N> {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Ok(Self::from_str(&s).unwrap())
        }
    }
}

/// Allows generating [`TinyId`] identifiers using the [`nanoid`](https://docs.rs/nanoid) crate.
/// The generated string length is configurable with `N` and defaults to 21 characters.
pub struct TinyIdGen<const N: usize = 21>;
impl<const N: usize, D> GenerateIdStateless<D> for TinyIdGen<N>
where
    D: IdDomain<Backing = TinyId<N>>,
{
    fn generate_id() -> Id<D> {
        Id::new(TinyId::from_str(&nanoid::nanoid!(N)).unwrap())
    }
}

#[macro_export]
/// Convenience macro to declare a type as being an identifier domain using TinyId as a backing type.
macro_rules! tiny_id_domain {
    // Standard length
    ($domain_type:ident, $name:literal) => {
        impl $crate::IdDomain for $domain_type {
            const NAME: &'static str = $name;
            type Backing = $crate::tiny_id::TinyId;
            type Generator = $crate::tiny_id::TinyIdGen;
            type ConstRepr = &'static str;
        }
    };

    // Custom length
    ($domain_type:ident, $name:literal, $length:literal) => {
        impl $crate::IdDomain for $domain_type {
            const NAME: &'static str = $name;
            type Backing = $crate::tiny_id::TinyId<$length>;
            type Generator = $crate::tiny_id::TinyIdGen<$length>;
            type ConstRepr = &'static str;
        }
    };
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn has_correct_length() {
        struct Bird;
        impl IdDomain for Bird {
            const NAME: &'static str = "Bird";
            type Backing = TinyId<16>;
            type Generator = TinyIdGen<16>;
            type ConstRepr = ();
        }

        let new_bird_id = Bird::generate_id();
        assert_eq!(new_bird_id.backing().len(), 16);
    }

    #[test]
    fn len_works() {
        struct Bird;
        impl IdDomain for Bird {
            const NAME: &'static str = "Bird";
            type Backing = TinyId<16>;
            type Generator = ();
            type ConstRepr = ();
        }

        let new_bird_id = Bird::new_id("short");
        assert_eq!(new_bird_id.backing().len(), 5);
    }

    #[test]
    fn macro_works() {
        struct Bird;
        tiny_id_domain!(Bird, "Bird");

        let new_bird_id = Bird::generate_id();
        assert_eq!(new_bird_id.backing().len(), 21);

        const CONST_BIRD: &str = "constant";
        let unconst_bird = Bird::new_id(CONST_BIRD);
        assert_eq!(unconst_bird.backing().as_str(), "constant");
    }
}
