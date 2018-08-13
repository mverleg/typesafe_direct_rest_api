use iface::iface::example_one::ExampleOne;
use iface::iface::example_one::SearchQuery;
use iface::iface::example_one::SearchResult;
use apilib::apilib::request::TRequest;
use apilib::apilib::response::TResponse;

#[derive(new, Debug)]
pub struct ExampleOneService {}

impl ExampleOne for ExampleOneService {
    fn search(&self, request: TRequest<SearchQuery>) -> TResponse<SearchResult> {
        TResponse::Ok(SearchResult::new(vec![
            request.value.keywords.clone() + " one",
            request.value.keywords.clone() + " two",
            request.value.keywords + " three",
        ]))
    }
}
