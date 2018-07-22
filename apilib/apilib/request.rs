use apilib::transfer::Transfer;
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TRequest<T: Transfer> {
    value: T,
}

impl<T: Transfer> TRequest<T> {
    pub fn new(value: T) -> Self {
        TRequest { value }
    }
}

impl<T: Transfer> Transfer for TRequest<T> {
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
