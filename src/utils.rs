pub trait FlatMapResult<T,E> {
    fn flat_map<O, F: FnOnce(T) -> Result<O,E>> (self, map: F) -> Result<O,E>;
}

impl<T,E> FlatMapResult<T,E> for Result<T,E> {
    fn flat_map<O, F: FnOnce(T) -> Result<O,E>>(self, map: F) -> Result<O,E> {
        match self {
            Err(x) => Err(x),
            Ok(x) => map(x)
        }
    }
}