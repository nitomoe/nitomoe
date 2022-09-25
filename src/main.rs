use actix_web::{
    App, get, HttpServer, HttpResponse, middleware::Logger, Responder, web, http::StatusCode
};
use askama::Template;
use templates::{
    board_template::BoardTemplate,
    index_template::IndexTemplate,
    not_found_template::NotFoundTemplate,
};

mod handlers;
mod templates;

fn render_template<T: Template>(status_code: StatusCode, template: T) -> HttpResponse {
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
    render_template(StatusCode::NOT_FOUND, NotFoundTemplate)
}

#[get("/")]
async fn index_handler() -> impl Responder {
    render_template(StatusCode::OK, IndexTemplate)
}

#[get("/{board}")]
async fn board_handler(path: web::Path<String>) -> HttpResponse {
    let board_name = path.to_owned();

    if board_name == "neet" {
        render_template(StatusCode::OK, BoardTemplate)
    }
    else {
        not_found_handler().await
    }
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
                .service(handlers::assets_handler::normalize_css)
                .service(handlers::assets_handler::app_css)
                .service(handlers::assets_handler::auth_css)
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