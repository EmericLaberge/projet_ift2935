use fake::Fake;
use rand;
#[derive(Debug)]
pub struct Team {
    team_name: String,
}

impl Team {
    pub fn new(team_name: &str) -> Self {
        Self {
            team_name: team_name.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO TEAM (Team_Name) VALUES ('{}')",
            self.team_name
        )
    }

    pub fn generate_fake_team() -> Self {
        Self {
            team_name: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
