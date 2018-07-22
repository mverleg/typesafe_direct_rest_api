
// TODO @mverleg: should this be binary instead of strings?
// TODO @mverleg: how to set the serialization format?

/// Trait for types that can be transferred through the API.
/// See https://serde.rs/lifetimes.html for why DeserializeOwned
// TODO @mverleg: bounds, see https://stackoverflow.com/questions/51464671/
pub trait Transfer: Clone {
    /// Create a string representation of this type.
    // TODO @mverleg: this can be removed after Transfer implies Serialize/Deserialize https://stackoverflow.com/questions/51464671/
     fn encode(self) -> String;

    /// Create an object from a a string representation.
    // TODO @mverleg: this can be removed after Transfer implies Serialize/Deserialize https://stackoverflow.com/questions/51464671/
     fn decode(repr: &str) -> Self;

    /// If any data is lost or changes when encoding and decoding, then clean should remove/change the same data.
    /// This is to ensure that direct calls behave the same as remote api calls.
    fn clean(self) -> Self;
}
