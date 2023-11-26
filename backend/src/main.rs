#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
    "login failed cause this not implemented yet"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello, login])
}
