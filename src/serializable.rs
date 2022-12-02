use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::*;
use js_sys::{Date, JsString};

#[wasm_bindgen(js_name = "ISerializable")]
pub struct SerializableJs {
    start: Date,
    end: Date,
    until: Option<Date>,
    rrule: JsString
}

pub struct Serializable {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub until: Option<DateTime<Utc>>,
    pub rrule: String
}

impl Serializable {
    pub fn fromJs(serializable: SerializableJs) -> Serializable {
        Serializable {
            start: DateTime::from(serializable.start),
            end: DateTime::from(serializable.end),
            until: serializable.until.map(|date| DateTime::from(date)),
            rrule: serializable.rrule.into()
        }
    }
}