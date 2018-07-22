use apilib::request::TRequest;
use apilib::transfer::Transfer;
use apilib::response::TResponse;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct TestData {
    nr: i32,
    text: String,
    #[serde(skip)]
    internal: i32,
}

impl TestData {
    pub fn new(nr: i32, text: String) -> Self {
        TestData { nr, text, internal: 42 }
    }
}

impl Transfer for TestData {
    fn clean(mut self) -> Self {
        self.internal = 42;
        self
    }
}

#[test]
fn test_request() {
    let mut ex = TRequest::new(TestData::new(1, "hi".to_owned()));
    let txt = serde_json::to_string(&ex).unwrap();
    ex.value.internal = 0;  // internal is skipped so will become 0.
    let back: TRequest<TestData> = serde_json::from_str(&txt).unwrap();
    assert_eq!(ex, back);
}

#[test]
fn test_response_ok() {
    let mut ex = TResponse::Ok(TestData::new(1, "hi".to_owned()));
    let txt = serde_json::to_string(&ex).unwrap();
    ex = match ex {
        TResponse::Ok(mut val) => {
            val.internal = 0;
            TResponse::Ok(val)
        },
        _ => panic!("impossible"),
    };
    let back: TResponse<TestData> = serde_json::from_str(&txt).unwrap();
    assert_eq!(ex, back);
}

#[test]
fn test_response_err() {
    let ex = TResponse::Err(404, "ow noes!".to_owned());
    let txt = serde_json::to_string(&ex).unwrap();
    let back: TResponse<TestData> = serde_json::from_str(&txt).unwrap();
    assert_eq!(ex, back);
}
