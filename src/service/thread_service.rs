use diesel::{
    PgConnection, QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods
};
use crate::models::thread::{
    InsertThread, Thread
};
use crate::schema::threads::dsl::*;

pub fn insert_thread(conn: &mut PgConnection, thread: InsertThread) -> QueryResult<Thread> {
    diesel::insert_into(threads).values(thread).get_result(conn)
}

pub fn get_threads_for_board_id(conn: &mut PgConnection, thread_board_id: i32) -> QueryResult<Vec<Thread>> {
    threads
        .filter(
            board_id.eq(thread_board_id)
        )
        .order_by(updated_at.desc())
        .get_results(conn)
}