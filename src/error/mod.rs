mod internal;
pub use internal::*;

use enum_dispatch::enum_dispatch;
use tonic::Status;

#[enum_dispatch]
pub enum Error {
    Internal,
}

impl From<Error> for Status {
    fn from(err: Error) -> Status {
        log::error!("{}", err.to_string());
        match err {
            Error::Internal(internal) => Status::internal(internal.to_string()),
        }
    }
}

#[enum_dispatch::enum_dispatch(Internal)]
#[enum_dispatch::enum_dispatch(Error)]
trait Kind {
    fn to_string(&self) -> String {
        "Kind".into()
    }
}
