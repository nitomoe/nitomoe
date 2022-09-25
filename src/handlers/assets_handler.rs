use actix_web::{get, Responder, HttpResponse};

static APP_COMPILED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
static AUTH_COMPILED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/auth.css"));
static NORMALIZE_CSS: &'static str = include_str!("../../assets/css/normalize.css");

#[inline]
async fn render(data: &'static str) -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(data)
}

#[get("app.css")]
pub async fn app_css() -> impl Responder {
    render(APP_COMPILED_CSS).await
}

#[get("auth.css")]
pub async fn auth_css() -> impl Responder {
    render(AUTH_COMPILED_CSS).await
}

#[get("normalize.css")]
pub async fn normalize_css() -> impl Responder {
    render(NORMALIZE_CSS).await
}

#[get("sad.gif")]
pub async fn sad_img() -> impl Responder {
    // FIXME: Don't create a vector
    
    HttpResponse::Ok().content_type("image/gif").body(include_bytes!("../../assets/img/sad.gif").to_vec())
}