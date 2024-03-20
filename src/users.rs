use fake::{Fake, Faker};
use rand;
#[derive(Debug)] pub struct User {
    user_id: u16,
    email: String,
    address: String,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(
        user_id: u16,
        email: &str,
        address: &str,
        first_name: &str,
        last_name: &str,
    ) -> Self {
        Self {
            user_id,
            email: email.to_string(),
            address: address.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO users (User_ID, Email, Address, First_Name, Last_Name) VALUES ({}, '{}', '{}', '{}', '{}')",
            self.user_id, self.email, self.address, self.first_name, self.last_name
        )
    }

    pub fn generate_fake_user() -> Self {
        Self {
            user_id: rand::random::<u16>(),
            email: fake::faker::internet::en::FreeEmail().fake(),
            address: fake::faker::address::en::StreetName().fake(),
            first_name: fake::faker::name::en::FirstName().fake(),
            last_name: fake::faker::name::en::LastName().fake(),
        }
    }

    pub fn get_user_id(&self) -> u16 {
        self.user_id
    }

    pub fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn change_address(&mut self, new_address: &str) {
        self.address = new_address.to_string();
    }
}
