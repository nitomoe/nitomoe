use actix_web::{
    App, get, HttpServer, HttpResponse, middleware::{Logger, NormalizePath}, Responder, web, http::StatusCode
};
use askama::Template;
use db::{establish_db_connection, PgPool};
use models::file;
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

async fn render_template<T: Template>(status_code: StatusCode, template: &T) -> HttpResponse {
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
    render_template(StatusCode::NOT_FOUND, &NotFoundTemplate).await
}

#[get("/")]
async fn index_handler() -> impl Responder {
    render_template(StatusCode::OK, &IndexTemplate).await
}

#[get("/{board}")]
async fn board_handler(
    pool: web::Data<PgPool>,
    path: web::Path<String>
) -> Result<HttpResponse, actix_web::Error> {
    use crate::models::thread::Thread;
    use diesel::{BelongingToDsl, RunQueryDsl};

    let mut conn = pool.get().map_err(actix_web::error::ErrorInternalServerError)?;
    let board_name = path.into_inner();

    let board = service::board_service::get_by_name(&mut conn, &board_name);
    
    {
        use diesel::GroupedBy;
        use crate::models::board::Board;
        use crate::models::thread::Thread;
        use crate::schema::boards;

        let boards: Vec<Board> = boards::table.load::<Board>(&mut conn).expect("error loading boards");
        let threads: Vec<Thread> = Thread::belonging_to(&boards).load::<Thread>(&mut conn).expect("error loading threads");
        let grouped_threads: Vec<Vec<Thread>> = threads.grouped_by(&boards);
        let result: Vec<(Board, Vec<Thread>)> = boards.into_iter().zip(grouped_threads).collect();

        log::warn!("{:#?}", result);
    }

    match board {
        Ok(board) => {
            let threads = Thread::belonging_to(&board).load::<Thread>(&mut conn).map_err(actix_web::error::ErrorInternalServerError)?;

            let template = BoardTemplate {
                board: board
            };

            // Ok(
            //     render_template(
            //         StatusCode::OK,
            //         &template
            //     )
            //     .await
            // )

            Ok(HttpResponse::Ok().body(template.render().unwrap()))
        }
        Err(e) => {
            log::error!("Board not found: {:?}", e);
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
            .app_data(actix_web::web::PayloadConfig::new(50_000_000)) // 50mb max payload size
            .service(index_handler)
            .service(
                web::scope("/assets/css")
                .service(handlers::assets_handler::normalize_css)
                .service(handlers::assets_handler::app_css)
                .service(handlers::assets_handler::auth_css)
            )
            .service(
                web::scope("/assets/js")
                .service(handlers::assets_handler::api_js)
                .service(handlers::assets_handler::sidebar_js)
                .service(handlers::assets_handler::toaster_js)
                .service(handlers::assets_handler::service_thread_js)
                .service(handlers::assets_handler::service_post_js)
            )
            .service(
                web::scope("/assets/img")
                .service(handlers::assets_handler::sad_img)
            )
            .service(
                web::scope("/api/v1")
                .service(handlers::api_handler::new_thread)
            )
            .service(board_handler)
            .default_service(web::get().to(not_found_handler))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}