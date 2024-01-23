use jsonwebtoken::{DecodingKey, Validation};
use rocket::{
    http::Status,
    outcome::Outcome,
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub is_teacher: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSignupInfo {
    pub password: String,
    pub user_info: User,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserAuthClaims {
    pub exp: u64,
    pub id: Uuid,
}

use rocket::request::{self, FromRequest, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuthClaims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let mut auth_tokens = req.headers().get("Authorization");
        let token = auth_tokens.next();
        if let Some(token) = token {
            let token = token.split_once(" ");
            if let Some(token) = token {
                let token = token.1;
                let claims = jsonwebtoken::decode::<UserAuthClaims>(
                    token,
                    &DecodingKey::from_secret(std::env::var("SECRET").unwrap().as_ref()),
                    &Validation::new(jsonwebtoken::Algorithm::HS256),
                );
                return Outcome::Success(claims.unwrap().claims);
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
