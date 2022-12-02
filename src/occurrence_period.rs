use chrono::{DateTime, Duration, Utc};
use js_sys::Date;

pub struct OccurrencePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl OccurrencePeriod {
    pub fn new(start: DateTime<Utc>, duration_minutes: i32) -> OccurrencePeriod {
        let end = start + Duration::minutes(duration_minutes.into());
        OccurrencePeriod { start, end }
    }
    
    pub fn from_dates(start: DateTime<Utc>, end: DateTime<Utc>) -> OccurrencePeriod {
        OccurrencePeriod { start, end }
    }
}
