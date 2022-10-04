use diesel::{
    PgConnection, QueryResult, RunQueryDsl
};
use crate::schema::posts::dsl::*;
use crate::models::post::{InsertPost, Post};

pub fn insert_post(conn: &mut PgConnection, post: InsertPost) -> QueryResult<Post> {
    diesel::insert_into(posts).values(post).get_result(conn)
}