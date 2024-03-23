use tiberius::ToSql;
use std::sync::Arc;
#[derive(Debug)]
pub struct Game {
    sport_id: i32,
    event_id: i32,
    game_date: String,
    final_score: Option<i32>,
}

impl Game {
    pub fn new(
        sport_id: i32,
        event_id: i32,
        game_date: &str,
        final_score: Option<i32>,
    ) -> Self {
        Self {
            sport_id,
            event_id,
            game_date: game_date.to_string(),
            final_score,
        }
    }

    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Games (SportID, EventID, GameDate, FinalScore) VALUES (@P1, @P2, @P3, @P4);";
        let params = vec![
            Arc::new(self.sport_id.clone()) as Arc<dyn ToSql>,
            Arc::new(self.event_id.clone()),
            Arc::new(self.game_date.clone()),
            Arc::new(self.final_score.clone()),
        ];
        (query, params)
    }


}
