mod app;
mod library;
mod elements;

use crate::app::home::home;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[get("/")]
    async fn home_page() -> impl Responder {
        HttpResponse::Ok().body(home().serve())
    }

    HttpServer::new(|| {
        App::new()
            .service(home_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
