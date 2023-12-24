use sqlx::mysql::MySqlPoolOptions;

#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;
pub mod utils;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // let allowed_origins = rocket_cors::AllowedOrigins::some_exact(&[""]);

    let cors = rocket_cors::CorsOptions {
        // allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build().manage(pool).attach(cors).mount(
        "/api",
        routes![
            routes::user::post_user_login,
            routes::user::post_user_signup,
            routes::user::get_user_info,
            routes::user::get_user_list,
            routes::class::post_class_join,
        ],
    )
}
