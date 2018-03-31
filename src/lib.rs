#[macro_use]
extern crate exonum;
extern crate bodyparser;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate diesel;
extern crate toml;

use diesel::PgConnection;
use diesel::prelude::*;

use std::env;

mod api;
pub mod config;
pub mod db_schema;
pub mod error;
pub mod schema;
pub mod service;
pub mod superuser_key;
pub mod transactions;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
