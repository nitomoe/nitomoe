use actix_web::{
    App, get, HttpServer, HttpResponse, middleware::Logger, Responder, web, http
};
use askama::Template;
use crate::templates::index_template::IndexTemplate;

mod handlers;
mod templates;

async fn not_found_handler() -> impl Responder {
    ("Not Found", http::StatusCode::NOT_FOUND)
}

#[get("/")]
async fn index_handler() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(IndexTemplate.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=debug,debug");

    #[cfg(not(debug_assertions))]
    std::env::set_var("RUST_LOG", "actix_web=info,info");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(index_handler)
            .service(
                web::scope("/assets/css")
                .service(handlers::assets_handler::app_css)
                .service(handlers::assets_handler::auth_css)
            )
            .default_service(web::get().to(not_found_handler))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}