extern crate simple_server;

use self::simple_server::Server;
use std::collections::HashMap;
use iface::iface::example_one::ExampleOne;
use gen_rest_server::example_one::ExampleOneWrapper;
use myserver::myserver::example_one::ExampleOneService;
use apilib::apilib::target::TMethod;
use apilib::apilib::target::TTarget;
use std::path::PathBuf;

#[derive(new)]
struct IndexEntry {
    method: TMethod,
    url: PathBuf,
    target: Box<TTarget>,
}

pub fn run_server() {
    // These should be regexes instead of paths, but for MWE paths are ok
    let mut index = HashMap::new();
    let wrap_service = Box::new(ExampleOneService::new());
    index.insert(wrap_service.url(), IndexEntry::new(
        wrap_service.method(),
        wrap_service.url(),
        Box::new(ExampleOneWrapper::new(wrap_service)),
    ));

    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());
        match index.get(request.method()) {
            Some(entry) => {
                // Ok(response.body("Hello Rust!".as_bytes().to_vec())?)
                if entry.method == request.method() {
                    Ok(response.body("todo".as_bytes().to_vec())?)
                } else {
                    unimplemented!("405")
                }
            },
            None => unimplemented!("404")
        }
    });

    server.listen("127.0.0.1", "8888");
}
