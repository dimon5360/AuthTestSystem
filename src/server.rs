use actix_web::{App, HttpServer};
use tera::{Tera};

use crate::router;
use crate::handler;

#[derive()]
pub struct Server {
    ip: String,
}

impl Server {
    pub fn new(ip: &str) -> Server {
        Server { ip: ip.to_string() }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Start listening {}", self.ip);
        HttpServer::new(|| {

            let tera = match Tera::new("static/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

            App::new()
                .app_data(router::AppData {tmpl: tera})
                .service(router::index)
                .service(router::auth)
                .service(router::ping)
                .service(router::main)
                .service(handler::auth)
                .service(handler::login)
                .service(handler::logout)
        })
        .bind(&self.ip)
        .expect(format!("Cannot bind to {}", &self.ip).as_str())
        .workers(1)
        .run()
        .await
    }
}
