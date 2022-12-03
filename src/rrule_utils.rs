use std::str::FromStr;
use chrono::{DateTime, Utc};
use rrule::{RRuleSet, Tz};
use crate::occurrence_period::OccurrencePeriod;
use crate::serializable::Serializable;

pub fn parse_between(ev: Serializable, start: DateTime<Utc>, end: DateTime<Utc>, include_partial: bool) -> Vec<OccurrencePeriod> {
    let tz = Tz::UTC;

    let rrule_str = string_as_option(ev.rrule.as_str());

    match rrule_str {
        None => {
            if ev.until.is_some() {
                let until = ev.until.unwrap();
                
                if ev.start > until {
                    return vec![];
                }
            }

            vec![OccurrencePeriod::from_dates(ev.start, ev.end)]
        }

        Some(rrule) => {
            let until = ev.until.ok_or("Until is required when rrule is not empty").unwrap();

            let rrules: Vec<String> = if rrule.contains("DTSTART") {
                rrule
                    .split("DTSTART")
                    .filter(|s| !s.is_empty())
                    .map(|s| "DTSTART".to_owned() + s).collect()
            } else {
                vec![rrule.clone()]
            };

            let rrule_sets: Vec<RRuleSet> = rrules.iter().map(|rrule| {
                rrule.parse().unwrap()
            })
                .collect();


            let mut occurrences: Vec<OccurrencePeriod> = vec![];
            let duration = ev.end.signed_duration_since(ev.start);

            for rrule_set in rrule_sets {
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


                // for each raw occurrence, if it is within the start and end date, add it to the occurrences
                for occ in raw_occurrences {
                    let occ_utc = occ.with_timezone(&Utc);

                    if until < occ_utc {
                        continue;
                    }

                    if include_partial && occ_utc < ev.start {
                        let diff = ev.start.signed_duration_since(occ_utc);
                        occurrences.push(OccurrencePeriod { start: occ_utc, end: ev.start + duration - diff });
                    } else if occ_utc >= ev.start {
                        occurrences.push(OccurrencePeriod { start: occ_utc, end: occ_utc + duration });
                    }
                }
            }

            occurrences
        }
    }
}

fn string_as_option(s: &str) -> Option<String> {
    if s.is_empty() || s == "" {
        None
    } else {
        Some(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc, TimeZone};
    use crate::occurrence_period::OccurrencePeriod;

    #[test]
    fn test_parse_biweekly_week1() {
        let ev = Serializable {
            start: str_to_utc("2022-08-08T10:00:00Z"),
            end: str_to_utc("2022-08-08T11:00:00Z"),
            until: Some(str_to_utc("9999-12-31T23:59:59Z")),
            rrule: "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH".to_string(),
        };

        let start = str_to_utc("2022-08-10T21:00:00Z");
        let end = str_to_utc("2022-08-11T22:59:58Z");

        let periods = parse_between(ev, start, end, false);

        assert_eq!(periods.len(), 0);
    }

    #[test]
    fn test_parse_biweekly_week2() {
        let ev = Serializable {
            start: str_to_utc("2022-08-08T10:00:00Z"),
            end: str_to_utc("2022-08-08T11:00:00Z"),
            until: Some(str_to_utc("9999-12-31T23:59:59Z")),
            rrule: "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH".to_string(),
        };

        let start = str_to_utc("2022-08-17T21:00:00Z");
        let end = str_to_utc("2022-08-18T22:59:58Z");

        let periods = parse_between(ev, start, end, false);

        assert_eq!(periods.len(), 1);
    }

    #[test]
    fn test_parse_biweekly_both_weeks() {
        let ev = Serializable {
            start: str_to_utc("2022-08-08T10:00:00Z"),
            end: str_to_utc("2022-08-08T11:00:00Z"),
            until: Some(str_to_utc("9999-12-31T23:59:59Z")),
            rrule: "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH".to_string(),
        };

        let start = str_to_utc("2022-08-11T21:00:00Z");
        let end = str_to_utc("2022-08-18T22:59:58Z");

        let periods = parse_between(ev, start, end, false);

        assert_eq!(periods.len(), 3); // 1 from week 1, 2 from week 2
    }
    
    #[test]
    fn test_instance() {
        let ev = Serializable {
            start: str_to_utc("2022-08-08T10:00:00Z"),
            end: str_to_utc("2022-08-08T11:00:00Z"),
            until: Some(str_to_utc("2022-08-08T11:00:00Z")),
            rrule: "".to_string()
        };

        let start = str_to_utc("2022-08-01T00:00:00Z");
        let end = str_to_utc("2022-08-31T00:00:00Z");

        let periods = parse_between(ev, start, end, false);
        
        assert_eq!(periods.len(), 1);
    }

    #[test]
    fn test_instance_until_less_than_end() {
        let ev = Serializable {
            start: str_to_utc("2022-08-08T10:00:00Z"),
            end: str_to_utc("2022-08-08T11:00:00Z"),
            until: Some(str_to_utc("2022-08-08T09:00:00Z")),
            rrule: "".to_string()
        };

        let start = str_to_utc("2022-08-01T00:00:00Z");
        let end = str_to_utc("2022-08-31T00:00:00Z");

        let periods = parse_between(ev, start, end, false);

        assert_eq!(periods.len(), 0);
    }
}


fn str_to_utc(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}