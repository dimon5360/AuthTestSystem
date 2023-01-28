use actix_web::{get, web, HttpRequest, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/index.html"))
}

pub struct AppData {
    pub tmpl: Tera,
}

#[get("/main/{username}")]
pub async fn main(username: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<AppData>().unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", username.as_ref());
    let rendered = data.tmpl.render("authorized.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[get("/api/v1/auth")]
pub async fn auth() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; chatset=utf-8")
        .body(include_str!("../static/auth.html"))
}
