#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate animals_svc;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

