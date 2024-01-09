use rocket::{http::Status, serde::json::Json, State};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::user::UserAuthClaims;

#[get("/class/user_list?<class_id>")]
pub async fn get_class_user_list(
    class_id: Uuid,
    state: &State<MySqlPool>,
    auth: UserAuthClaims,
) -> Result<(Status, Json<Vec<Uuid>>), Status> {
    match sqlx::query!(
        r#"
        SELECT user_id AS `user_id: Uuid`
        FROM class_user
        WHERE class_id = ? AND EXISTS (
            SELECT 1
            FROM class_user
            WHERE class_id = ? AND user_id = ?
        )
        "#,
        class_id,
        class_id,
        auth.id
    )
    .fetch_all(state.inner())
    .await
    {
        Ok(records) => {
            let ids: Vec<Uuid> = records.into_iter().map(|record| record.user_id).collect();
            if ids.len() == 0 {
                Err(Status::NotFound)
            } else {
                Ok((Status::Ok, Json(ids)))
            }
        }
        Err(e) => {
            println!("{e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[post("/class/join?<class_id>")]
pub async fn post_class_join(
    class_id: Uuid,
    state: &State<MySqlPool>,
    auth: UserAuthClaims,
) -> Status {
    match sqlx::query!(
        r#"INSERT INTO class_user (class_id, user_id)
        VALUES (?, ?)"#,
        class_id,
        auth.id
    )
    .execute(state.inner())
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
