use std::str::FromStr;
use chrono::{DateTime, Utc};
use rrule::{RRuleSet, Tz};
use crate::occurrence_period::OccurrencePeriod;
use crate::serializable::Serializable;

pub fn parse_between(ev: Serializable, start: DateTime<Utc>, end: DateTime<Utc>, include_partial: bool) -> Vec<OccurrencePeriod> {
    let tz = Tz::UTC;
    
    let rrule_str = string_as_option(ev.rrule.as_str());

    return match rrule_str {
        None => {
            if ev.until.is_some() {
                let until = ev.until.unwrap();
                if start > until {
                    return vec![];
                }
            }

            return vec![OccurrencePeriod::from_dates(ev.start, ev.end)];
        }

        Some(rrule) => {
            let until = ev.until.ok_or("Until is required when rrule is not empty").unwrap();

            let rrule_set: RRuleSet = rrule.parse().unwrap();

            let rrule_set = rrule_set
                .after(start.with_timezone(&tz))
                .before(end.with_timezone(&tz));

            // .all required a limit in order to prevent infinite loops
            // in pass-roster the most frequently occurring event is a daily event
            // so 1000 occurrences should be more than enough
            let (raw_occurrences, _) = rrule_set.all(1000);

            if raw_occurrences.len() >= 1000 {
                panic!("Too many events per rrule");
            }

            let mut occurrences: Vec<OccurrencePeriod> = vec![];
            let duration = ev.end.signed_duration_since(ev.start);

            // for each raw occurrence, if it is within the start and end date, add it to the occurrences
            for occ in raw_occurrences {
                let occUtc = occ.with_timezone(&Utc);

                if until < occUtc {
                    continue;
                }

                if include_partial && occUtc < ev.start {
                    let diff = ev.start.signed_duration_since(occUtc);
                    occurrences.push(OccurrencePeriod { start: occUtc, end: ev.start + duration - diff });
                } else if occUtc >= ev.start {
                    occurrences.push(OccurrencePeriod { start: occUtc, end: occUtc + duration });
                }
            }

            occurrences
        }
    };
}

fn string_as_option(s: &str) -> Option<String> {
    if s.is_empty() || s == "" {
        None
    } else {
        Some(s.to_string())
    }
}

#[cfg(test)]
mod tests {}

