use std::time::UNIX_EPOCH;

use rocket::{
    http::Status,
    serde::{json::Json, uuid::Uuid},
    State,
};
use sha2::Digest;
use sqlx::MySqlPool;

use crate::models::user::{User, UserAuthClaims, UserLoginInfo, UserSignupInfo};

fn get_token_from_id(id: Uuid) -> String {
    use jsonwebtoken::*;
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claim = UserAuthClaims {
        exp: now + 15 * 60,
        id,
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(std::env::var("SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

#[post("/user/login", data = "<login>")]
pub async fn post_user_login(
    pool: &State<MySqlPool>,
    login: Json<UserLoginInfo>,
) -> Result<(Status, String), Status> {
    match sqlx::query!(
        "SELECT user_id AS `user_id: Uuid`, password AS `password: Vec<u8>` FROM user WHERE email = ?",
        login.email
    )
    .fetch_one(pool.inner())
    .await
    {
        Ok(record) => {
            let correct_hashed_password = record.password.as_slice();
            let salt = &correct_hashed_password[0..16];
            let salted = [salt, login.password.as_bytes()].concat();
            let mut hasher = sha2::Sha256::new();
            hasher.update(salted);
            let result = hasher.finalize();
            if result.as_slice() == &correct_hashed_password[16..] {
                let token = get_token_from_id(record.user_id);
                Ok((Status::Ok, token))
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(err) => {
            println!("{err:?}");
            Err(Status::NotFound)
        }
    }
}

#[post("/user/signup", data = "<input>")]
pub async fn post_user_signup(
    pool: &State<MySqlPool>,
    input: Json<UserSignupInfo>,
) -> Result<(Status, String), Status> {
    let salt = Uuid::new_v4();
    let salt = salt.as_bytes().as_slice();
    let password = input.password.as_bytes();

    let salted = [salt, password].concat();

    let mut hasher = sha2::Sha256::new();
    hasher.update(salted);
    let result = hasher.finalize();
    let hashed_password = [salt, result.as_slice()].concat();

    let uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO user (user_id, name_first, name_last, email, is_student, password) VALUES (?, ?, ?, ?, ?, ?)",
        uuid,
        input.user_info.name_first,
        input.user_info.name_last,
        input.user_info.email,
        input.user_info.is_student,
        hashed_password
    )
    .execute(pool.inner())
    .await
    .unwrap();

    let token = get_token_from_id(uuid);

    Ok((Status::Ok, token))
}

#[get("/user/info?<user_id>")]
pub async fn get_user_info(
    user_id: Uuid,
    state: &State<MySqlPool>,
) -> Result<(Status, Json<User>), Status> {
    match sqlx::query_as!(
        User,
        r#"SELECT name_first, name_last, email, is_student AS `is_student: bool`
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

#[get("/user/class_list")]
pub async fn get_user_class_list(
    state: &State<MySqlPool>,
    auth: UserAuthClaims,
) -> Json<Vec<Uuid>> {
    match sqlx::query!(
        r#"SELECT class_id AS `class_id: Uuid`
        FROM class_user
        WHERE user_id = ?"#,
        auth.id,
    )
    .fetch_all(state.inner())
    .await
    {
        Ok(classes) => Json(classes.into_iter().map(|record| record.class_id).collect()),
        Err(_) => Json(vec![]),
    }
}
