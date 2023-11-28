use rocket::serde::uuid::Uuid;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: Uuid,
    pub name_first: String,
    pub name_last: String,
}
