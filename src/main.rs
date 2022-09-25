use actix_web::{
    App, get, HttpServer, HttpResponse, middleware::{Logger, NormalizePath}, Responder, web, http::StatusCode
};
use askama::Template;
use db::{establish_db_connection, PgPool};
use templates::{
    board_template::BoardTemplate,
    index_template::IndexTemplate,
    not_found_template::NotFoundTemplate,
};

mod db;
mod env;
mod error;
mod models;
mod schema;
mod service;
mod handlers;
mod templates;

async fn render_template<T: Template>(status_code: StatusCode, template: T) -> HttpResponse {
    match template.render() {
        Ok(rendered) => {
            HttpResponse::build(status_code).content_type("text/html").body(rendered)
        }
        Err(e) => {
            log::error!("Failed to render template: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn not_found_handler() -> HttpResponse {
    render_template(StatusCode::NOT_FOUND, NotFoundTemplate).await
}

#[get("/")]
async fn index_handler() -> impl Responder {
    render_template(StatusCode::OK, IndexTemplate).await
}

#[get("/{board}")]
async fn board_handler(
    pool: web::Data<PgPool>,
    path: web::Path<String>
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().map_err(actix_web::error::ErrorInternalServerError)?;
    let board_name = path.into_inner();

    let board = service::board_service::get_by_name(&mut conn, &board_name);

    match board {
        Ok(board) => {
            Ok(render_template(StatusCode::OK, BoardTemplate { board }).await)
        }
        Err(_e) => {
            Ok(not_found_handler().await)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=debug,debug");

    #[cfg(not(debug_assertions))]
    std::env::set_var("RUST_LOG", "actix_web=info,info");

    env_logger::init();

    let db_pool = web::Data::new(establish_db_connection());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(actix_web::middleware::TrailingSlash::Trim))
            .app_data(db_pool.clone())
            .service(index_handler)
            .service(
                web::scope("/assets/css")
                .service(handlers::assets_handler::normalize_css)
                .service(handlers::assets_handler::app_css)
                .service(handlers::assets_handler::auth_css)
            )
            .service(
                web::scope("/assets/js")
                .service(handlers::assets_handler::sidebar_js)
                .service(handlers::assets_handler::toaster_js)
            )
            .service(
                web::scope("/assets/img")
                .service(handlers::assets_handler::sad_img)
            )
            .service(board_handler)
            .default_service(web::get().to(not_found_handler))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}