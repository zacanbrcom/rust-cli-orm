#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod models;

pub fn create_conn() -> Result<PgConnection, failure::Error> {
    dotenv::dotenv().ok();
    Ok(PgConnection::establish(&std::env::var("DATABASE_URL")?)?)
}