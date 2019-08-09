use std::{env, io};

use actix_files as fs;
use actix_web::{
    error, middleware, web, App, HttpResponse, HttpServer,
    Result,
};
use log::{error};
use env_logger;
use tera::{compile_templates};
use serde_derive::{Deserialize};

use rust_birkana::document_from_string;

#[derive(Deserialize)]
pub struct FormData {
    text: String,
}

fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse> {
    let body = tmpl.render("index.html.tera", &tera::Context::new())
        .map_err(|err| {
            error!("error rendering index template: {}", err);
            error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(body))
}

fn generate(params: web::Form<FormData>) -> Result<HttpResponse> {
    let hex_string: String = params.text.bytes().map(|x| format!("{:x}", x)).collect();
    let document = document_from_string(hex_string);
    
    Ok(HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(document.to_string()))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "error,actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| {
        let tera =
            compile_templates!("templates/**/*");

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(index)),
            )
            .service(web::resource("/generate").route(web::post().to(generate)))
            .service(fs::Files::new("/static", "static"))
    })
    .keep_alive(65)
    .bind("127.0.0.1:8080")?
    .start();

    sys.run()
}
