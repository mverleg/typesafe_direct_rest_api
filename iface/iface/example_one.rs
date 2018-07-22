
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
    keywords: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResult {
    matches: Vec<String>,
}

pub trait ExampleOne {
    fn search(request: TRequest<SearchQuery>) -> TResponse<SearchResult> {
        let result = TResponse::new(
            request.value.clone() + " one".to_owned(),
            request.value.clone() + " two".to_owned(),
            request.value  + " three".to_owned(),
        );
    }
}

impl Endpoint for ExampleOne {
    fn method() -> TMethod {
        TMethod::Get
    }

    fn url() -> String {
        "/ex1".to_owned()
    }
}
