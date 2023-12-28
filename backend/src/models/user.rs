use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub is_student: bool,
}

#[derive(rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSignupInfo {
    pub password: String,
    pub user_info: User,
}
