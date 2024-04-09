use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tiberius::{ToSql};
use tiberius::time::Date;

#[derive(Serialize, Deserialize, Debug, derive_new::new)]
pub struct Event {
    id: i32,
    name: String,
    date_start: String,
    date_end: String,
}

impl Event {

    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Events (Name, StartDate, EndDate) VALUES (@P1, @P2, @P3)";
        let params = vec![
            Arc::new(self.name.clone()) as Arc<dyn ToSql>,
            Arc::new(self.date_start.clone()) as Arc<dyn ToSql>,
            Arc::new(self.date_end.clone()) as Arc<dyn ToSql>,
        ];
        (query, params)
    }
}
