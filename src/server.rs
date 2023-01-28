use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tera::Tera;

use crate::handler;
use crate::router;

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

        HttpServer::new(|| {
            let tera = match Tera::new("static/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

            App::new()
                .app_data(router::AppData { tmpl: tera })
                .service(router::index)
                .service(router::auth)
                .service(router::ping)
                .service(router::main)
                .service(handler::auth)
                .service(handler::login)
                .service(handler::logout)
        })
        .bind_openssl(&self.ip, builder)
        .expect(format!("Cannot bind to {}", &self.ip).as_str())
        .workers(1)
        .run()
        .await
    }
}
