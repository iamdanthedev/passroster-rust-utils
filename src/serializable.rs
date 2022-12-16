use chrono::{DateTime, TimeZone, Utc};
use js_sys::{Date, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SerializableJs {
    start: Date,
    end: Date,
    until: Option<Date>,
    rrule: JsString,
}

#[wasm_bindgen]
impl SerializableJs {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Date, end: Date, until: Option<Date>, rrule: JsString) -> SerializableJs {
        SerializableJs {
            start,
            end,
            until,
            rrule,
        }
    }

    pub fn get_start(&self) -> Date {
        self.start.clone()
    }
}

pub struct Serializable {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub until: Option<DateTime<Utc>>,
    pub rrule: String,
}

impl Serializable {
    pub fn from_js(serializable: SerializableJs) -> Serializable {
        Serializable {
            start: Utc
                .timestamp_millis_opt(serializable.start.get_time() as i64)
                .unwrap(),
            end: Utc
                .timestamp_millis_opt(serializable.end.get_time() as i64)
                .unwrap(),
            until: serializable
                .until
                .map(|date| Utc.timestamp_millis_opt(date.get_time() as i64).unwrap()),
            rrule: serializable.rrule.into(),
        }
    }
}
