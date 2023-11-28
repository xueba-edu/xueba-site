#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;

#[launch]
fn rocket() -> _ {
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
