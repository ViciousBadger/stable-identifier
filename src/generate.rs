use crate::{Id, IdDomain};

/// Allows a type to generate identifiers in a 'stateless' manner.
pub trait GenerateIdStateless<D: IdDomain> {
    /// Generates a new stable identifier.
    fn generate_id() -> Id<D>;
}

/// Allows a type to generate identifiers in a 'stateful' manner.
///
/// Useful when the act of generating IDs has side effects, like incrementing an ID counter.
pub trait GenerateIdStateful<D: IdDomain> {
    /// Generates a new stable identifier using the state of `self`.
    fn generate_id_stateful(&mut self) -> Id<D>;
}
