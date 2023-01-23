use actix_web::{App, HttpServer};

use crate::router;

#[derive()]
pub struct Server{
    ip: String
}

impl Server {

    pub fn new(ip: &str) -> Server {
        Server{ ip: ip.to_string() }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Start listening {}", self.ip);
        HttpServer::new(|| {
            App::new()
                .service(router::hello)
                .service(router::echo)
        })
        .bind(&self.ip)?
        .run()
        .await
    }
}
