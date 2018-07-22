use apilib::transfer::Transfer;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TResponse<T: Transfer> {
    Ok(T),
    Err(String),
}

impl<T: Transfer> TResponse<T> {
    pub fn ok(value: T) -> Self {
        TResponse::Ok(value)
    }

    pub fn err(message: String) -> Self {
        TResponse::Err(message)
    }
}

// TODO @mverleg: move these bounds to Transfer
impl<'de, T: Transfer + Serialize + Deserialize<'de>> Transfer for TResponse<T> {
    fn encode(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode(repr: &str) -> Self {
        serde_json::from_str(repr).unwrap()
    }

    fn clean(self) -> Self {
        self.value.clean();
        self
    }
}

// TODO @mverleg: I would like to use derive so I don't have to type this
impl<T> PartialEq for TResponse<T> where T: Transfer + PartialEq {
    fn eq(&self, other: &TResponse<T>) -> bool {
        self.value == other.value
    }
}
