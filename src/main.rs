use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use factoring::*;
use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, Deserialize)]
struct Factoring {
    value: String,
}

#[get("/")]
async fn home() -> impl Responder {
    let generated = generate();
    let content = generated.get("index.html").unwrap();
    HttpResponse::Ok().body(content.data.to_vec())
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
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(web::resource("/api/factoring").route(web::post().to(factoring)))
    })
    .workers(50)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
