


#[derive(Debug)]
pub struct Event {
    event_id: u16,
    date_start: String,
    date_end: String,
}

impl Event {
    pub fn new(event_id: u16, date_start: &str, date_end: &str) -> Self {
        Self {
            event_id,
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
        }
    }

    pub fn to_insert_statement(&self) -> String {
        format!(
            "INSERT INTO Event (Event_ID, Date_Start, Date_End) VALUES ({}, '{}', '{}')",
            self.event_id, self.date_start, self.date_end
        )
    }
}

