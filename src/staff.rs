use random_string::generate;
#[derive(Debug)]
pub struct Staff {
    user_id: u16,
    nas: String,
    hiring_date: String,
}

impl Staff {
    pub fn new(user_id: u16, nas: &str, hiring_date: &str) -> Staff {
        Staff {
            user_id,
            nas: nas.to_string(),
            hiring_date: hiring_date.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO staff (User_ID, NAS, Hiring_Date) VALUES ({}, '{}', '{}')",
            self.user_id, self.nas, self.hiring_date
        )
    }

    pub fn generate_fake_staff() -> Staff {
        Staff {
            user_id: 3,
            nas: generate(9, "1234567890"),
            hiring_date: "2021-01-01".to_string(),
        }
    }
}
