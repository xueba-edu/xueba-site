use rocket::{
    http::Status,
    serde::{
        json::Json,
        uuid::Uuid,
    },
};

use crate::models::user::User;

#[post("/user/login")]
pub async fn post_user_login() -> Status {
    Status::Ok
}

#[post("/user/signup")]
pub async fn post_user_signup() -> Status {
    Status::Ok
}

#[get("/user/info?<user_id>")]
pub async fn get_user_info(user_id: Uuid) -> Result<(Status, Json<User>), Status> {
    Ok((
        Status::Ok,
        Json(User {
            user_id,
            name_first: "Richie".into(),
            name_last: "Zhang".into(),
        }),
    ))
}

#[get("/user/list?<school_id>")]
pub async fn get_user_list(school_id: i32) -> Result<(Status, Json<Vec<Uuid>>), Status> {
    Ok((
        Status::Ok,
        Json(vec![
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ]),
    ))
}
