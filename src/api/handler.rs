
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tera::{Context, Tera};

use crate::models::user_model::UserInfo;
use crate::utils::utils::calc_hash;
use crate::storage::storage::*;

pub struct AppData {
    pub tmpl: Tera,
    pub db: actix::Addr<PostgresHandler>
}

#[get("/api/v1/index")]
pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../../api/v1/js/index.js"))
}

#[get("/api/v1/login.js")]
pub async fn get_login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../../api/v1/js/login.js"))
}

#[get("api/v1/auth.js")]
pub async fn get_auth() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../../api/v1/js/auth.js"))
}

#[get("api/v1/logout")]
pub async fn get_logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript; chatset=utf-8")
        .body(include_str!("../../api/v1/js/logout.js"))
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
pub async fn post_login(body: String, req: HttpRequest) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));
    let data = req.app_data::<AppData>().unwrap();

    let res: Result<UserInfo, serde_json::Error> = serde_json::from_str(&body);
    match res {
        Ok(login_data) => {

            let hash_username = calc_hash(&login_data.username);
            let hash_password = calc_hash(&login_data.password);

            data.db.do_send(AddUser {
                username: hash_username,
                password: hash_password
            });

            HttpResponse::Ok().json("hello")
        }
        Err(_) => HttpResponse::BadRequest().into()
    }
}

#[post("/api/v1/auth")] // sign in
pub async fn post_auth(body: String) -> HttpResponse {
    println!("{}", format!("Got thing: {:?}", body));

    std::thread::sleep(std::time::Duration::from_secs(1));

    HttpResponse::Ok().json("hello")
}