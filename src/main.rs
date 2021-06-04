use actix_web::{App, HttpServer, http::ContentEncoding, middleware};

extern crate learn_actix_web;

use learn_actix_web::{repositories::init_first_note, routes::{index, insert}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_singleton_value();

    let address = "127.0.0.1:7878";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(index)
            .service(insert)
    })
    .bind(address)?
    .run()
    .await
}

fn init_singleton_value() {
    init_first_note();
}