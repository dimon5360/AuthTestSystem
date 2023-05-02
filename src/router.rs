use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../api/v1/html/index.html"))
}

#[get("/api/v1/login")]
pub async fn get_login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../api/v1/html/login.html"))
}

#[get("/api/v1/auth")]
pub async fn get_auth() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../api/v1/html/auth.html"))
}

#[get("/ping")]
pub async fn get_ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}
