use diesel::{
    Identifiable, Insertable, Queryable
};
use crate::schema::files;

#[derive(Identifiable, Queryable)]
pub struct File {
    pub id: i64,
    pub filename: String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub extension: String,
    pub post_id: i64,
    pub uid: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct InsertFile {
    pub filename: String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub extension: String,
    pub post_id: i64
}