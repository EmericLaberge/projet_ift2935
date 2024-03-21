use fake::Fake;
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

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO Sports (Name, ScoreFormat) VALUES ('{}', '{}');",
            self.sport_name, self.score_format
        )
    }

    pub fn generate_fake_sport() -> Self {
        Self {
            sport_name: fake::faker::company::en::Profession().fake(),
            score_format: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
