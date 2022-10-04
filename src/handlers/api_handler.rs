use std::{net::IpAddr, str::FromStr, path::Path};

use actix_multipart_extract::{MultipartForm, Multipart, File};
use actix_web::{HttpResponse, post, web, HttpRequest};
use diesel::PgConnection;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, error::Error, service::{board_service, thread_service, post_service, poster_service, file_service}, models::{thread::{Thread, InsertThread}, post::{InsertPost, Post}, file::InsertFile}, env};

async fn create_thread(
    conn: &mut PgConnection,
    ip_network: IpNetwork,
    board_name: &str,
    create_thread_input: &CreateThreadInput
) -> Result<CreateThreadResponse, Error> {
    let board = board_service::get_by_name(conn, board_name).map_err(|_| Error::BoardDoesNotExist)?;

    if create_thread_input.file.is_none() {
        return Err(Error::NewThreadMissingFile)
    }

    let file = create_thread_input.file.as_ref().unwrap();

    let subject = &create_thread_input.subject;
    let body = &create_thread_input.body;

    conn.build_transaction().run::<(Thread, Post), diesel::result::Error, _>(|conn| {
        let insert_thread = InsertThread {
            subject: subject.to_owned(),
            board_id: board.id
        };
        let thread = thread_service::insert_thread(conn, insert_thread)?;
        
        let poster = poster_service::get_or_insert_poster(conn, ip_network)?;

        let insert_post = InsertPost {
            thread_id: thread.id,
            poster_id: poster.id,
            body: body.to_owned()
        };
        let post = post_service::insert_post(conn, insert_post)?;

        let ext = match file.content_type.as_str() {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/gif" => "gif",
            _ => todo!()
        };

        let insert_file = InsertFile {
            filename: file.name.to_owned(),
            size: file.bytes.len() as i32,
            width: 0,
            height: 0,
            extension: ext.to_owned(),
            post_id: post.id
        };
        let inserted_file = file_service::insert_file(conn, insert_file)?;

        {
            let path_string = format!("{}/media/{}.{}", env::STORAGE_PATH.as_str(), inserted_file.uid, inserted_file.extension);

            let file_path = Path::new(&path_string);

            let _ = std::fs::write(file_path, &file.bytes);
        }
        {
            let path_string = format!("{}/media/{}s.{}", env::STORAGE_PATH.as_str(), inserted_file.uid, inserted_file.extension);

            let file_path = Path::new(&path_string);

            let _ = std::fs::write(file_path, &file.bytes);
        }

        Ok((thread, post))
    })?;

    Ok(CreateThreadResponse::new_success())
}

#[derive(Serialize)]
struct CreateThreadResponse {
    success: bool,
    message: String
}

impl CreateThreadResponse {
    fn new_success() -> Self {
        Self { success: true, message: "success".to_owned() }
    }
    fn new_failure(message: String) -> Self {
        Self { success: false, message: message }
    }
}

#[derive(Debug, Deserialize, MultipartForm)]
pub struct CreateThreadInput {
    #[serde(rename = "boardName")]
    board_name: String,
    subject: Option<String>,
    key: Option<String>,
    body: Option<String>,
    #[multipart(max_size = 20MB)]
    file: Option<File>
}

#[post("/{board}/new_thread")]
pub async fn new_thread(
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
    path: web::Path<String>,
    form: Multipart<CreateThreadInput>
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().map_err(actix_web::error::ErrorInternalServerError)?;

    let connection_info = req.connection_info();
    let ip_addr_str = connection_info.realip_remote_addr().unwrap();
    let ip_addr = IpAddr::from_str(ip_addr_str).map_err(actix_web::error::ErrorInternalServerError)?;
    let ip_network = IpNetwork::new(ip_addr, 32).map_err(actix_web::error::ErrorInternalServerError)?;

    match create_thread(
        &mut conn,
        ip_network,
        path.as_str(),
        &form
    ).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(HttpResponse::Ok().json(CreateThreadResponse::new_failure(e.to_string())))
    }
}