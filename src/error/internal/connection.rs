
pub struct Connection(String);

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl <E: std::error::Error> From<E> for Connection {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}