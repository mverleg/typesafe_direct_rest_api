
// TODO @mverleg: should this be binary instead of strings?
// TODO @mverleg: how to set the serialization format?

use serde::Serialize;
use serde::Deserialize;

/// Trait for types that can be transferred through the API.
pub trait Transfer: Serialize + Deserialize {
    /// Create a string representation of this type.
    // fn encode(self) -> String;

    /// Create an object from a a string representation.
    // fn decode(repr: &str) -> Self;

    /// If any data is lost or changes when encoding and decoding, then clean should remove/change the same data.
    /// This is to ensure that direct calls behave the same as remote api calls.
    fn clean(self) -> Self;
}
