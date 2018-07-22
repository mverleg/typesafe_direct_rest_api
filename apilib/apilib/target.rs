
pub enum TMethod {
    GET,     // read
    POST,    // add
    PUT,     // replace
    PATCH,   // adapt
    DELETE,  // remove
    // Others not supported
}

pub trait TTarget {
    fn method() -> TMethod;
    fn url() -> String;
}
