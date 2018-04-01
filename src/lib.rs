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
use exonum::crypto::PublicKey;

use std::env;

use db_schema::superuser_keys;
use error::Error;
use superuser_key::NewSuperuserKey;

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

pub fn check_superuser_public_key(public_key: &PublicKey) -> Result<(), failure::Error> {
    let conn = establish_connection();

    let count = superuser_keys::table.count().get_result::<i64>(&conn)?;
    if count == 0 {
        let new_key = NewSuperuserKey {
            public_key: public_key.to_hex(),
        };
        diesel::insert_into(superuser_keys::table)
            .values(&new_key)
            .execute(&conn)?;
        return Ok(());
    }

    let key_exists = diesel::select(diesel::dsl::exists(
        superuser_keys::table.filter(superuser_keys::public_key.eq(public_key.to_hex())),
    )).get_result(&conn)?;

    if key_exists {
        Ok(())
    } else {
        Err(Error::BadSuperuserPublicKey(*public_key))?
    }
}
