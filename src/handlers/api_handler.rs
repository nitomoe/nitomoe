use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;

#[derive(Serialize)]
struct CreateThreadResponse {
    hello: String
}

#[post("/{board}/new_thread")]
pub async fn new_thread(
    path: web::Path<String>
) -> impl Responder {
    HttpResponse::Ok().json(CreateThreadResponse { hello: path.into_inner() } )
}