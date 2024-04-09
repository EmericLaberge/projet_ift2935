use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tiberius::ToSql;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    event_id: i32,
    date_start: String,
    date_end: String,
}

impl Event {
    pub fn new(event_id: i32, date_start: &str, date_end: &str) -> Self {
        Self {
            event_id,
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
        }
    }

    pub fn to_insert_query(&self) -> (&str, Vec<Arc<dyn ToSql>>) {
        let query = "INSERT INTO Events (EventID, DateStart, DateEnd) VALUES (@P1, @P2, @P3);";
        let params = vec![
            Arc::new(self.event_id.clone()) as Arc<dyn ToSql>,
            Arc::new(self.date_start.clone()),
            Arc::new(self.date_end.clone()),
        ];
        (query, params)
    }
}
