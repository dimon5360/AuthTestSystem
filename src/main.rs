
use chrono::Local;
use dotenv;
use std::env;

const BUILD: u8 = 1;
const MINOR: u8 = 0;
const MAJOR: u8 = 0;

fn main() {
    println!("Core service version v.{}.{}.{} from {}", MAJOR, MINOR, BUILD, Local::now());
    let path = env::current_dir().unwrap();

    dotenv::from_path(format!("{}/{}", path.display().to_string(), "config/.env")).unwrap();
    let host = "SERVICE_HOST";
    
    match env::var(host) {
        Ok(v) => println!("{}, {}", host, v),
        Err(e) => panic!("${} is not set ({})", host, e)
    }
}
