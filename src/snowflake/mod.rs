use std::num::TryFromIntError;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{SystemTime, SystemTimeError};

enum SnowflakeError {
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

struct SnowflakeGenerator {
    epoch: SystemTime,
    id: u32,
    sequence: u32,
    prev_time: Arc<Mutex<u64>>,
}

impl SnowflakeGenerator {
    fn next(&mut self) -> Result<u64, SnowflakeError> {
        let mut time = self.get_local_time()?;
        let mut prev_time = self.prev_time.lock()?;
        if *prev_time == time {
            self.sequence += 1;
            if self.sequence > 0b111_111_111_111 {
                return Err(SnowflakeError::SequenceOverflow);
            }
        } else {
            self.sequence = 0;
        };
        *prev_time = time;

        /*
            0 ..41.. 41 - 42 ..10.. 52 - 53 ..12.. 64
            <timestamp>   <    id    >   < sequence >
        */
        Ok(time << 22 | self.id << 12 | self.sequence)
    }

    fn get_local_time(&self) -> Result<u64, SnowflakeError> {
        let now = SystemTime::now().duration_since(self.epoch)?;
        let a = u64::try_from(now.as_millis());

        match u64::try_from(now.as_millis()) {
            Ok(x) if x < 2u64.pow(41) => Ok(x),
            _ => Err(SnowflakeError::TimestampOverflow),
        }
    }
}
