use rocket::serde::uuid::Uuid;

#[derive(rocket::serde::Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: Uuid,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub is_student: bool,
}
