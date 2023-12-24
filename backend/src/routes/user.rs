use rocket::{
    http::Status,
    serde::{json::Json, uuid::Uuid},
    State,
};
use sha2::Digest;
use sqlx::MySqlPool;
use uuid::uuid;

use crate::models::user::User;

#[post("/user/login")]
pub async fn post_user_login(pool: &State<MySqlPool>) -> Status {
    let user_id = uuid!("e7e8ed68-67f4-4dcc-ba74-cade10e73b9d");
    match sqlx::query!(
        "SELECT password AS `password: Vec<u8>` FROM user WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool.inner())
    .await
    {
        Ok(record) => {
            let correct_hashed_password = record.password.as_slice();
            let salt = &correct_hashed_password[0..16];
            // TODO: make this actually work cause i forgot how we create password
        }
        Err(err) => println!("{err:?}"),
    }
    Status::Ok
}

#[post("/user/signup")]
pub async fn post_user_signup(pool: &State<MySqlPool>) -> Status {
    // TODO: actually explain what is going on cause idk i forgot
    let salt = Uuid::new_v4();
    let salt = salt.as_bytes().as_slice();
    let password = b"hello world";

    let salted = [salt, password].concat();

    let mut hasher = sha2::Sha256::new();
    hasher.update(salted);
    let result = hasher.finalize();
    let hashed_password = [salt, result.as_slice()].concat();

    sqlx::query!(
        "INSERT INTO user (user_id, name_first, name_last, email, is_student, password) VALUES (?, ?, ?, ?, ?, ?)",
        Uuid::new_v4(),
        "asdadasdada",
        "asdad",
        "asdadasdsad@gmail.com",
        true,
        hashed_password
    )
    .execute(pool.inner())
    .await
    .unwrap();

    Status::Ok
}

#[get("/user/info?<user_id>")]
pub async fn get_user_info(
    user_id: Uuid,
    state: &State<MySqlPool>,
) -> Result<(Status, Json<User>), Status> {
    match sqlx::query_as!(
        User,
        r#"SELECT user_id AS `user_id: Uuid`, name_first, name_last, email, is_student AS `is_student: bool`
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

#[get("/user/list?<class_id>")]
pub async fn get_user_list(
    class_id: Uuid,
    state: &State<MySqlPool>,
) -> Result<(Status, Json<Vec<Uuid>>), Status> {
    match sqlx::query!(
        // r#"
        // SELECT user_id AS `user_id: Uuid`
        // FROM user
        // "#
        r#"SELECT user_id AS `user_id: Uuid`
        FROM class_user
        WHERE class_id = ?"#,
        class_id
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
