#[derive(Debug)]
pub struct Game {
    sport_id: u16,
    event_id: u16,
    game_date: String,
    final_score: Option<u16>,
}

impl Game {
    pub fn new(
        sport_id: u16,
        event_id: u16,
        game_date: &str,
        final_score: Option<u16>,
    ) -> Self {
        Self {
            sport_id,
            event_id,
            game_date: game_date.to_string(),
            final_score,
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO Games (SportID, EventID, GameDate, FinalScore) VALUES ({}, {}, '{}', {});",
            self.sport_id,
            self.event_id,
            self.game_date,
            self.final_score.unwrap_or(0)
        )
    }
}

