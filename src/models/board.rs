use diesel::{
    Identifiable, Queryable
};
use crate::schema::boards;

#[derive(Debug, Identifiable, Queryable)]
pub struct Board {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}