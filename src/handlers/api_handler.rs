use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;
use futures_util::stream::StreamExt as _;

#[derive(Serialize)]
struct CreateThreadResponse {
    hello: String
}

#[post("/{board}/new_thread")]
pub async fn new_thread(
    path: web::Path<String>,
    mut payload: actix_multipart::Multipart
) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            log::info!("CHUNK: {:?}", std::str::from_utf8(&chunk.unwrap()));
        }
    }
    HttpResponse::Ok().json(CreateThreadResponse { hello: path.into_inner() } )
}