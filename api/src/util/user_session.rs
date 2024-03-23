use tower_sessions::Session;

use crate::common::app_error::AppError;

pub const USER_ID_KEY: &str = "user_id";

pub async fn get_user_id_from_session(session: Session) -> Result<i64, AppError> {
    let user_id = session
        .get::<i64>(USER_ID_KEY)
        .await?
        .expect("User not recognized");

    Ok(user_id)
}

pub async fn add_user_id_to_session(session: Session, user_id: i64) -> Result<(), AppError> {
    session.insert(USER_ID_KEY, user_id).await?;

    Ok(())
}
