use chrono::{DateTime, Utc};

#[derive(Copy, Clone)]
pub struct OccurrencePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl OccurrencePeriod {
    pub fn from_dates(start: DateTime<Utc>, end: DateTime<Utc>) -> OccurrencePeriod {
        OccurrencePeriod { start, end }
    }
}
