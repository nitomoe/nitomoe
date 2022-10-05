use askama::Template;

use crate::models::{board::Board, thread::Thread, post::Post, file::File};

#[derive(Template)]
#[template(path = "board.html", print = "code")]
pub struct BoardTemplate {
    pub board: Board
}