use actix_web::{get, Responder, HttpResponse};

static COMPILED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/app.css"));

#[get("app.css")]
pub async fn css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(COMPILED_CSS)
}