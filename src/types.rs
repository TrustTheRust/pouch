use serde::Deserialize;
use serde_json::Value;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

use crate::errors::Error;
use pouch;
#[derive(Deserialize, Debug)]
pub struct DatabaseInfo {
    pub doc_count: i32,
    pub update_seq: i32,
    pub idb_attachment_format: String,
    pub db_name: String,
    pub auto_compaction: bool,
    pub adapter: String,
}

impl DatabaseInfo {
    pub fn new() -> Self {
        DatabaseInfo {
            db_name: String::from("unknown"),
            adapter: String::from("unknown"),
            idb_attachment_format: String::from("unknown"),
            doc_count: 0,
            update_seq: 0,
            auto_compaction: false,
        }
    }
}

impl TryFrom<JsValue> for DatabaseInfo {
    type Error = crate::errors::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let info: DatabaseInfo = value.into_serde().unwrap();
        Ok(info)
    }
}

#[derive(Deserialize, Debug)]
pub struct Document<T> {
    pub _id: String,
    pub data: T,
}

impl<T> TryFrom<JsValue> for Document<T> {
    type Error = crate::errors::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let _raw_doc: Value = value.into_serde().unwrap();

        Err(Error::Pouch("Not implementefd yet"))
    }
}
