mod mods;
use mods::test;

extern crate clap;
use clap::App;

use actix_web::{web, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    App::new("myapp")
        .version("1.0")
        .about("Does great things!")
        .author("Kevin K.")
        .get_matches();

    HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
