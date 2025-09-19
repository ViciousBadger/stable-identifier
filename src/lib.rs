pub mod prelude {
    pub use super::{
        domain::{
            GenerateIdStateful, GenerateIdStateless, IdDomain, IdDomainWithStatefulGenerator,
            IdDomainWithStatelessGenerator,
        },
        id::StableId,
        identify::{IdentifyAs, StableTypeId},
    };

    #[cfg(feature = "bevy")]
    pub use super::bevy::IdAppExt;
}

pub mod nanoid {
    use super::{
        domain::{GenerateIdStateless, IdDomain},
        id::StableId,
    };

    pub struct NanoIdGen;
    impl<D: IdDomain<Backing = String>> GenerateIdStateless<D> for NanoIdGen {
        fn generate_id() -> StableId<D> {
            StableId::new(nanoid::nanoid!())
        }
    }
}

mod domain;
mod id;
mod identify;

#[cfg(feature = "bevy")]
mod bevy;
