mod utils;

use chrono::{DateTime, Duration, Utc, FixedOffset};
use rrule::{RRuleSet, Tz};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// external method, something we might want to call from rust
// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub struct OccurrencePeriod {
    pub start: i64,
    pub end: i64,
}

#[wasm_bindgen]
impl OccurrencePeriod {
    fn new(start: DateTime<Tz>, duration_minutes: i32) -> OccurrencePeriod {
        let end = start + Duration::minutes(duration_minutes.into());
        
        // let start_js = js_sys::Date::new(&JsValue::from(start.timestamp_millis()));
        // let end_js = js_sys::Date::new(&JsValue::from(end.timestamp_millis()));
        
        OccurrencePeriod { start: start.timestamp_millis(), end: end.timestamp_millis() }
    }
    
    pub fn get_start(&self) -> i64 {
        return self.start
    }
    
    pub fn get_end(&self) -> i64 {
        self.end
    }
}

#[wasm_bindgen(js_name = "greet")]
pub fn greet(s: String) {
    web_sys::console::log_1(&"Hello, world!".into());
    println!("Hello, {}!", s);
}

#[wasm_bindgen(js_name = "parseRruleBetween")]
pub fn parse_rrule_between(rrule: String, duration: i32, after_iso_str: String, before_iso_str: String) -> js_sys::BigInt64Array {
    let tz = Tz::UTC;
    
    web_sys::console::log_1(&rrule.as_str().into());
    web_sys::console::log_1(&after_iso_str.as_str().into());
    web_sys::console::log_1(&before_iso_str.as_str().into());
    
    let after = DateTime::parse_from_rfc3339(&after_iso_str.as_str()).unwrap();
    let before = DateTime::parse_from_rfc3339(&before_iso_str.as_str()).unwrap();

    web_sys::console::log_1(&"1".into());

    let rrule_set: RRuleSet = rrule.parse().unwrap();
    web_sys::console::log_1(&"1.1".into());
    let rrule_set = rrule_set
        .after(after.with_timezone(&tz))
        .before(before.with_timezone(&tz));

    web_sys::console::log_1(&"2".into());

    let (events, _) = rrule_set.all(10000);

    web_sys::console::log_1(&"3".into());
    web_sys::console::log_1(&("events total: ".to_owned() + events.len().to_string().as_str()).into());

    println!("Events total: {:?}", events.len());
    
    let seq: Vec<i64> = events.iter()
        .map(|event| OccurrencePeriod::new(event.clone(), duration))
        .flat_map(|period| vec![period.start, period.end])
        .collect();
    
    js_sys::BigInt64Array::from(seq.as_slice())
}
