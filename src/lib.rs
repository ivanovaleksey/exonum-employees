#[macro_use]
extern crate exonum;
#[macro_use]
extern crate lazy_static;
extern crate bodyparser;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod api;
mod error;
pub mod schema;
pub mod service;
pub mod transactions;
