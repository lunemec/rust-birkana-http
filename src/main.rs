#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rust_birkana;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

use rocket::http::ContentType;
use rocket::request::Form;
use rocket::response::content::Content;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use rust_birkana::document_from_string;

#[derive(FromForm)]
struct Text {
    text: String,
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
    };
    Template::render("index", &context)
}

#[get("/static/<file..>")]
fn staticfiles(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/generate", data = "<input_form>")]
fn generate(input_form: Form<Text>) -> Content<String> {
    let input = input_form.get();
    let hex_string: String = input.text.bytes().map(|x| format!("{:x}", x)).collect();
    let document = document_from_string(hex_string);
    Content(ContentType::SVG, document.to_string())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, staticfiles, generate])
        .attach(Template::fairing())
        .launch();
}
