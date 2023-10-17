use std::num::TryFromIntError;
use std::sync::PoisonError;
use std::time::SystemTimeError;

pub(super) enum SnowflakeError {
    TimestampOverflow,
    SequenceOverflow,
    SystemTime,
    PoisonError,
}

impl From<SystemTimeError> for SnowflakeError {
    fn from(_value: SystemTimeError) -> Self {
        SnowflakeError::SystemTime
    }
}

impl From<TryFromIntError> for SnowflakeError {
    fn from(value: TryFromIntError) -> Self {
        SnowflakeError::TimestampOverflow
    }
}

impl From<PoisonError<_>> for SnowflakeError {
    fn from(value: TryFromIntError) -> Self {
        SnowflakeError::PoisonError
    }
}
