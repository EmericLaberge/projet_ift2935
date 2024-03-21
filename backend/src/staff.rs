use chrono::{Duration, NaiveDate};
use rand::Rng;
use random_string::generate;
use std::sync::Arc;
use tiberius::ToSql;

#[derive(Debug)]
pub struct Staff {
    user_id: i32,
    nas: i32,
    hiring_date: String,
}

impl Staff {

pub fn new(user_id: i32, nas: i32, hiring_date: String) -> Staff {
    Staff {
        user_id,
        nas: nas,
        hiring_date: hiring_date,
    }
}

pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
    let query = "INSERT INTO Staff (UserID, NAS, HiringDate) VALUES (@P1, @P2, @P3);";
    let params = vec![
        Arc::new(self.user_id.clone()) as Arc<dyn ToSql>,
        Arc::new(self.nas.clone()),
        Arc::new(self.hiring_date.clone()) as Arc<dyn ToSql>,
    ];
    (query, params)
}

pub fn generate_fake_staff() -> Staff {
    Staff {
        user_id: 3,
        // generate a random i32 number
        nas: rand::random::<i32>(),
        // generate a random date
        hiring_date: "2024-03-21".to_string(),
    }
}
}
