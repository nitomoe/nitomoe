use askama::Template;

use crate::models::{board::Board, thread::Thread, post::Post, file::File};

#[derive(Template)]
#[template(path = "board.html")]
pub struct BoardTemplate {
    pub board: (Board, Vec<(Thread, Vec<Post>)>)
}