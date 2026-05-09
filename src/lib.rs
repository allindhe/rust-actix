use actix_web::{App, HttpResponse, HttpServer, dev::Server, get};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();

    Ok(server)
}
