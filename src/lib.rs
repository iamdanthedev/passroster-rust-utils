mod utils;
mod occurrence_period;
mod rrule_utils;
mod serializable;

use wasm_bindgen::prelude::*;
use crate::rrule_utils::{parse_between};
use crate::serializable::{Serializable, SerializableJs};
use chrono::{DateTime, TimeZone, Utc};
use js_sys::{Date};
use crate::occurrence_period::{OccurrencePeriod};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "parseBetween")]
pub fn parse_between_js(ev: SerializableJs, start: Date, end: Date, include_partial: bool) -> js_sys::BigInt64Array {
    let ev = Serializable::from_js(ev);
    let start = Utc.timestamp_millis_opt(start.get_time() as i64).unwrap();
    let end = Utc.timestamp_millis_opt(end.get_time() as i64).unwrap();
    
    let periods = parse_between(ev, start, end, include_partial);

    let seq: Vec<i64> = periods.iter().flat_map(|period| vec![period.start.timestamp_millis(), period.end.timestamp_millis()]).collect();

    js_sys::BigInt64Array::from(seq.as_slice())
}
