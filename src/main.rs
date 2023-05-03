mod api;
mod server;
mod storage;
mod models;
mod utils;

use server::server::Server;

fn parse_env(key: &str) -> Result<String, ()> {
    match std::env::var(key) {
        Ok(v) => return Ok(v),
        Err(e) => panic!("${} is not set ({})", key, e),
    }
}

#[actix_web::main]
async fn main() {
    let path = std::env::current_dir().unwrap();

    dotenv::from_path(format!("{}/{}", path.display().to_string(), "config/app.env")).unwrap();

    let version = parse_env("VERSION").unwrap();
    let host_ip = parse_env("SERVICE_HOST").unwrap();

    println!("Auth service version v.{} from {}",
        version,
        chrono::Local::now()
    );

    Server::new(host_ip.as_str()).run().await.unwrap()
}
