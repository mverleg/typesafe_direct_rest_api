use iface::iface::example_one::ExampleOne;
use iface::iface::example_one::SearchQuery;
use iface::iface::example_one::SearchResult;
use apilib::apilib::request::TRequest;
use apilib::apilib::response::TResponse;

#[derive(new, Debug)]
pub struct ExampleOneWrapper {
    service: Box<ExampleOne>,
}

impl ExampleOne for ExampleOneWrapper {
    fn search(&self, request: TRequest<SearchQuery>) -> TResponse<SearchResult> {
        // todo
    }
}
