use crate::models::user::User;
use sqlx::mysql::MySqlPoolOptions;
use rocket::serde::uuid::Uuid;

#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match sqlx::query_as!(
        User,
        "SELECT user_id AS `user_id: Uuid`, name_first, name_last, email FROM user"
    )
    .fetch_one(&pool)
    .await
    {
        Ok(user) => println!("{user:?}"),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                sqlx::query!(
                    "INSERT INTO user (user_id, name_first, name_last, email) VALUES (?, ?, ?, ?)",
                    Uuid::new_v4(),
                    "Richie",
                    "Zhang",
                    "rqzhang@gmail.com"
                )
                .execute(&pool)
                .await
                .unwrap();
            }
            _ => println!("{err:?}"),
        },
    }

    rocket::build().mount(
        "/api",
        routes![
            routes::user::post_user_login,
            routes::user::post_user_signup,
            routes::user::get_user_info,
            routes::user::get_user_list,
        ],
    )
}
