use actix_web::{App, HttpServer};

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
        HttpServer::new(|| {
            App::new()
                .service(router::main)
                .service(router::index)
                .service(router::ping)
                .service(router::login)
                .service(router::logout)
        })
        .bind(&self.ip)
        .expect(format!("Cannot bind to {}", &self.ip).as_str())
        .workers(1)
        .run()
        .await
    }
}
