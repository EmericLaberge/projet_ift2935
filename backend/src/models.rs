// User model
pub struct User {
    pub id: i32,
    pub email: String,
    pub address: String,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(email: &str, address: &str, first_name: &str, last_name: &str) -> Self {
        Self {
            id: 0, // Assuming ID will be set elsewhere, e.g., by the database
            email: email.to_string(),
            address: address.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

// Event model
pub struct Event {
    pub id: u16,
    pub date_start: String,
    pub date_end: String,
}

impl Event {
    pub fn new(id: u16, date_start: &str, date_end: &str) -> Self {
        Self {
            id,
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
        }
    }
}

// Sport model
pub struct Sport {
    pub id: u16,
    pub sport_name: String,
    pub score_format: String,
}

impl Sport {
    pub fn new(sport_name: &str, score_format: &str) -> Self {
        Self {
            id: 0, // Assuming ID will be set elsewhere
            sport_name: sport_name.to_string(),
            score_format: score_format.to_string(),
        }
    }
}

// Game model
pub struct Game {
    pub id: u16,
    pub sport_id: u16,
    pub event_id: u16,
    pub game_date: String,
    pub final_score: Option<u16>,
}

impl Game {
    pub fn new(sport_id: u16, event_id: u16, game_date: &str, final_score: Option<u16>) -> Self {
        Self {
            id: 0, // Assuming ID will be set elsewhere
            sport_id,
            event_id,
            game_date: game_date.to_string(),
            final_score,
        }
    }
}

// Team model
pub struct Team {
    pub id: u16,
    pub team_name: String,
}

impl Team {
    pub fn new(team_name: &str) -> Self {
        Self {
            id: 0, // Assuming ID will be set elsewhere
            team_name: team_name.to_string(),
        }
    }
}

// Staff model
pub struct Staff {
    pub id: u16,
    pub user_id: u16,
    pub nas: String,
    pub hiring_date: String,
}

impl Staff {
    pub fn new(user_id: u16, nas: &str, hiring_date: &str) -> Self {
        Self {
            id: 0, // Assuming ID will be set elsewhere
            user_id,
            nas: nas.to_string(),
            hiring_date: hiring_date.to_string(),
        }
    }
}

