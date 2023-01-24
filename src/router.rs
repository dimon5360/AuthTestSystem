use actix_web::{get, post, HttpResponse};

#[get("/")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/login.html"))
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[get("/scripts/login.js")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../scripts/login.js"))
}

#[post("/api/v1/auth/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/login.html"))
}

#[post("/api/v1/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/index.html"))
}
