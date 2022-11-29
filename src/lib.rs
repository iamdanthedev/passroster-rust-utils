mod utils;
mod occurrence_period;
mod rrule_utils;

use wasm_bindgen::prelude::*;
use crate::rrule_utils::parse_rrule_between;

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


#[wasm_bindgen(js_name = "greet")]
pub fn greet(s: String) {
    web_sys::console::log_1(&"Hello, world!".into());
    println!("Hello, {}!", s);
}

#[wasm_bindgen(js_name = "parseRruleBetween")]
pub fn parse_rrule_between_ext(rrule: String, duration: i32, after_iso_str: String, before_iso_str: String) -> js_sys::BigInt64Array {
    let periods = parse_rrule_between(rrule, duration, after_iso_str, before_iso_str);
    
    let seq: Vec<i64> = periods.iter().flat_map(|period| vec![period.start, period.end]).collect();
    
    js_sys::BigInt64Array::from(seq.as_slice())
}

// .flat_map(|period| vec![period.start, period.end])
