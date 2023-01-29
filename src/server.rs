use actix_web::{App, get, HttpServer, HttpRequest, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tera::Tera;

use crate::handler;
use crate::router;
use crate::storage;

#[derive()]
pub struct Server {
    ip: String,
}

struct ServerData {
    db: actix::Addr<storage::PostgresHandler>
}

#[get("/read")]
async fn read_from_db(req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<ServerData>().unwrap();
    data.db.do_send(storage::AddUser {
        user_name: "".to_string(),
      });
    HttpResponse::Ok().body("pong")
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
            let tera = match Tera::new("static/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            
            App::new()
                .app_data(router::AppData { tmpl: tera })
                .app_data(ServerData { db: pg.clone() })
                .service(router::index)
                .service(router::auth)
                .service(router::ping)
                .service(router::main)
                .service(handler::auth)
                .service(handler::login)
                .service(handler::logout)
                .service(read_from_db)
        })
        .bind_openssl(&self.ip, builder)
        .expect(format!("Cannot bind to {}", &self.ip).as_str())
        .run()
        .await
    }
}
