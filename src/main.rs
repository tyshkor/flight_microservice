mod models;
mod utils;
mod handlers;

use actix_web::{web::post, web, error, App, HttpResponse, HttpServer};

use crate::{
    handlers::search_path,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {

        App::new()
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                      error::InternalError::from_response(
                          "",
                          HttpResponse::BadRequest()
                              .content_type("application/json")
                              .body(format!(r#"{{"error":"{}"}}"#, err)),
                      )
                      .into()
        }))
        .route("/path", post().to(search_path))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}