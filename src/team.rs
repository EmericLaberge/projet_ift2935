use fake::{Fake, Faker};
use rand;
#[derive(Debug)]
pub struct Team {
    team_id: u16,
    team_name: String,
}

impl Team {
    pub fn new(team_id: u16, team_name: &str) -> Self {
        Self {
            team_id,
            team_name: team_name.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO TEAM (Team_ID, Team_Name) VALUES ({}, '{}')",
            self.team_id, self.team_name
        )
    }

    pub fn generate_fake_team() -> Self {
        Self {
            team_id: rand::random::<u16>(),
            team_name: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
