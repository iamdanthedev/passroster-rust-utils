use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use rrule::{RRuleSet, Tz};

#[derive(Debug, Clone)]
pub struct InvalidRRuleError {
    message: String,
}

impl InvalidRRuleError {
    fn new(message: String) -> InvalidRRuleError {
        InvalidRRuleError { message }
    }
}

impl fmt::Display for InvalidRRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

pub fn parse_rrules(rrule_sets_str: &str) -> Result<Vec<RRuleSet>, InvalidRRuleError> {
    let rrules_result = string_to_rrule_sets_str(rrule_sets_str);
    
    if rrules_result.is_err() {
        return Err(rrules_result.err().unwrap());
    }
    
    let rrules = rrules_result.unwrap();

    let mut rrule_sets: Vec<RRuleSet> = Vec::new();
    let mut error: Option<InvalidRRuleError> = None;

    for rrule in rrules {
        let without_until = remove_until(&rrule);
        let rrule_result = RRuleSet::from_str(&without_until);

        match rrule_result {
            Ok(rrule_set) => {
                rrule_sets.push(rrule_set);
            }
            Err(err) => {
                error = Some(InvalidRRuleError::new(err.to_string()));
                break;
            }
        }
    }

    match error {
        Some(err) => Err(err),
        None => Ok(rrule_sets),
    }
}

pub fn parse_between(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    rrule: &str,
) -> Result<Vec<DateTime<Utc>>, InvalidRRuleError> {
    let tz = Tz::UTC;

    let rrule_set_result = parse_rrules(&rrule);

    if rrule_set_result.is_err() {
        return Err(rrule_set_result.err().unwrap());
    }

    let rrule_sets = rrule_set_result.unwrap();

    let mut occurrences: Vec<DateTime<Utc>> = vec![];

    for rrule_set in rrule_sets {
        let rrule_set = rrule_set
            .after(start.with_timezone(&tz))
            .before(end.with_timezone(&tz));

        // .all required a limit in order to prevent infinite loops
        // in pass-roster the most frequently occurring event is a daily event
        // so 1000 occurrences should be more than enough
        let (raw_occurrences, _) = rrule_set.all(1000);
        
        let slice: Vec<DateTime<Utc>> = raw_occurrences
            .iter()
            .map(|occ| occ.with_timezone(&Utc))
            .collect();
        
        occurrences.extend(slice);
    }

    Ok(occurrences)
}

fn string_to_rrule_sets_str(s: &str) -> Result<Vec<String>, InvalidRRuleError> {
    if !s.contains("DTSTART") {
        return Err(InvalidRRuleError::new("DTSTART is required".to_string()));
    }

    let result = s
        .split("DTSTART")
        .filter(|s| !s.is_empty())
        .map(|s| "DTSTART".to_owned() + s)
        .collect();
    
    Ok(result)
}

fn remove_until(s: &str) -> String {
    lazy_static! {
        static ref UNTIL_RE_1: Regex = Regex::new("UNTIL=.+$").unwrap();
        static ref UNTIL_RE_2: Regex = Regex::new("UNTIL=.+\n").unwrap();
        static ref BYDAY_UNDEF_RE: Regex = Regex::new("BYDAY=undefined").unwrap();
    }

    let mut result = UNTIL_RE_1.replace_all(s, "").to_string();
    result = UNTIL_RE_2.replace_all(&result, "\n").to_string();
    result = BYDAY_UNDEF_RE.replace_all(&result, "").to_string();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_to_rrules_dtstart_is_mandatory() {
        let result = string_to_rrule_sets_str("FREQ=YEARLY;BYMONTH=1;BYMONTHDAY=1");
        assert!(result.is_err());
    }

    #[test]
    fn test_string_to_rrules_simple() {
        let result = string_to_rrule_sets_str("DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR");
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_string_to_rrules_multiple() {
        let result = string_to_rrule_sets_str("DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH");
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_parse_biweekly_week1() {
        let rrule = "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH";
        let start = str_to_utc("2022-08-10T21:00:00Z");
        let end = str_to_utc("2022-08-11T22:59:58Z");

        let periods = parse_between(start, end, rrule);

        assert_eq!(periods.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_biweekly_week2() {
        let rrule = "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH";
        let start = str_to_utc("2022-08-17T21:00:00Z");
        let end = str_to_utc("2022-08-18T22:59:58Z");

        let periods = parse_between(start, end, rrule);

        assert_eq!(periods.unwrap().len(), 1);
    }

    #[test]
    fn test_parse_biweekly_both_weeks() {
        let rrule = "DTSTART;TZID=Europe/London:20210614T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR\nDTSTART;TZID=Europe/London:20210621T110000\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH";
        let start = str_to_utc("2022-08-11T21:00:00Z");
        let end = str_to_utc("2022-08-18T22:59:58Z");

        let periods = parse_between(start, end, rrule);

        assert_eq!(periods.unwrap().len(), 3); // 1 from week 1, 2 from week 2
    }

    #[test]
    fn test_rrule_with_until() {
        let rrule = "DTSTART;TZID=Europe/London:20220613T180000\nRRULE:FREQ=DAILY;INTERVAL=1;UNTIL=20220614T230000";
        let start = str_to_utc("2022-11-25T00:00:00Z");
        let end = str_to_utc("2022-12-05T00:00:00Z");

        let periods = parse_between(start, end, rrule);

        assert_eq!(periods.unwrap().len(), 10);
    }

    #[test]
    fn test_remove_until() {
        let input = "DTSTART;TZID=Europe/London:20220513T222900\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,TU,WE,TH;UNTIL=20220603T225959\nDTSTART;TZID=Europe/London:20220516T222900\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=FR;UNTIL=20220603T225959".to_string();
        let result = remove_until(&input);
        assert_eq!(result, "DTSTART;TZID=Europe/London:20220513T222900\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,TU,WE,TH;\nDTSTART;TZID=Europe/London:20220516T222900\nRRULE:FREQ=WEEKLY;INTERVAL=2;BYDAY=FR;");
    }

    fn str_to_utc(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }
}
