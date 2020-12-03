use actix_web::{get, web, Responder, HttpServer, App};
#[get("/{id}/{name}")]
async fn index(web::Path((id, name)):web::Path<(u32, String)>) -> impl Responder {
    format!("hello {} id:{}", id, name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(||  App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
