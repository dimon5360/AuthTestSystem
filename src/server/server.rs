use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tera::Tera;

use crate::api::handler;
use crate::api::handler::AppData;
use crate::api::router;
use crate::storage::storage;

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

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();

        let pg = storage::PostgresHandler::new();

        HttpServer::new(move || {
            let tera = match Tera::new("api/v1/html/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

            App::new()
                .app_data(AppData { tmpl: tera, db: pg.clone() })

                .service(router::get_index)
                .service(router::get_ping)
                .service(router::get_auth)
                .service(router::get_login)

                .service(handler::get_index)
                .service(handler::get_main)

                .service(handler::get_auth)
                .service(handler::get_login)
                .service(handler::get_logout)
                .service(handler::post_auth)
                .service(handler::post_login)
                .service(handler::post_logout)
        })
        .bind_openssl(&self.ip, builder)
        .expect(format!("Cannot bind to {}", &self.ip).as_str())
        .run()
        .await
    }
}
