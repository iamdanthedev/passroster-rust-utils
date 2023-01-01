mod rrule_utils;
mod utils;

use crate::rrule_utils::{parse_between, parse_rrules};
use crate::utils::set_panic_hook;
use chrono::{DateTime, TimeZone, Utc};
use js_sys::{BigInt64Array, Date};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "parseBetween")]
pub fn parse_between_js(
    start: Date,
    end: Date,
    rrule: &str,
) -> BigInt64Array {
    set_panic_hook();

    let start = Utc.timestamp_millis_opt(start.get_time() as i64).unwrap();
    let end = Utc.timestamp_millis_opt(end.get_time() as i64).unwrap();

    let occurrences_result = parse_between(start, end, rrule);
    
    match occurrences_result {
        Ok(occurrences) => occurrences_to_bigint64array(&occurrences),
        Err(err) => {
            panic!("{}", err);
        }
    }
}

#[wasm_bindgen(js_name = "validate")]
pub fn validate(rrule_sets_str: js_sys::JsString) -> JsValue {
    set_panic_hook();

    let s: String = rrule_sets_str.into();
    let result = parse_rrules(s.as_str());
    
    match result {
        Ok(_) => JsValue::NULL,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}

fn occurrences_to_bigint64array(occurrences: &Vec<DateTime<Utc>>) -> BigInt64Array {
    let seq: Vec<i64> = occurrences
        .iter()
        .map(|occ| occ.timestamp_millis())
        .collect();
    
    BigInt64Array::from(seq.as_slice())
}
