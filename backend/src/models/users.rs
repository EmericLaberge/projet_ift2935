
use fake::Fake;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tiberius::ToSql;



#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct User {
    id: Option<i32>,
    email: String,
    address: String,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
}

impl User {
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "EXEC spRegisterUser @Email = @P1, @Address = @P2, @FirstName = @P3, @LastName = @P4, @Username = @P5, @Password = @P6";
        let params = vec![
            Arc::new(self.email.clone()) as Arc<dyn ToSql>,
            Arc::new(self.address.clone()),
            Arc::new(self.first_name.clone()),
            Arc::new(self.last_name.clone()),
            Arc::new(self.username.clone()),
            Arc::new(self.password.clone()),
        ];
        (query, params)
    }
    pub fn to_alter_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query =
            "Update Users SET Address=@P2,FirstName=@P3,LastName=@P4,Password=@P5 WHERE ID = @P1;";
        let params = vec![
            Arc::new(self.id.clone()) as Arc<dyn ToSql>,
            Arc::new(self.address.clone()) as Arc<dyn ToSql>,
            Arc::new(self.first_name.clone()) as Arc<dyn ToSql>,
            Arc::new(self.last_name.clone()) as Arc<dyn ToSql>,
            Arc::new(self.password.clone()) as Arc<dyn ToSql>,
        ];
        (query, params)
    }

    pub fn get_user_teams_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "EXEC spGetUserTeams @UserID = @P1";
        let params = vec![Arc::new(self.id.clone()) as Arc<dyn ToSql>];
        (query, params)
    }

    pub fn get_users_events_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "EXEC spGetUserEvents @UserID = @P1";
        let params = vec![Arc::new(self.id.clone()) as Arc<dyn ToSql>];
        (query, params)
    }


    

    /// # Example
    /// ```
    /// let user = User::generate_fake_user();
    /// ```
    pub fn generate_fake_user() -> Self {
        Self {
            id: None,
            email: fake::faker::internet::en::FreeEmail().fake(),
            address: fake::faker::address::en::StreetName().fake(),
            first_name: fake::faker::name::en::FirstName().fake(),
            last_name: fake::faker::name::en::LastName().fake(),
            username: fake::faker::internet::en::Username().fake(),
            password: fake::faker::internet::en::Password(6..8).fake(),
        }
    }

    pub fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn change_address(&mut self, new_address: &str) {
        self.address = new_address.to_string();
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
}
