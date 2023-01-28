mod handler;
mod router;
mod server;

const BUILD: u8 = 5;
const MINOR: u8 = 0;
const MAJOR: u8 = 0;

#[actix_web::main]
async fn main() {
    println!(
        "Core service version v.{}.{}.{} from {}",
        MAJOR,
        MINOR,
        BUILD,
        chrono::Local::now()
    );
    let path = std::env::current_dir().unwrap();

    dotenv::from_path(format!("{}/{}", path.display().to_string(), "config/.env")).unwrap();
    let host = "SERVICE_HOST";

    match std::env::var(host) {
        Ok(ip) => server::Server::new(ip.as_str()).run().await.unwrap(),
        Err(e) => panic!("${} is not set ({})", host, e),
    }
}
