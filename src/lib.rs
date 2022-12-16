mod occurrence_period;
mod rrule_utils;
mod serializable;
mod utils;

use crate::occurrence_period::OccurrencePeriod;
use crate::rrule_utils::{get_cache_key, parse_between};
use crate::serializable::{Serializable, SerializableJs};
use crate::utils::set_panic_hook;
use chrono::{TimeZone, Utc};
use js_sys::{BigInt64Array, Date};
use lru::LruCache;
use once_cell::sync::Lazy;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CACHE: Lazy<Mutex<LruCache<String, Vec<OccurrencePeriod>>>> = Lazy::new(|| {
    let cache = LruCache::new(NonZeroUsize::new(1000).unwrap());
    Mutex::new(cache)
});

#[wasm_bindgen(js_name = "parseBetween")]
pub fn parse_between_js(
    ev: SerializableJs,
    start: Date,
    end: Date,
    include_partial: bool,
) -> js_sys::BigInt64Array {
    set_panic_hook();

    let ev = Serializable::from_js(ev);
    let start = Utc.timestamp_millis_opt(start.get_time() as i64).unwrap();
    let end = Utc.timestamp_millis_opt(end.get_time() as i64).unwrap();

    let occurrences = parse_between(&ev, start, end, include_partial);

    occurrences_to_bigint64array(&occurrences)
}

#[wasm_bindgen(js_name = "parseBetweenCached")]
pub fn parse_between_cached_js(
    ev: SerializableJs,
    start: Date,
    end: Date,
    include_partial: bool,
) -> js_sys::BigInt64Array {
    let ev = Serializable::from_js(ev);
    let start = Utc.timestamp_millis_opt(start.get_time() as i64).unwrap();
    let end = Utc.timestamp_millis_opt(end.get_time() as i64).unwrap();

    let mut cache = CACHE.lock().unwrap();
    let cache_key = get_cache_key(&ev, &start, &end, include_partial).to_owned();

    if cache.peek(&cache_key).is_some() {
        let cached = cache.get(&cache_key).unwrap();
        return occurrences_to_bigint64array(cached);
    }

    let occs = parse_between(&ev, start, end, include_partial);
    let result = occurrences_to_bigint64array(&occs);
    let occs_cache = occs.clone();

    cache.put(cache_key, occs_cache);
    return result;
}

fn occurrences_to_bigint64array(occurrences: &Vec<OccurrencePeriod>) -> BigInt64Array {
    let seq: Vec<i64> = occurrences
        .iter()
        .flat_map(|occ| vec![occ.start.timestamp_millis(), occ.end.timestamp_millis()])
        .collect();
    BigInt64Array::from(seq.as_slice())
}
