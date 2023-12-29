use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub is_student: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSignupInfo {
    pub password: String,
    pub user_info: User,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserAuthClaims {
    pub id: Uuid,
}
