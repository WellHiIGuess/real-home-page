mod app;
mod library;
mod elements;

use std::{path::Path, fs::File, io::prelude::*};

use crate::{app::home::home, library::js_packet::JSPacket};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use actix_files::Files;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[get("/")]
    async fn home_page() -> impl Responder {
        HttpResponse::Ok().body(home().serve())
    }

    #[get("/$get_js/{file}")]
    async fn get_js(file: web::Path<String>) -> impl Responder {
        HttpResponse::Ok().body(JSPacket::new(&file).to_string())
    }

    println!("Now serving at http://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new()
            .service(home_page)
            .service(get_js)
            .service(Files::new("/pub", "./src/pub"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
