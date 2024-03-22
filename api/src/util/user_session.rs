use tower_sessions::Session;

use crate::{common::app_error::AppError, USER_ID_KEY};

pub async fn get_user_id_from_session(session: Session) -> Result<i64, AppError> {
    let user_id = session
        .get::<i64>(USER_ID_KEY)
        .await?
        .expect("User not recognized");

    Ok(user_id)
}
