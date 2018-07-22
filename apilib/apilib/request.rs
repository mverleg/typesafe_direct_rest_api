use apilib::transfer::Transfer;
use serde_json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TRequest<T: Transfer> {
    value: T,
}

impl<T: Transfer> TRequest<T> {
    pub fn new(value: T) -> Self {
        TRequest { value }
    }
}

// TODO @mverleg: move these bounds to Transfer
impl<'de, T: Transfer + Serialize + Deserialize<'de>> Transfer for TRequest<T> {
    fn encode(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode(repr: &str) -> Self {
        let obj: Self = serde_json::from_str(repr).unwrap();
        obj.clone()
    }

    fn clean(self) -> Self {
        self.value.clean();
        self
    }
}

// TODO @mverleg: I would like to use derive so I don't have to type this
impl<T> PartialEq for TRequest<T> where T: Transfer + PartialEq {
    fn eq(&self, other: &TRequest<T>) -> bool {
        self.value == other.value
    }
}
