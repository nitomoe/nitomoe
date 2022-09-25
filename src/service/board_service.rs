// use diesel::{PgConnection, QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use crate::schema::boards::dsl::*;
use crate::models::board::Board;

pub fn get_by_name(conn: &mut PgConnection, board_name: &str) -> QueryResult<Board> {
    boards.filter(name.eq(board_name)).get_result(conn)
}