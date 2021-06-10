use impl_enum::with_methods;

use super::Kind;

#[enum_dispatch::enum_dispatch]
#[with_methods {
    pub fn to_string(&self) -> String {}
}]
pub enum Internal {
    Connection,
    StdError,
}

impl<E: std::error::Error> From<E> for Internal {
    fn from(err: E) -> Self {
        StdError(err.to_string()).into()
    }
}

pub struct Connection(String);
impl Kind for Connection {}

impl<E: std::error::Error> From<E> for Connection {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}

pub struct StdError(pub String);
impl Kind for StdError {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<E: std::error::Error> From<E> for StdError {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}
