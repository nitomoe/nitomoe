use diesel::dsl::exists;
use diesel::{prelude::*, QueryDsl, RunQueryDsl, select};
use crate::schema::boards::dsl::*;
use crate::models::board::Board;

pub fn get_by_name(conn: &mut PgConnection, board_name: &str) -> QueryResult<Board> {
    boards.filter(name.eq(board_name)).get_result(conn)
}

pub fn exists_by_name(conn: &mut PgConnection, board_name: &str) -> QueryResult<bool> {
    select(exists(boards.filter(name.eq(board_name)))).get_result(conn)
}