use chrono::{DateTime};
use rrule::{RRuleSet, Tz};
use crate::occurrence_period::OccurrencePeriod;

pub fn parse_rrule_between(rrule: String, duration: i32, after_iso_str: String, before_iso_str: String) -> Vec<OccurrencePeriod> {
    let tz = Tz::UTC;

    let after = DateTime::parse_from_rfc3339(&after_iso_str.as_str()).unwrap();
    let before = DateTime::parse_from_rfc3339(&before_iso_str.as_str()).unwrap();
    let rrule_set: RRuleSet = rrule.parse().unwrap();
    
    let rrule_set = rrule_set
        .after(after.with_timezone(&tz))
        .before(before.with_timezone(&tz));

    let (events, _) = rrule_set.all(10000);

    events.iter().map(|event| OccurrencePeriod::new(event.clone(), duration)).collect()
}

