use std::sync::Arc;

use fake::Fake;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tiberius::{self, ToSql};

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct Team {
    id: Option<i32>,
    team_name: String,
    team_level: String,
    team_type: String,
    sport: String,
}

// Enums to convert random int to the corresponding string
pub mod fake_data {
    pub mod team {
        pub fn level() -> String {
            let level = rand::random::<u8>() % 3;
            match level {
                0 => "Junior".to_string(),
                1 => "Recreational".to_string(),
                2 => "Competitive".to_string(),
                _ => "Junior".to_string(),
            }
        }

        pub fn team_type() -> String {
            let team_type = rand::random::<u8>() % 3;
            match team_type {
                0 => "Masculine".to_string(),
                1 => "Feminine".to_string(),
                2 => "Mixed".to_string(),
                _ => "Masculine".to_string(),
            }
        }
        pub fn sport() -> String {
            let sport = rand::random::<i32>() % 5;
            match sport {
                0 => "Soccer".to_string(),
                1 => "Basketball".to_string(),
                2 => "Volleyball".to_string(),
                3 => "Baseball".to_string(),
                4 => "Football".to_string(),
                _ => "Soccer".to_string(),
            }
        }
    }
}

impl Team {
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "EXEC spCreateTeam @Name = @p1, @TeamLevel = @p2, @TeamType = @p3, @SportName = @p4";
        let params = vec![
            Arc::new(self.team_name.clone()) as Arc<dyn ToSql>,
            Arc::new(self.team_level.clone()) as Arc<dyn ToSql>,
            Arc::new(self.team_type.clone()) as Arc<dyn ToSql>,
            Arc::new(self.sport.clone()) as Arc<dyn ToSql>,
        ];
        (query, params)
    }

    pub fn generate_fake_team() -> Self {
        Self {
            id: None,
            team_name: fake::faker::company::en::Buzzword().fake(),
            team_level: fake_data::team::level(),
            team_type: fake_data::team::team_type(),
            sport: fake_data::team::sport(),
        }
    }
}
