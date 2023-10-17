mod error;

use self::error::Error;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

struct SnowflakeGenerator {
    epoch: SystemTime,
    id: u32,
    sequence: u32,
    prev_time: Arc<Mutex<u64>>,
}

impl SnowflakeGenerator {
    fn next(&mut self) -> Result<u64, Error> {
        let mut time = self.get_local_time()?;
        let mut prev_time = self.prev_time.lock()?;
        if *prev_time == time {
            self.sequence += 1;
            if self.sequence > 0b111_111_111_111 {
                return Err(Error::SequenceOverflow);
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

    fn get_local_time(&self) -> Result<u64, Error> {
        let now = SystemTime::now().duration_since(self.epoch)?;
        let a = u64::try_from(now.as_millis());

        match u64::try_from(now.as_millis()) {
            Ok(x) if x < 2u64.pow(41) => Ok(x),
            _ => Err(Error::TimestampOverflow),
        }
    }
}
