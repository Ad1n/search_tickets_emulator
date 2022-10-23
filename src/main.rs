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
