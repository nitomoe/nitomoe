use askama::Template;

use crate::models::board::Board;

#[derive(Template)]
#[template(path = "board.html")]
pub struct BoardTemplate {
    pub board: Board
}