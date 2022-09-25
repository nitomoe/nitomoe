use askama::Template;

#[derive(Template)]
#[template(path = "board.html")]
pub struct BoardTemplate;