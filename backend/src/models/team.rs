use fake::Fake;
use rand;
use serde::{Deserialize, Serialize};
use tiberius;
use tiberius::ToSql;

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct Team {
    id: Option<i32>,
    team_name: String,
}

impl Team {
    pub fn to_insert_query(&self) -> (&str, Vec<Box<dyn tiberius::ToSql>>) {
        let query = "INSERT INTO Teams (Name) VALUES (@P1)";
        let params: Vec<Box<dyn tiberius::ToSql>> = vec![Box::new(self.team_name.clone())];
        (query, params)
    }

    pub fn generate_fake_team() -> Self {
        Self {
            id: None,
            team_name: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
