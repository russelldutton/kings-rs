use serde::Serialize;

#[derive(Serialize)]
pub struct Swap {
    pub id: i64,
    pub card_id: i64,
    pub donor_player_id: i64,
    pub recipient_player_id: i64,
}
