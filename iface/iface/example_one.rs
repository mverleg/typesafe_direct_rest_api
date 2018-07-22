
pub struct SearchQuery {
    keywords: String,
}

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
