#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use itertools::Itertools;
    use sqlx::SqlitePool;
    use std::{collections::HashMap, sync::Arc};

    use crate::{common::app_state::AppState, create_app};

    #[tokio::test]
    async fn test_is_running() {
        assert!(true);

        let _users = HashMap::<&str, i64>::new();

        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let _ = sqlx::migrate!().run(&pool).await;
        let state: Arc<AppState> = Arc::new(AppState { pool });

        let app = create_app(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/user/register/Russ").await;
        response.assert_status_success();
        let _jar = response
            .cookies()
            .iter()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect_vec();
        let _headers = response
            .headers()
            .iter()
            .map(|h| (h.0.to_string(), h.1.to_str().unwrap()))
            .collect_vec();
        let cookie = response.cookie("id");
        assert_eq!(cookie.name().to_string(), "id");
    }
}
