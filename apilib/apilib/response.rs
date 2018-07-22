use apilib::transfer::Transfer;

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
