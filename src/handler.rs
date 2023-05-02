
use crate::models;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tera::{Context, Tera};

pub struct AppData {
    pub tmpl: Tera,
}

#[get("/api/v1/index")]
pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../api/v1/js/index.js"))
}

#[get("/api/v1/login.js")]
pub async fn get_login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../api/v1/js/login.js"))
}

#[get("api/v1/auth.js")]
pub async fn get_auth() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../api/v1/js/auth.js"))
}

#[get("api/v1/logout")]
pub async fn get_logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../api/v1/js/logout.js"))
}


#[get("api/v1/main/{username}")]
pub async fn get_main(username: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<AppData>().unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", username.as_ref());
    let rendered = data.tmpl.render("authorized.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/api/v1/logout")]
pub async fn post_logout(body: String) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));

    HttpResponse::Ok().into()
}


#[post("/api/v1/login")] // log in
pub async fn post_login(body: String) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));

    models::user_model::hello();

    std::thread::sleep(std::time::Duration::from_secs(1));

    HttpResponse::Ok().json("hello")
}

#[post("/api/v1/auth")] // sign in
pub async fn post_auth(body: String) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));

    models::user_model::hello();

    std::thread::sleep(std::time::Duration::from_secs(1));

    HttpResponse::Ok().json("hello")
}