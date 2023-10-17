use std::num::TryFromIntError;
use std::sync::PoisonError;
use std::time::SystemTimeError;

pub(super) enum Error {
    TimestampOverflow,
    SequenceOverflow,
    SystemTime,
    PoisonError,
}

impl From<SystemTimeError> for Error {
    fn from(_value: SystemTimeError) -> Self {
        Error::SystemTime
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::TimestampOverflow
    }
}

impl From<PoisonError<_>> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::PoisonError
    }
}
