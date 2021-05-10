#![doc(html_favicon_url = "https://www.pouch.rs/assets/images/favicon.ico")]
#![doc(html_logo_url = "https://www.pouch.rs/assets/images/logo.svg")]


mod database;
pub use crate::database::Database;

pub mod types;


pub mod errors;


mod js_pouchdb;


mod utils;









pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}









pub mod prelude {
    pub use crate::database::Database;
    pub use crate::errors::Error;
    pub use crate::types::DatabaseInfo;
}

pub use self::prelude::*;
