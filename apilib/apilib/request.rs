use apilib::transfer::Transfer;
use serde_json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TRequest<T: Transfer> {
    pub value: T,
}

impl<T: Transfer> TRequest<T> {
    pub fn new(value: T) -> Self {
        TRequest { value }
    }
}

impl<'de, T: Transfer + Serialize + Deserialize<'de>> Transfer for TRequest<T> {
    fn clean(self) -> Self {
        TRequest { value: self.value.clean() }
    }
}

// TODO @mverleg: I would like to use derive so I don't have to type this
impl<T> PartialEq for TRequest<T> where T: Transfer + PartialEq {
    fn eq(&self, other: &TRequest<T>) -> bool {
        self.value == other.value
    }
}
