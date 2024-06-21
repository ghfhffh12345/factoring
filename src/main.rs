use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use factoring::*;
use serde::Deserialize;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HOME: String = fs::read_to_string("./src/index.html").unwrap();
}

#[derive(Debug, Deserialize)]
struct Factoring {
    value: String,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body((*HOME).to_string())
}

async fn factoring(req_body: web::Json<Factoring>) -> Result<HttpResponse> {
    let expression = req_body.value.trim().to_string();

    let mut particles = data_preprocessing(&expression);

    transposition(&mut particles);

    let particles = organize_term(particles);

    let result = factorization(&particles).unwrap();

    Ok(HttpResponse::Ok().body(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(web::resource("/api/factoring").route(web::post().to(factoring)))
    })
    .workers(50)
    .bind(("localhost", 8080))?
    .run()
    .await
}
