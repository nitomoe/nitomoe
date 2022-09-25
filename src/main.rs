use actix_web::{
    App, HttpServer, middleware::Logger, Responder, web, http
};

async fn not_found_handler() -> impl Responder {
    ("Not Found", http::StatusCode::NOT_FOUND)
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
            .default_service(web::get().to(not_found_handler))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}