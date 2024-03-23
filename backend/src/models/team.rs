use fake::Fake;
use rand;
use tiberius;
use tiberius::ToSql;
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

    pub fn to_insert_query(&self) -> (&str, Vec<Box<dyn tiberius::ToSql>>) {
        let query = "INSERT INTO Teams (Name) VALUES (@P1)";
        let params: Vec<Box<dyn tiberius::ToSql>> = vec![Box::new(self.team_name.clone())];
        (query, params)
    }
        

    pub fn generate_fake_team() -> Self {
        Self {
            team_name: fake::faker::company::en::Buzzword().fake(),
        }
    }
}
