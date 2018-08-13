
// Some code that the client could use, and then eventually:

pub fn do_client_things(service: Box<ExampleOne>) {
    let query = TRequest::new(SearchQuery::new("needle".to_owned()));
    println!("searching for {}", query.keywords);
    let response = service.search(query);
    match response {
        TResponse::Ok(val) => {
            println!("found {} results", val.matches.len());
            for mtch in val.matches {
                println!("* \"{}\"", mtch);
            }
        },
        TResponse::Err(msg) => panic!("rest request failed: {}", msg),
    }
}
