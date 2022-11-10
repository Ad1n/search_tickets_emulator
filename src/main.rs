pub mod ticket;

use std::{convert::Infallible, io};

use actix_files::{Files, NamedFile};
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use async_stream::stream;

use std::error::Error;
// use std::fmt::Error;
use std::fs;
use std::path::Path;

use heed::bytemuck::{Pod, Zeroable};
use heed::byteorder::BE;
use heed::types::*;
use heed::{Database, EnvOpenOptions};
// use serde::{Deserialize, Serialize};

// async fn batch_insert(body: web::Bytes) -> Result<HttpResponse, Error> {
//     // body is loaded, now we can deserialize json-rust
//     let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
//     let injson: JsonValue = match result {
//         Ok(v) => v,
//         Err(e) => json::object! {"err" => e.to_string() },
//     };
//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .body(injson.dump()))
// }

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Init LMDB
    let env_path = Path::new("target").join("test-database.mdb");
    let _ = fs::remove_dir_all(&env_path);
    fs::create_dir_all(&env_path)?;
    let env = EnvOpenOptions::new()
        .map_size(10 * 1024 * 1024) // 10MB
        .max_dbs(3)
        .open(&env_path)
        .unwrap();
    let db: Database<md5::Digest, ticket::SimpleTicket> =
        env.create_database(Some("test")).unwrap();

    log::info!("Starting HTTP server at http://localhost:2989");

    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            // .service(web::resource("/batch_insert").route(web::post().to(batch_insert)))
            // .service(web::resource("/search").route(web::post().to(search)))
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(Files::new("/static", "static").show_files_listing())
            // redirect
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "static/welcome.html"))
                        .finish()
                })),
            )
            // default
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 2989))?
    .workers(2)
    .run()
    .await
}
