use diesel::{
    Identifiable, Insertable, Queryable
};
use crate::schema::posts;

#[derive(Identifiable, Queryable)]
pub struct Post {
    pub id: i64,
    pub num: i64,
    pub thread_id: i64,
    pub poster_id: i64,
    pub body: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct InsertPost {
    pub thread_id: i64,
    pub poster_id: i64,
    pub body: Option<String>
}