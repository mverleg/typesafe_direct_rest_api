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
//    fn encode(self) -> String {
//        serde_json::to_string(&self).unwrap()
//    }
//
//    fn decode(repr: &str) -> Self {
//        serde_json::from_str(repr).unwrap()
//    }

    fn clean(mut self) -> Self {
        self.internal = 42;
        self
    }
}

#[test]
fn test_request() {
    let mut ex = TRequest::new(TestData::new(1, "hi".to_owned()));
    let txt = serde_json::to_string(&ex).unwrap();
    println!("request json: {}", &txt);
    ex.value.internal = 0;  // internal is skipped so will become 0.
    let back: TRequest<TestData> = serde_json::from_str(&txt).unwrap();
    assert_eq!(ex, back);
}
