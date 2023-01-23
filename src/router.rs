
use actix_web::{get, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/index.html"))
}