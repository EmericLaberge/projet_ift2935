use fake::Fake;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tiberius::ToSql;

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct Player {
    id: Option<i32>,
    user_id: i32,
    team_id: i32,
}

impl Player {
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Players (UserID, TeamID) VALUES (@P1, @P2);";
        let params = vec![
            Arc::new(self.user_id.clone()) as Arc<dyn ToSql>,
            Arc::new(self.team_id.clone()) as Arc<dyn ToSql>,
        ];
        (query, params)
    }

    pub fn to_delete_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "DELETE FROM Players WHERE ID = @P1;";
        let params = vec![Arc::new(self.id.clone()) as Arc<dyn ToSql>];
        (query, params)
    }
}
