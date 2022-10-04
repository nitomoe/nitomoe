use diesel::{
    PgConnection, QueryResult, RunQueryDsl
};
use crate::models::thread::{
    InsertThread, Thread
};
use crate::schema::threads::dsl::*;

pub fn insert_thread(conn: &mut PgConnection, thread: InsertThread) -> QueryResult<Thread> {
    diesel::insert_into(threads).values(thread).get_result(conn)
}