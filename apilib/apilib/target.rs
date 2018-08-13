use std::path::PathBuf;
use simple_server::Method;

pub enum TMethod {
    Get,     // read
    Post,    // add
    Put,     // replace, idempotent
    Patch,   // adapt, incremental
    Delete,  // remove
    // Others not supported
}

impl PartialEq<Method> for TMethod {
    fn eq(&self, other: &Method) -> bool {
        match (self, other) {
            (TMethod::Get, &Method::GET) => true,
            (TMethod::Post, &Method::POST) => true,
            (TMethod::Put, &Method::PUT) => true,
            (TMethod::Patch, &Method::PATCH) => true,
            (TMethod::Delete, &Method::DELETE) => true,
            _ => false,
        }
    }
}

// TODO @mverleg: currently these are per-trait instead of per-method
pub trait TTarget {
    // TODO @mverleg: self only for object-safety
    fn method(&self) -> TMethod;
    fn url(&self) -> PathBuf;
}
