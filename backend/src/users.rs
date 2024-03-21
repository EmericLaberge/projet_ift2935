use fake::Fake;
use rand;
use std::sync::Arc;
use tiberius::ToSql;
#[derive(Debug)]
pub struct User {
    email: String,
    address: String,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(email: &str, address: &str, first_name: &str, last_name: &str) -> Self {
        Self {
            email: email.to_string(),
            address: address.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Users (Email, Address, FirstName, LastName) VALUES (@P1, @P2, @P3, @P4);";
        let params = vec![
            Arc::new(self.email.clone()) as Arc<dyn ToSql>,
            Arc::new(self.address.clone()),
            Arc::new(self.first_name.clone()),
            Arc::new(self.last_name.clone()),
        ];
        (query, params)
    }

    pub fn generate_fake_user() -> Self {
        Self {
            email: fake::faker::internet::en::FreeEmail().fake(),
            address: fake::faker::address::en::StreetName().fake(),
            first_name: fake::faker::name::en::FirstName().fake(),
            last_name: fake::faker::name::en::LastName().fake(),
        }
    }

    pub fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn change_address(&mut self, new_address: &str) {
        self.address = new_address.to_string();
    }
}
