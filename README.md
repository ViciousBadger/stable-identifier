A small Rust crate that provides a convenient abstraction for working with identifiers of various types.

The main use case of this crate is to define your own "identifier domains" (categories of
things that can be identified) using the `IdDomain` trait. `IdDomain` can be implemented for
almost any type or even just a marker struct if you don't have relevant type available already.

Other minor features include:
- Providing random generation of identifiers in your domain
- Providing a const representation of identifiers in your domain
- Extracting identifiers of types using the `IdentifyAs` trait
- Assigning identifiers to types using the `StableTypeId` trait
