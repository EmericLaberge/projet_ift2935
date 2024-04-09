use serde::{Deserialize, Serialize};
use tiberius::ToSql;
use std::sync::Arc;
#[derive(Debug, derive_new::new, Serialize, Deserialize)]
pub struct Game {
    id: Option<i32>,
    sport_name: String,
    event_id: i32,
    first_team_id: i32,
    second_team_id: i32,
    game_date: String,
    final_score: String,
}

impl Game {


            
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Games (SportName, EventID, FirstTeamID, SecondTeamID, GameDate, FinalScore) VALUES (@P1, @P2, @P3, @P4, @P5, @P6)";
        let params: Vec<Arc<dyn ToSql>> = vec![
            Arc::new(self.sport_name.clone()),
            Arc::new(self.event_id),
            Arc::new(self.first_team_id),
            Arc::new(self.second_team_id),
            Arc::new(self.game_date.clone()),
            Arc::new(self.final_score.clone()),
        ];
        (query, params)
    }

    pub(crate) fn new(get_1: _, to_owned_1: _, get_2: _, get_3: _, get_4: _, to_owned_2: _, to_owned_3: _) -> _ {
        todo!()
    }


}
