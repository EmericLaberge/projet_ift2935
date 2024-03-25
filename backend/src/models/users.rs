use actix_web::FromRequest;
use fake::Fake;
use rand::Rng;
use std::sync::Arc;
use tiberius::{Client, Config};
use tokio_util::{compat::Compat, compat::TokioAsyncWriteCompatExt};
use tiberius::ToSql;
use serde::{Serialize, Deserialize};

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct User {
    id: Option<i32>,
    email: String,
    address: String,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query =
            "INSERT INTO Users (Email, Address, FirstName, LastName) VALUES (@P1, @P2, @P3, @P4);";
        let params = vec![
            Arc::new(self.email.clone()) as Arc<dyn ToSql>,
            Arc::new(self.address.clone()),
            Arc::new(self.first_name.clone()),
            Arc::new(self.last_name.clone()),
        ];
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
        }
    }

    pub fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn change_address(&mut self, new_address: &str) {
        self.address = new_address.to_string();
    }
}
