use actix_web::{get, http::StatusCode, App, HttpResponse, HttpServer};
use std::fs::read_to_string;

use actix_files as fs;

use ssr_rs::SSREnvironment;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/styles", "client/dist/ssr/styles/").show_files_listing())
            .service(fs::Files::new("/images", "client/dist/ssr/images/").show_files_listing())
            .service(fs::Files::new("/scripts", "client/dist/client/").show_files_listing())
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    let source = read_to_string("./client/dist/ssr/index.js").unwrap();
    let mut env = SSREnvironment::new(&source, "SSR", "Index");

    let mock_props = r##"{
        "params": [
            "hello",
            "ciao",
            "こんにちは" 
        ]
    }"##;

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(env.render(mock_props))
}
