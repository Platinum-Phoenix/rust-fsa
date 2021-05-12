pub mod models;
pub mod schema;

#[macro_use]
pub extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::{NewPost, Post};
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

