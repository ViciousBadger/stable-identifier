use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

use super::{domain::IdDomain, id::StableId};

pub trait IdAppExt {
    fn register_stable_id<D>(&mut self)
    where
        D: IdDomain + TypePath,
        D::Backing:
            std::fmt::Display + Send + Sync + TypePath + Serialize + for<'de> Deserialize<'de>;
}

impl IdAppExt for bevy::app::App {
    /// Register a stable identifier domain to be known by the Bevy type registry. This is
    /// nessecary in order to correctly serialize and deserialize `StableId<D>` using
    /// reflection, as well as to show IDs properly in the egui inspector if the `inspector`
    /// feature is enabled.
    fn register_stable_id<D>(&mut self)
    where
        D: IdDomain + TypePath,
        D::Backing:
            std::fmt::Display + Send + Sync + TypePath + Serialize + for<'de> Deserialize<'de>,
    {
        self.register_type::<StableId<D>>();

        #[cfg(feature = "inspector")]
        {
            // Register an Id as an inspector primitive to get much nicer displaying in Egui.
            use bevy_inspector_egui::inspector_egui_impls::InspectorEguiImpl;
            self.register_type_data::<StableId<D>, InspectorEguiImpl>();
        }
        {
            use bevy::reflect::{ReflectDeserialize, ReflectSerialize};
            self.register_type_data::<StableId<D>, ReflectSerialize>();
            self.register_type_data::<StableId<D>, ReflectDeserialize>();
        };
    }
}

#[cfg(test)]
mod tests {
    use bevy::reflect::{Reflect, TypePath};

    use crate::{nanoid::NanoIdGen, prelude::*};

    #[test]
    fn generate_and_identify() {
        // This WORKS!?!?!??!!?
        // Dog is both data container and IdConfig..
        #[derive(Reflect)]
        struct Dog {
            id: DogId,
        }
        type DogId = StableId<Dog>;
        impl IdDomain for Dog {
            const NAME: &'static str = "Dog";
            type Backing = String;
            type Generator = NanoIdGen;
            type ConstRepr = &'static str;
        }

        // dog = dog
        // woof
        impl IdentifyAs<Dog> for Dog {
            fn identify_as(&self) -> DogId {
                self.id.clone()
            }
        }

        let dog = Dog {
            id: Dog::generate_id(),
        };

        let dog_id = dog.identify_as();
        assert_eq!(dog_id, dog.id);

        // ensuring that StableId is Reflect when the inner type is!
        DogId::type_path();
    }

    #[test]
    fn basic_id() {
        #[derive(Reflect)]
        struct Basic {
            id: StableId<Basic>,
        }
        impl IdDomain for Basic {
            const NAME: &'static str = "basic";
            type Backing = String;
            type Generator = ();
            type ConstRepr = ();
        }

        // --- this still works!
        let basic_id = Basic::new_id("hello world".to_string());
        assert_eq!(basic_id, Basic::new_id("hello world".to_string()))

        // --- this does not exist!
        // let basic_id = Basic::generate_id();
        //
        // --- this will not compile because String cannot be created from ()!
        // struct Dumber;
        //
        // impl StableTypeId for Dumber {
        //     type Config = Dumb;
        //     const STABLE_TYPE_ID: <Self::Config as IdConfig>::ConstRepr = ();
        // }
    }

    #[test]
    fn domain_bounds() {
        struct Dog;
        impl IdDomain for Dog {
            const NAME: &'static str = "Dog";
            type Backing = String;
            type Generator = NanoIdGen;
            type ConstRepr = ();
        }
        struct Cat;
        impl IdDomain for Cat {
            const NAME: &'static str = "Cat";
            type Backing = String;
            type Generator = NanoIdGen;
            type ConstRepr = ();
        }
        let dog_id = Dog::new_id("hans".to_string());
        let cat_id = Cat::new_id("hans".to_string());
        assert_eq!(dog_id, dog_id);
        assert_eq!(cat_id, cat_id);
        // compiler does not allow eq because StableId<Dog> and StableId<Cat> are different types :)
        // assert_eq!(dog_id, cat_id);
    }

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
