use std::{env, io};
use std::time::Duration;

use actix_files as fs;
use actix_web::{
    error, middleware, web, App, HttpResponse, HttpServer,
    Result,
};
use log::error;
use env_logger;
use tera::Tera;
use serde_derive::Deserialize;
use num_cpus;

use rust_birkana::document_from_string;

#[derive(Deserialize)]
pub struct FormData {
    text: String,
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse> {
    let body = tmpl.render("index.html.tera", &tera::Context::new())
        .map_err(|err| {
            error!("error rendering index template: {}", err);
            error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(body))
}

async fn generate(params: web::Form<FormData>) -> Result<HttpResponse> {
    let hex_string: String = params.text.bytes().map(|x| format!("{:x}", x)).collect();
    let document = document_from_string(hex_string);
    
    Ok(HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(document.to_string()))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "error,actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(index)),
            )
            .service(web::resource("/generate").route(web::post().to(generate)))
            .service(fs::Files::new("/static", "static"))
    })
    .keep_alive(Duration::new(65,0))
    .bind(("0.0.0.0", 8080))?
    .workers(num_cpus::get())
    .run()
    .await
}
