use std::vec::Vec;

/// Query parameters. A vector of pairs (name, value) where
/// both name and value are strings. Params objects are produced
/// by the [crate::output::Output] and objects implementing the
/// [crate::search::QueryParams] trait.
pub type Params<'a> = Vec<(&'a str, String)>;

/// Marker for objects implementing the openfoodfacts API V0.
#[derive(Copy, Clone)]
pub struct V0;

/// Marker for objects implementing the openfoodfacts API V2.
#[derive(Copy, Clone)]
pub struct V2;

/// Version marker objects implement the Version trait. This serves
/// two purposes:
///
/// * Define generics bound to this trait.
/// * Produce the string representation of the version for use in
///   API URLs.
pub trait Version {
    /// Produce the string representation of the version supported
    /// by the implementing type.
    fn version(&self) -> &str;
}

impl Version for V0 {
    fn version(&self) -> &str {
        "v0"
    }
}

impl Version for V2 {
    fn version(&self) -> &str {
        "v2"
    }
}
