use askama::Template;

#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct NotFoundTemplate;