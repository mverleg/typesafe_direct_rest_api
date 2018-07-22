#![feature(extern_prelude)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod apilib {
    pub mod transfer;
    pub mod request;
    pub mod response;

    #[cfg(test)]
    mod tests;
}
