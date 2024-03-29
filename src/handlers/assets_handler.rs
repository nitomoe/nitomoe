use actix_web::{get, Responder, HttpResponse};

static APP_COMPILED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
static AUTH_COMPILED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/auth.css"));
static NORMALIZE_CSS: &'static str = include_str!("../../assets/css/normalize.css");

static API_JS: &'static str = include_str!("../../assets/js/api.js");
static SIDEBAR_JS: &'static str = include_str!("../../assets/js/sidebar.js");
static TOASTER_JS: &'static str = include_str!("../../assets/js/toaster.js");
static SERVICE_THREAD_JS: &'static str = include_str!("../../assets/js/service/thread.js");
static SERVICE_POST_JS: &'static str = include_str!("../../assets/js/service/post.js");

#[inline]
async fn render(mimetype: &str, data: &'static str) -> impl Responder {
    HttpResponse::Ok().content_type(mimetype).body(data)
}

#[get("app.css")]
pub async fn app_css() -> impl Responder {
    render("text/css", APP_COMPILED_CSS).await
}

#[get("auth.css")]
pub async fn auth_css() -> impl Responder {
    render("text/css", AUTH_COMPILED_CSS).await
}

#[get("normalize.css")]
pub async fn normalize_css() -> impl Responder {
    render("text/css", NORMALIZE_CSS).await
}

#[get("sad.gif")]
pub async fn sad_img() -> impl Responder {
    // FIXME: Don't create a vector
    
    HttpResponse::Ok().content_type("image/gif").body(include_bytes!("../../assets/img/sad.gif").to_vec())
}

#[get("api.js")]
pub async fn api_js() -> impl Responder {
    render("text/javascript", API_JS).await
}

#[get("sidebar.js")]
pub async fn sidebar_js() -> impl Responder {
    render("text/javascript", SIDEBAR_JS).await
}

#[get("toaster.js")]
pub async fn toaster_js() -> impl Responder {
    render("text/javascript", TOASTER_JS).await
}

#[get("service/thread.js")]
pub async fn service_thread_js() -> impl Responder {
    render("text/javascript", SERVICE_THREAD_JS).await
}

#[get("service/post.js")]
pub async fn service_post_js() -> impl Responder {
    render("text/javascript", SERVICE_POST_JS).await
}