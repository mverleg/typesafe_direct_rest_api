use apilib::transfer::Transfer;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TResponse<T: Transfer> {
    Ok(T),
    Err(String),
}

impl<'de, T: Transfer + Serialize + Deserialize<'de>> TResponse<T> {
    pub fn ok(value: T) -> Self {
        TResponse::Ok(value)
    }

    // maybe 'code' should be an enum to only allow valid http codes
    pub fn err(code: u16, message: String) -> Self {
        TResponse::Err(message)
    }
}

impl<'de, T: Transfer + Serialize + Deserialize<'de>> Transfer for TResponse<T> {
//    fn encode(self) -> String {
//        serde_json::to_string(&self).unwrap()
//    }
//
//    fn decode(repr: &str) -> Self {
//        serde_json::from_str(repr).unwrap()
//    }

    fn clean(self) -> Self {
        match self {
            TResponse::Ok(value) => TResponse::Ok(value.clean()),
            err => err,
        }
    }
}

// TODO @mverleg: I would like to use derive so I don't have to type this
impl<T> PartialEq for TResponse<T> where T: Transfer + PartialEq {
    fn eq(&self, other: &TResponse<T>) -> bool {
        match (self, other) {
            (TResponse::Ok(sok), TResponse::Ok(ook)) => sok == ook,
            (TResponse::Ok(_), TResponse::Err(_)) => false,
            (TResponse::Err(_), TResponse::Ok(_)) => false,
            (TResponse::Err(ser), TResponse::Err(oer)) => ser == oer,
        }
    }
}
