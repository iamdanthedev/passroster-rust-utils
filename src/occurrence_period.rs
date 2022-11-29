use chrono::{DateTime, Duration};
use rrule::{Tz};

pub struct OccurrencePeriod {
    pub start: i64,
    pub end: i64,
}

impl OccurrencePeriod {
    pub fn new(start: DateTime<Tz>, duration_minutes: i32) -> OccurrencePeriod {
        let end = start + Duration::minutes(duration_minutes.into());
        OccurrencePeriod { start: start.timestamp_millis(), end: end.timestamp_millis() }
    }
}
