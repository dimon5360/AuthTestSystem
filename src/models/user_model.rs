
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct DetailUserInfo {
    pub username: String,
    pub password: String,
    pub email: String
}
