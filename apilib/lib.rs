#![feature(extern_prelude)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate simple_server;

pub mod apilib {
    pub mod transfer;
    pub mod request;
    pub mod response;
    pub mod target;

    #[cfg(test)]
    mod tests;
}
