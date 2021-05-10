use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

use crate::errors::Error;
use crate::js_pouchdb::bindings::PouchDB;
use crate::types::DatabaseInfo;
use crate::utils::log;

pub struct Database {
    js_db: PouchDB,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            js_db: PouchDB::new(String::from(name)),
        }
    }

    pub async fn info(&self) -> Result<DatabaseInfo, Error> {
        log("Pouch: getting database info");
        JsFuture::from(self.js_db.info()).await?.try_into()
    }

    pub async fn close(&self) -> Result<(), Error> {
        JsFuture::from(self.js_db.close()).await?;
        Ok(())
    }
}
