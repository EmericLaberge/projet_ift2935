use fake::Fake;
use rand;
use std::sync::Arc;
use tiberius::ToSql;
#[derive(Debug)]
pub struct Sport {
    sport_name: String,
    score_format: String,
}

impl Sport {
    pub fn new(sport_name: &str, score_format: &str) -> Self {
        Self {
            sport_name: sport_name.to_string(),
            score_format: score_format.to_string(),
        }
    }

    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Sports (SportName, ScoreFormat) VALUES (@P1, @P2);";
        let params = vec![
            Arc::new(self.sport_name.clone()) as Arc<dyn ToSql>,
            Arc::new(self.score_format.clone()),
        ];
        (query, params)
    }

    pub fn generate_fake_sport() -> Self {
        Self {
            sport_name: fake::faker::company::en::Profession().fake(),
            score_format: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
