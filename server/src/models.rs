use crate::schema::posts;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub text: String,
}

impl Post {
    /// Creates and inserts a post into the database
    pub fn new(conn: &PgConnection, title: &str, text: &str) -> QueryResult<Post> {
        let new_post = NewPost {
            title: title.to_owned(),
            text: text.to_owned(),
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
    }
}
