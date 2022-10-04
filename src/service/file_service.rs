use diesel::{
    PgConnection, QueryResult, RunQueryDsl
};
use crate::schema::files::dsl::*;
use crate::models::file::{InsertFile, File};

pub fn insert_file(conn: &mut PgConnection, file: InsertFile) -> QueryResult<File> {
    diesel::insert_into(files).values(file).get_result(conn)
}