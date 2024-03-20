use fake::{Fake, Faker};
use rand;

#[derive(Debug)]
pub struct Sport {
    sport_id: u16,
    sport_name: String,
    score_format: String,
}

impl Sport {
    pub fn new(sport_id: u16, sport_name: &str, score_format: &str) -> Self {
        Self {
            sport_id,
            sport_name: sport_name.to_string(),
            score_format: score_format.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO Sport (Sport_ID, Sport_Name, Score_Format) VALUES ({}, '{}', '{}')",
            self.sport_id, self.sport_name, self.score_format
        )
    }
}
