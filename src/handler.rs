use actix_web::{get, post, HttpResponse};

#[get("/scripts/auth")]
pub async fn auth() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../scripts/auth.js"))
}

#[post("/api/v1/auth/login")]
pub async fn login(body: String) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));

    HttpResponse::Ok().json("hello")
}

#[post("/api/v1/auth/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/index.html"))
}
