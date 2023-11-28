use rocket::{
    http::Status,
    serde::{json::Json, uuid::Uuid},
    State,
};
use sqlx::MySqlPool;

use crate::models::user::User;

#[post("/user/login")]
pub async fn post_user_login() -> Status {
    Status::Ok
}

#[post("/user/signup")]
pub async fn post_user_signup() -> Status {
    // sqlx::query!(
    //     "INSERT INTO user (user_id, name_first, name_last, email) VALUES (?, ?, ?, ?)",
    //     Uuid::new_v4(),
    //     "Richie",
    //     "Zhang",
    //     "rqzhang@gmail.com"
    // )
    // .execute(&pool)
    // .await
    // .unwrap();
    Status::Ok
}

#[get("/user/info?<user_id>")]
pub async fn get_user_info(
    user_id: Uuid,
    state: &State<MySqlPool>,
) -> Result<(Status, Json<User>), Status> {
    match sqlx::query_as!(
        User,
        r#"SELECT user_id AS `user_id: Uuid`, name_first, name_last, email 
        FROM user 
        WHERE user_id = ?"#,
        user_id
    )
    .fetch_one(state.inner())
    .await
    {
        Ok(user) => Ok((Status::Ok, Json(user))),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

#[get("/user/list?<school_id>")]
pub async fn get_user_list(
    school_id: i32,
    state: &State<MySqlPool>,
) -> Result<(Status, Json<Vec<Uuid>>), Status> {
    match sqlx::query!(
        r#"SELECT user_id AS `user_id: Uuid`
        FROM user"#
    )
    .fetch_all(state.inner())
    .await
    {
        Ok(users) => Ok((
            Status::Ok,
            Json(users.into_iter().map(|x| x.user_id).collect()),
        )),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}
