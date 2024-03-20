use fake::{Fake, Faker};
use rand;
#[derive(Debug)]
pub struct Game {
    game_id: u16,
    sport_id: u16,
    event_id: u16,
    game_date: String,
    final_score: Option<u16>,
}

impl Game {
    pub fn new(game_id: u16, sport_id: u16, event_id: u16, game_date: &str, final_score: Option<u16>) -> Self {
        Self {
            game_id,
            sport_id,
            event_id,
            game_date: game_date.to_string(),
            final_score,
        }
    }

    pub fn to_insert_statement(&self) -> String {
        let final_score = match self.final_score {
            Some(score) => score.to_string(),
            None => "NULL".to_string(),
        };
        format!(
            "INSERT INTO GAME (Game_ID, Sport_ID, Event_ID, Game_Date, Final_Score) VALUES ({}, {}, {}, '{}', {})",
            self.game_id, self.sport_id, self.event_id, self.game_date, final_score
        )
    }
}
