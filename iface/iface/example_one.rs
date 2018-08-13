use apilib::apilib::target::TMethod;
use apilib::apilib::request::TRequest;
use apilib::apilib::response::TResponse;
use apilib::apilib::target::TTarget;
use apilib::apilib::transfer::Transfer;
use std::path::PathBuf;

#[derive(new, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
    pub keywords: String,
}

impl Transfer for SearchQuery {
    fn clean(self) -> Self {
        self
    }
}

#[derive(new, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResult {
    pub matches: Vec<String>,
}

impl Transfer for SearchResult {
    fn clean(self) -> Self {
        self
    }
}

pub trait ExampleOne {
    // TODO @mverleg: the '&self' is only to make it object-safe
    fn search(&self, request: TRequest<SearchQuery>) -> TResponse<SearchResult>;
}

impl TTarget for ExampleOne {
    fn method(&self) -> TMethod {
        TMethod::Get
    }

    fn url(&self) -> PathBuf {
        // TODO @mverleg: there has to be a one-line way...
        let mut path = PathBuf::new();
        path.push("/ex1");
        path
    }
}
