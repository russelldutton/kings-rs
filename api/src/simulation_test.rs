#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use sqlx::SqlitePool;
    use std::{collections::HashMap, sync::Arc};
    use tower_sessions::cookie::Cookie;

    use crate::{
        common::app_state::AppState, create_app, entities::card::Card,
        models::player_model::PlayerModel,
    };

    #[tokio::test]
    async fn simulate_game() {
        let users = vec!["Russ", "Jack", "Bob", "Peter"];
        let mut user_cookies = HashMap::<String, Cookie>::new();

        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let _ = sqlx::migrate!().run(&pool).await;
        let state: Arc<AppState> = Arc::new(AppState { pool });

        let app = create_app(state.clone());
        let mut server = TestServer::new(app).unwrap();

        for user in users.iter() {
            let response = server
                .get(format!("/user/register/{}", user).as_str())
                .await;

            response.assert_status_success();
            user_cookies.insert(user.to_string(), response.cookie("id"));

            server.clear_cookies();
        }

        let host = users[0];
        server.add_cookie(user_cookies.get(host).unwrap().clone());

        let response = server.get("/game/lobby/create").await;
        response.assert_status_success();
        let player_model = response.json::<PlayerModel>();
        let session_code = player_model.session_code;
        let game_id = player_model.game_id;

        server.clear_cookies();

        for user in users.iter().skip(1) {
            server.add_cookie(user_cookies.get(*user).unwrap().clone());
            let response = server
                .get(format!("/game/lobby/join/{}", session_code).as_str())
                .await;
            response.assert_status_success();
            server.clear_cookies();
        }

        server.add_cookie(user_cookies.get(host).unwrap().clone());
        let prepared_game_respnose = server
            .get(format!("/game/{}/prepare", game_id).as_str())
            .await;

        prepared_game_respnose.assert_status_success();

        let cards = sqlx::query_as!(Card, "SELECT * FROM cards")
            .fetch_all(&state.pool)
            .await
            .unwrap();

        assert_eq!(cards.len(), 52);
    }
}
