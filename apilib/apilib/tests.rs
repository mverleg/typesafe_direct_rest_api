use apilib::request::TRequest;
use apilib::transfer::Transfer;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct TestData {
    nr: i32,
    text: String,
    internal: i32,
}

impl TestData {
    pub fn new(nr: i32, text: String) -> Self {
        TestData { nr, text, internal: 0 }
    }
}

impl Transfer for TestData {
    fn encode(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode(repr: &str) -> Self {
        serde_json::from_str(repr).unwrap()
    }

    fn clean(mut self) -> Self {
        self.internal = 0;
        self
    }
}

#[test]
fn test_request() {
    let ex = TRequest::new(TestData::new(1, "hi".to_owned()));
    let txt = ex.clone().encode();
    let back = TRequest::decode(&txt);
    assert_eq!(ex, back);
}
