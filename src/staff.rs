use chrono::prelude::*;
use fake::{Fake, Faker};
use rand;

#[derive(Debug)]
pub struct Staff {
    staff_id: u16,
    user_id: u16,
    nas: String,
    hiring_date: String,
}

impl Staff {
    pub fn new(staff_id: u16, user_id: u16, nas: &str, hiring_date: &str) -> Staff {
        Staff {
            staff_id,
            user_id,
            nas: nas.to_string(),
            hiring_date: hiring_date.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO Staff (Staff_ID, User_ID, NAS, Hiring_Date) VALUES ({}, {}, '{}', '{}')",
            self.staff_id, self.user_id, self.nas, self.hiring_date
        )
    }
}


